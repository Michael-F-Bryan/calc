# Parsing

The first step in creating our calculator is turning a stream of text provided
by the user into something more computer-friendly. This structure is usually
referred to as an [Abstract Syntax Tree] and is essentially just a tree where 
each leaf node is an "atom" (the smallest possible construct in a language, 
usually constants or identifiers). All non-leaf nodes then correspond to the 
compound constructs such as binary operators or function calls.

To make things easier we'll be using [lalrpop] to generate our parsing code and
construct the AST. If you've never heard of `lalrpop` I *highly recommend* you
check out [their guide].

## Setting Up Lalrpop

To use `lalrpop` we'll need to add it to our dependencies and set up the build
script. While we're at it, lets also make sure we've added `inkwell` and 
`failure` as dependencies (for LLVM bindings and error handling respectively).

First lets create a new cargo project. We'll structure it as a main `calc` crate 
with a small binary that just parses the command line arguments and sets 
everything up before calling into the central crate to run everything.

```
$ cargo new calc
```

Then update `Cargo.toml`:

```toml
# Cargo.toml

[package]
name = "calc"
version = "0.1.0"
authors = ["Michael Bryan <michaelfbryan@gmail.com>"]
build = "build.rs"

[dependencies]
inkwell = { git = "https://github.com/TheDan64/inkwell", features = ["llvm3-7"] }
failure = "0.1.1"
lalrpop-util = "0.14.0"
regex = "0.2.7"

[build-dependencies]
lalrpop = "0.14.0"
```

And the build script:

```rust
// build.rs

extern crate lalrpop;

fn main() {
    lalrpop::process_root().unwrap();
}
```

With the lalrpop build system set up we can lay out the crate's skeleton.
It's usually a good idea to break each phase of a compiler (because that's what
we're effectively making) out into their own modules, so here's the tentative 
directory structure:

```text
- /
  - bin/
    - yalc.rs
  - src/
    - lib.rs
    - syntax/
      - mod.rs
      - ast.rs
      - grammar.lalrpop
```

At the moment, we've stubbed out the rust files with a bunch of `extern crate` 
and `mod` statements.

## The Language Grammar

Now we've got a lot of the boilerplate set up, we can start trying to figure out
what our language's grammar should look like. 

The easiest way to do this is by writing out a bunch of example use cases.

```text
# This is a comment
5 * (3+4)  # You can do the usual arithmetic stuff
x = 3*PI/4  # and read/write variables
y = sin(x)^2 # plus call functions
```

While this language won't be turing complete (we don't have conditionals or 
loops), it should be a fairly decent calculator.

Once you have several examples the next step is to formalize the language 
grammar to make it easier to parse. This is usually done by writing a bunch of
"rules" in [Backus-Naur Form][bnf].

```ebnf
expr := <term>
      | "(" <expr> ")"
      | <function-call>
term := <factor>
      | <term> "+" <term>
      | <term> "-" <term>
factor := NUMBER
        | IDENTIFIER
        | <factor> "*" <factor>
        | <factor> "/" <factor>
function-call := IDENTIFIER "(" <arg-list> ")"
arg-list := EPSILON
          | <expr> ("," <expr>)*
```

To put it in human terms, we would read the first rule as saying "an *expr*
is either a *term*, an *expr* surrounded by parentheses, or a *function
call*".

## The AST

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

## Writing `grammar.lalrpop`

Now we've got some types to work with we can write a grammar which `lalrpop` 
will use when generating the parser.

The top of the `grammar.lalrpop` will be inserted into the generated file as-is,
making it the perfect place to insert the import statements we'll need.

At the moment we only need to put a single line here.

```rust
use syntax::ast::{Expr, Atom, BinaryOp, FunctionCall};
```

Next we tell `lalrpop` that the grammar section has started

```rust
grammar;
```

Our grammar has roughly two levels of precedence, so that means we create two 
rules for working with arithmetic expressions.

```rust
pub Expr: Expr = {
    <l:Expr> "+" <r:Factor> => BinaryOp::add(l, r).into(),
    <l:Expr> "-" <r:Factor> => BinaryOp::sub(l, r).into(),
    Factor,
};

Factor: Expr = {
    <l:Factor> "*" <r:Atom> => BinaryOp::mult(l, r.into()).into(),
    <l:Factor> "/" <r:Atom> => BinaryOp::div(l, r.into()).into(),
    "(" <e:Expr> ")" => e,
    <f:FunctionCall> => f.into(),
    <a:Atom> => a.into(),
};
```

There's also the rule for function calls:

```rust
pub FunctionCall: FunctionCall = {
    <i:ident> "(" <a:CommaSeparated<Expr>> ")" => FunctionCall::new(i, a),
};

CommaSeparated<T>: Vec<T> = { 
    <v:(<T> ",")*> <e:T?> => match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
};
```

And finally, we define the rules for parsing an `Atom`.

```rust
pub Atom: Atom = {
    num => Atom::Number(<>),
  ident => Atom::Ident(<>),
};

num: f64 = {
  <s:r"[0-9]+(\.[0-9]+)?"> => s.parse().unwrap(),
};

ident: String = {
  <i:r"[a-zA-Z][a-zA-Z0-9_-]*"> => i.to_string(),
};
```

As a sanity check, we should add some tests to make sure the language's
grammar parses correctly.

First up, we'll test for parsing atoms.

```rust
#[test]
fn parse_a_number_atom() {
    let src = "3.14";
    let should_be = Atom::Number(3.14);

    let got = grammar::parse_Atom(src).unwrap();
    assert_eq!(got, should_be);
}

#[test]
fn parse_an_identifier() {
    let src = "x";
    let should_be = Atom::Ident(String::from(src));

    let got = grammar::parse_Atom(src).unwrap();
    assert_eq!(got, should_be);
}
```

Then a binary op,

```rust
#[test]
fn parse_a_multiply() {
    let src = "a * 5";
    let should_be = BinaryOp::mult(Atom::Ident(String::from("a")).into(), Atom::Number(5.0).into());
    let should_be = Expr::from(should_be);

    let got = grammar::parse_Expr(src).unwrap();
    assert_eq!(got, should_be);
}
```

And we should also add a test for function calls.

```rust
#[test]
fn parse_a_function_call() {
    let src = "sin(90.0)";
    let should_be = FunctionCall::new("sin", vec![Expr::Atom(Atom::Number(90.0))]);

    let got = grammar::parse_FunctionCall(src).unwrap();
    assert_eq!(got, should_be);
}
```

So far our tests have checked individual grammar rules in isolation. To ensure
operator precedence is encoded correctly in the language's grammar we'll need to
create a non-trivial example and make sure it gives us *exactly* the parse tree
we'd expect.

```rust
    #[test]
    fn complex_parse_tree() {
        let src = "5 + (3-2) * x - sin(90.0)";
        let should_be = BinaryOp::sub(
            BinaryOp::add(Atom::from(5).into(),
                BinaryOp::mult(
                    BinaryOp::sub(Atom::from(3).into(), Atom::from(2).into()).into(),
                    Atom::from("x").into(),
                ).into()).into(),
                FunctionCall::new("sin", vec![Atom::Number(90.0).into()]).into()
        );
        let should_be = Expr::from(should_be);

        let got = grammar::parse_Expr(src).unwrap();

        assert_eq!(got, should_be);
    }
```


[Abstract Syntax Tree]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[lalrpop]: https://github.com/lalrpop/lalrpop
[their guide]: http://lalrpop.github.io/lalrpop/README.html
[bnf]: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form