//! Generate LLVM IR for a valid `calc` expression.

#![allow(missing_docs)]

mod compiler;

pub use self::compiler::Compiler;

use syntax::Expr;
use inkwell::context::Context;
use inkwell::module::Module;
use failure::Error;
use slog::Logger;

use syntax::visit::Visitor;

pub fn translate(ast: &Expr, ctx: &Context, logger: &Logger) -> Result<Module, Error> {
    info!(logger, "Starting the compilation phase");

    let c = Compiler::new_with_logger(ctx, logger);
    c.compile(ast);

    unimplemented!()
}
