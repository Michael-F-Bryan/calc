use std::fmt::{self, Debug, Formatter};
use slog::{Discard, Logger};
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::values::FloatValue;

use syntax::visit::{self, Visitor};

pub struct Compiler<'ctx> {
    ctx: &'ctx Context,
    logger: Logger,
    builder: Builder,
    stack: Vec<FloatValue>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Compiler<'ctx> {
        Compiler::new_with_logger(ctx, &Logger::root(Discard, o!()))
    }

    pub fn new_with_logger(ctx: &'ctx Context, logger: &Logger) -> Compiler<'ctx> {
        Compiler {
            ctx: ctx,
            builder: ctx.create_builder(),
            logger: logger.new(o!("phase" => "trans")),
            stack: Vec::new(),
        }
    }
}

impl<'ctx> Debug for Compiler<'ctx> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Compiler")
            .field("ctx", self.ctx)
            .field("logger", &self.logger)
            .field("stack", &self.stack)
            .finish()
    }
}

impl<'ctx> Visitor for Compiler<'ctx> {}
