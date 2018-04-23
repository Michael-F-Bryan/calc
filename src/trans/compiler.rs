use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::FloatType;
use inkwell::values::{FloatValue, FunctionValue};
use slog::{Discard, Logger};
use std::fmt::{self, Debug, Formatter};

use syntax::{Atom, BinaryOp, Expr, FunctionCall, Op};

/// The signature used for `calc`'s entrypoint, `"calc_main"`.
pub type CalcMain = unsafe extern "C" fn() -> f64;
pub const CALC_ENTRYPOINT: &str = "calc_main";

pub struct Compiler<'ctx> {
    ctx: &'ctx Context,
    logger: Logger,
    builder: Builder,
    double: FloatType,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Compiler<'ctx> {
        Compiler::new_with_logger(ctx, &Logger::root(Discard, o!()))
    }

    pub fn new_with_logger(ctx: &'ctx Context, logger: &Logger) -> Compiler<'ctx> {
        let logger = logger.new(o!("phase" => "trans"));

        let double = ctx.f64_type();

        let builder = ctx.create_builder();

        Compiler {
            ctx,
            builder,
            logger,
            double,
        }
    }

    /// Compile an AST tree to a LLVM `Module`.
    pub fn compile(&self, ast: &Expr) -> Module {
        let mut module = self.ctx.create_module("calc");

        self.compile_function(&mut module, CALC_ENTRYPOINT, ast);

        module
    }

    fn compile_function(&self, module: &mut Module, name: &str, body: &Expr) -> FunctionValue {
        // hard-code all functions to be `fn() -> f64`
        let sig = self.double.fn_type(&[], false);
        let func = module.add_function(name, &sig, None);

        let entry = func.append_basic_block("entry");
        self.builder.position_at_end(&entry);

        let ret = self.compile_expr(body);

        self.builder.build_return(Some(&ret));

        func
    }

    fn compile_expr(&self, expr: &Expr) -> FloatValue {
        match *expr {
            Expr::Atom(ref atom) => self.compile_atom(atom),
            Expr::BinaryOp(ref op) => self.compile_binary_op(op),
            Expr::FunctionCall(ref call) => self.compile_function_call(call),
        }
    }

    fn compile_atom(&self, atom: &Atom) -> FloatValue {
        match *atom {
            Atom::Number(n) => self.double.const_float(n),
            _ => unimplemented!(),
        }
    }

    fn compile_binary_op(&self, op: &BinaryOp) -> FloatValue {
        let left = self.compile_expr(&op.left);
        let right = self.compile_expr(&op.right);

        match op.op {
            Op::Add => self.builder.build_float_add(&left, &right, "add"),
            Op::Subtract => self.builder.build_float_sub(&left, &right, "sub"),
            Op::Multiply => self.builder.build_float_mul(&left, &right, "mul"),
            Op::Divide => self.builder.build_float_div(&left, &right, "div"),
        }
    }

    fn compile_function_call(&self, call: &FunctionCall) -> FloatValue {
        unimplemented!()
    }
}

impl<'ctx> Debug for Compiler<'ctx> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Compiler")
            .field("ctx", self.ctx)
            .field("logger", &self.logger)
            .field("double", &self.double)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use inkwell::targets::{InitializationConfig, Target};
    use inkwell::values::InstructionOpcode;
    use inkwell::OptimizationLevel;

    #[test]
    fn compile_a_single_instruction() {
        Target::initialize_native(&InitializationConfig::default()).unwrap();

        let should_be = 3.14;
        let src = Expr::Atom(Atom::Number(should_be));

        let ctx = Context::create();
        let got = Compiler::new(&ctx).compile(&src);

        let sig = ctx.f32_type().fn_type(&[], false);
        let _func = got.add_function("dummy", &sig, None);

        let calc_main = got.get_function("calc_main").unwrap();
        assert_eq!(calc_main.count_basic_blocks(), 1);

        let entry = calc_main.get_entry_basic_block().unwrap();
        let last_inst = entry.get_last_instruction().unwrap();

        assert_eq!(last_inst.get_opcode(), InstructionOpcode::Return);

        let ee = got.create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            let func = ee.get_function::<CalcMain>("calc_main").unwrap();

            let got = func();
            assert_eq!(got, should_be);
        }
    }

    fn execute(src: &str) -> f64 {
        Target::initialize_native(&InitializationConfig::default()).unwrap();

        let ast = ::syntax::parse(src).unwrap();
        let ctx = Context::create();
        let module = Compiler::new(&ctx).compile(&ast);

        let ee = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            let func = ee.get_function::<CalcMain>("calc_main").unwrap();

            func()
        }
    }

    #[test]
    fn execute_some_binary_ops() {
        let inputs = vec![
            ("1+1", 2.0),
            ("1-1", 0.0),
            ("2*4.5", 9.0),
            ("100.0/3", 100.0 / 3.0),
        ];

        for (src, should_be) in inputs {
            let got = execute(src);
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn execute_a_more_complex_statement() {
        let src = "5 * (100 + 3) / 9 - 2.5";
        let should_be = 5.0 * (100.0 + 3.0) / 9.0 - 2.5;

        let got = execute(src);
        assert_eq!(got, should_be);
    }
}
