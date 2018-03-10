# Setting Up Lalrpop

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

- /
  - bin/
    - yalc.rs
  - src/
    - lib.rs
    - syntax/
      - mod.rs
      - ast.rs
      - grammar.lalrpop

At the moment, we've stubbed out the rust files with a bunch of `extern crate` 
and `mod` statements.
