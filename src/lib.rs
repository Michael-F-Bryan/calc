//! A JIT compiler written in Rust, using LLVM and [inkwell].
//!
//! To run JIT compiled code, the compiler goes through several phases:
//!
//! 1. Parse the source code into an AST (Abstract Syntax Tree)
//! 2. Translate the AST into its equivalent LLVM IR
//! 3. JIT compile the LLVM IR
//!
//! [inkwell]: https://github.com/TheDan64/inkwell

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]

extern crate failure;
extern crate inkwell;
extern crate lalrpop_util;
extern crate regex;
#[macro_use]
extern crate slog;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod syntax;
pub mod trans;
