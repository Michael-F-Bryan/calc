//! A JIT compiler written in Rust, using LLVM and [inkwell].
//!
//! [inkwell]: https://github.com/TheDan64/inkwell


#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations)]

extern crate failure;
extern crate inkwell;
extern crate lalrpop_util;
extern crate regex;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub mod syntax;
