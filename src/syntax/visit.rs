use syntax::ast::{Expr, FunctionCall, BinaryOp, Atom};

/// A utility trait for traversing an AST.
pub trait Visitor {
    fn visit_expr(&mut self, e: &Expr) {
        walk_expr(self, e);
    }

    fn visit_binary_op(&mut self, b: &BinaryOp) {
        walk_binary_op(self, b);
    }

    fn visit_function_call(&mut self, f: &FunctionCall) {
        walk_function_call(self, f);
    }

    fn visit_atom(&mut self, atom: &Atom) {}
}

pub fn walk_expr<V: Visitor + ?Sized>(visitor: &mut V, e: &Expr) {
    match *e {
        Expr::Atom(ref a) => visitor.visit_atom(a),
        _ => unimplemented!()
    }
}

pub fn walk_binary_op<V: Visitor + ?Sized>(visitor: &mut V, b: &BinaryOp) {
    visitor.visit_expr(&b.left);
    visitor.visit_expr(&b.right);
}

pub fn walk_function_call<V: Visitor + ?Sized>(visitor: &mut V, f: &FunctionCall) {
    for arg in &f.arguments {
        visitor.visit_expr(arg);
    }
}
