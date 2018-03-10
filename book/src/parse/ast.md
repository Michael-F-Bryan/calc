# The AST

Now we have an idea of the language's syntax and the various elements in it,
we can define an Abstract Syntax Tree for it.

At the very bottom of the tree is the `Atom`. This is either a number literal or
an identifier.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Number(f64),
    Ident(String),
}
```

To make constructing an `Atom` easier, you probably want to implement `From<T>`
for `f64`, `String`, and `&'a str`.

Next up is the `BinaryOp`. This is just a container which holds its left and 
right arguments, plus the operation that was used.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOp {
    pub op: Op,
    pub left: Expr,
    pub right: Expr,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Divide,
    Multiply,
    Subtract,
}
```

If you were paying attention, you will have seen that the type of a `BinaryOp`'s 
`left` operand is `Expr`. This will be our language's top-level construct and is
implemented as a simple enum.


```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    FunctionCall(FunctionCall),
    Atom(Atom),
    BinaryOp(Box<BinaryOp>),
}
```

The last thing we need to define is a `FunctionCall`. This is just a thing that
has a name and a bunch of arguments.

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}
```

It is recommended to sprinkle the `ast` module with implementations of `From` or
similar constructors/helper functions to make working with an AST and creation
easier.

> **Note:** Assignment nodes have been left as an exercise for the reader. 
> They're not overly difficult to add to the language, in fact, there's a way to
> add them without needing to define any new types. 

