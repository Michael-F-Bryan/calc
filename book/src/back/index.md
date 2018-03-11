# Converting to LLVM IR

Now we've got a more computer-friendly representation of our program we need
to convert it to LLVM's Intermediate Representation. This IR can come in several
forms, a human-readable "assembly", a compiled bitcode, or an in-memory tree of
objects (similar to our current AST).

LLVM uses the [`Module`] as its base compilation unit (think of it as a single
`*.c` file), with a `Module` containing several functions, datatypes, constants,
or global variables.

Because our simple calculator doesn't allow you to declare functions, we're 
going to throw everything into one big main function with the signature 
`fn calc_main() -> f64`. This way when we JIT compile the program we can call 
into the `calc_main()` function to execute everything. It also means it's quite
trivial to compile the program into a shared library (`*.so` or DLL) so other
programs can call it.

This conversion process is done by using the `Visitor` trait defined [earlier]
to recursively walk the AST, generating LLVM instructions for each node.

[`Module`]: http://llvm.org/doxygen/classllvm_1_1Module.html#details
[earlier]: parse/visit.html