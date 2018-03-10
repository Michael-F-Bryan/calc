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


[Abstract Syntax Tree]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[lalrpop]: https://github.com/lalrpop/lalrpop
[their guide]: http://lalrpop.github.io/lalrpop/README.html
[bnf]: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form