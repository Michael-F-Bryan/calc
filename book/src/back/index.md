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

This conversion process is done by recursively walking the parsed AST, turning 
each node into the corresponding LLVM instruction(s). 

Later on, the `Visitor` trait will be used to do some minor type-checking by
looking for variables uses/assignments and function calls.

## The Compiler Struct

LLVM uses a `Context` for the various internal states and variables involved 
during the compilation process, which we'll encapsulate in a `Compiler` struct.

The `Compiler` will hold an IR `Builder` and a cached LLVM `FloatType` 
representing a `double`.

```rust
pub struct Compiler<'ctx> {
    ctx: &'ctx Context,
    logger: Logger,
    builder: Builder,
    double: FloatType,
}
```

The constructor for `Compiler` isn't overly exciting:

```rust
impl<'ctx> Compiler<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Compiler<'ctx> {
        Compiler::new_with_logger(ctx, &Logger::root(Discard, o!()))
    }

    pub fn new_with_logger(ctx: &'ctx Context, logger: &Logger) -> Compiler<'ctx> {
        let logger = logger.new(o!("phase" => "trans"));

        let double = ctx.f64_type();

        let builder = ctx.create_builder();

        Compiler {
            ctx,
            builder,
            logger,
            double,
        }
    }
}
```

The compilation process is actually quite simple. We take in an AST and 
recursively visit each node, generating the corresponding LLVM IR. We'll 
hard-code the module to be `"calc"` and compile our one and only function.

```rust
    /// Compile an AST tree to a LLVM `Module`.
    pub fn compile(&self, ast: &Expr) -> Module {
        let mut module = self.ctx.create_module("calc");

        self.compile_function(&mut module, CALC_ENTRYPOINT, ast);

        module
    }
```

In LLVM, you create a function by first declaring its signature, then add one or
more basic blocks (contiguous set of instructions without any branching or 
jumps). The entry point of every function is typically named `"entry"`.

```rust
    fn compile_function(&self, module: &mut Module, name: &str, body: &Expr) -> FunctionValue {
        // hard-code all functions to be `fn() -> f64`
        let sig = self.double.fn_type(&[], false);
        let func = module.add_function(name, &sig, None);

        let entry = func.append_basic_block("entry");
        self.builder.position_at_end(&entry);

        let ret = self.compile_expr(body);

        self.builder.build_return(Some(&ret));

        func
    }
```

Compiling an `Expr` is just a case of `match`ing on the type of expression and
calling the corresponding `compile_*()` method.

```rust
    fn compile_expr(&self, expr: &Expr) -> FloatValue {
        match *expr {
            Expr::Atom(ref atom) => self.compile_atom(atom),
            Expr::BinaryOp(ref op) => self.compile_binary_op(op),
            Expr::FunctionCall(ref call) => self.compile_function_call(call),
        }
    }
```


[`Module`]: http://llvm.org/doxygen/classllvm_1_1Module.html#details
[earlier]: parse/visit.html