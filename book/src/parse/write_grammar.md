# Writing `grammar.lalrpop`

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

