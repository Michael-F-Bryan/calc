# Creating an AST Visitor

Now that we can parse source code into an AST we need a mechanism for traversing
the tree. The way this is commonly done in Rust is by defining a `Visitor` trait
which, by default, will recursively walk the tree.

A good example of this is [`syn::visit::Visit`].

## The Visitor Trait

Our AST is nowhere near as complex as the Rust AST, so we shouldn't require 
as many methods as `syn`'s `Visit` trait.

```rust
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
```

We also define several `walk_*()` functions that will allow the `Visitor` to 
recursively visit each node in the tree using the default traversal order. By
making them `pub` we allow users to use them to continue walking the tree when
they've done something at a particular node.

```rust
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
```

Don't forget to update `mod.rs` so this `visit` module is included in the crate.

```rust
// src/syntax/mod.rs

mod ast;
mod grammar;
pub mod visit;

pub use self::ast::*;
```


[`syn::visit::Visit`]: https://docs.rs/syn/0.12.14/syn/visit/trait.Visit.html