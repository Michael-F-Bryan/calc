use std::fmt::{self, Debug, Formatter};
use std::mem;
use slog::{Discard, Logger};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::FloatValue;
use inkwell::types::FloatType;
use inkwell::execution_engine::ExecutionEngine;
use failure::Error;

use syntax::visit::{self, Visitor};
use syntax::{Atom, BinaryOp, Expr};

pub struct Compiler<'ctx> {
    ctx: &'ctx Context,
    logger: Logger,
    builder: Builder,
    module: Module,
    double: FloatType,
    stack: Vec<FloatValue>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Compiler<'ctx> {
        Compiler::new_with_logger(ctx, &Logger::root(Discard, o!()))
    }

    pub fn new_with_logger(ctx: &'ctx Context, logger: &Logger) -> Compiler<'ctx> {
        let logger = logger.new(o!("phase" => "trans"));

        let double = ctx.f64_type();
        let stack = Vec::new();

        let builder = ctx.create_builder();
        let module = ctx.create_module("calc");

        // the calc_main function has a hard-coded signature
        let sig = double.fn_type(&[], false);
        let calc_main = module.add_function("calc_main", &sig, None);

        // position the builder at the start of our `entry` block
        let entry = calc_main.append_basic_block("entry");
        builder.position_at_end(&entry);

        Compiler {
            ctx,
            builder,
            module,
            logger,
            double,
            stack,
        }
    }

    pub fn compile(mut self, ast: &Expr) -> Module {
        self.visit_expr(ast);

        // the stack should have just one value on it. This is what we'll
        // return from calc_main
        assert_eq!(
            self.stack.len(),
            1,
            "The return stack should have just one element. This is a bug."
        );

        let ret = self.stack.pop().unwrap();
        self.builder.build_return(Some(&ret));

        self.module.clone()
    }
}

impl<'ctx> Debug for Compiler<'ctx> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Compiler")
            .field("ctx", self.ctx)
            .field("logger", &self.logger)
            .field("module", &self.module)
            .field("double", &self.double)
            .field("stack", &self.stack)
            .finish()
    }
}

impl<'ctx> Visitor for Compiler<'ctx> {
    fn visit_atom(&mut self, a: &Atom) {
        let inst = match *a {
            Atom::Number(n) => self.double.const_float(n),
            Atom::Ident(ref id) => unimplemented!("You can't use variables just yet!"),
        };

        self.stack.push(inst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::values::InstructionOpcode;
    use inkwell::OptimizationLevel;

    #[test]
    fn compile_a_single_instruction() {
        let should_be = 3.14;
        let src = Expr::Atom(Atom::Number(should_be));

        let ctx = Context::create();
        let got = Compiler::new(&ctx).compile(&src);

        got.print_to_stderr();

        let calc_main = got.get_function("calc_main").unwrap();
        assert_eq!(calc_main.count_basic_blocks(), 1);

        let entry = calc_main.get_entry_basic_block().unwrap();
        let last_inst = entry.get_last_instruction().unwrap();

        assert_eq!(last_inst.get_opcode(), InstructionOpcode::Return);

        // let ee = got.create_jit_execution_engine(OptimizationLevel::None)
        //     .unwrap();

        unsafe {
            // let func_addr = ee.get_function_address("calc_main").unwrap();
            // let func: *const fn() -> f64 = mem::transmute(func_addr as usize);

            // let got = (*func)();
            // assert_eq!(got, should_be);
        }
    }
}
