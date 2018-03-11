//! The language's parser and AST representation.
//! 
//! The main entry point to the parser is via the [`parse()`] function. This 
//! takes source text and tries to convert it into its AST representation. If
//! you then want to inspect the parsed program you can use the [`Visitor`]
//! trait for AST traversal.
//! 
//! [`parse()`]: fn.parse.html
//! [`Visitor`]: visit/trait.Visitor.html

mod ast;
mod grammar;
pub mod visit;

pub use self::ast::*;

use failure::Error;

/// Parse a string into its AST representation.
pub fn parse(src: &str) -> Result<Expr, Error> {
    grammar::parse_Expr(src)
        .map_err(|e| e.map_token(|tok| tok.to_string()))
        .map_err(Error::from)
}