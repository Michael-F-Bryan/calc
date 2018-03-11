# Introduction

This is an introduction to using LLVM and the [inkwell] crate to write a JIT
compiled calculator in Rust.

## Roadmap

By the end of this endeavour we want to have a command-line calculator which can

- Do all the basic arithmetic operations (`5 * (7+8)`)
- Have access to a bunch of pre-defined constants (`2 * PI / 3`)
- Call mathematical functions from the C math library (`sin(2*PI/3)` calls the 
  `sin()` function from `libm`)
- Create our own variables (`angle = 3 * PI / 4`)

If there is time we might even try to define our own functions. It'd also be 
pretty cool to compile the code as a shared library (`*.so` or DLL) so it can be
linked into other programs.

To do this, our calculator will need to run several phases

- Parse the input into its AST ([Abstract Syntax Tree]) representation
- Use [inkwell] to turn this AST into a LLVM [Module] \(a single unit of 
  compilation in LLVM) and define a top level `calc_main()` function
- JIT compile this `Module`
- Call the `calc_main` (possibly passing in arguments) and print out the result

For simplicity of implementation, the only data type our language will know
about is the `double` (a 64-bit floating point number).

[inkwell]: https://github.com/TheDan64/inkwell
[Abstract Syntax Tree]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[Module]: http://llvm.org/doxygen/classllvm_1_1Module.html#details