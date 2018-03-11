//! Syntax tree traversal.
//!
//! Each method of the `Visitor` trait is a hook that can be overridden to
//! customize the behavior when visiting the corresponding type of node. By
//! default, every method recursively visits the substructure of the input by
//! invoking the right visitor method of each of its fields.
//!
//! Use the `walk_*()` functions to continue traversing the AST in the default
//! traversal order.

use syntax::ast::{Atom, BinaryOp, Expr, FunctionCall};

/// A utility trait for traversing an AST.
pub trait Visitor {
    /// Visit an `Expr` node.
    fn visit_expr(&mut self, e: &Expr) {
        walk_expr(self, e);
    }

    /// Visit a binary operation.
    fn visit_binary_op(&mut self, b: &BinaryOp) {
        walk_binary_op(self, b);
    }

    /// Visit a function call.
    fn visit_function_call(&mut self, f: &FunctionCall) {
        walk_function_call(self, f);
    }

    /// Visit an `Atom`.
    fn visit_atom(&mut self, _atom: &Atom) {}
}

/// Continue to recursively walk an expression, calling the visitor's
/// `visit_atom()`, `visit_function_call()`, or `visit_binary_op()` method
/// depending on what type of `Expr` it is.
pub fn walk_expr<V: Visitor + ?Sized>(visitor: &mut V, e: &Expr) {
    match *e {
        Expr::Atom(ref a) => visitor.visit_atom(a),
        Expr::FunctionCall(ref f) => visitor.visit_function_call(f),
        Expr::BinaryOp(ref b) => visitor.visit_binary_op(b),
    }
}

/// Recursively visit a binary operation's left and right operands.
pub fn walk_binary_op<V: Visitor + ?Sized>(visitor: &mut V, b: &BinaryOp) {
    visitor.visit_expr(&b.left);
    visitor.visit_expr(&b.right);
}

/// Recursively visit each argument in the function call.
pub fn walk_function_call<V: Visitor + ?Sized>(visitor: &mut V, f: &FunctionCall) {
    for arg in &f.arguments {
        visitor.visit_expr(arg);
    }
}
