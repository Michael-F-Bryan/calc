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

## Super Basic Compilation

The compilation process is actually quite simple. We take in an AST and
recursively visit each node, generating the corresponding LLVM IR. To begin
with, we'll hard-code the module to be `"calc"` and compile our one and only
function. 

For this first pass we're going to take several short-cuts (noticeable by the
use of `unimplemented!()`) so we can get the initial compiler working.

```rust
    /// Compile an AST tree to a LLVM `Module`.
    pub fn compile(&self, ast: &Expr) -> Module {
        const CALC_ENTRYPOINT: &'static str = "calc_main";

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

We're not going to worry about variables just yet, so compiling atoms is just
a case of emitting a constant.

```rust
    fn compile_atom(&self, atom: &Atom) -> FloatValue {
        match *atom {
            Atom::Number(n) => self.double.const_float(n),
            _ => unimplemented!(),
        }
    }
```

Compiling a binary op is also fairly straightforward, we need to `match` on the
type of operation and then use the `Builder`'s `build_float_*()` methods to
emit the corresponding LLVM IR.

There's a little twist to this step though. In order to compile a binary 
operation we need to give LLVM its two operands. This means we'll need to 
recursively call `compile_expr()` on `op.left` and `op.right` before the `match`
bit.

```rust
    fn compile_binary_op(&self, op: &BinaryOp) -> FloatValue {
        let left = self.compile_expr(&op.left);
        let right = self.compile_expr(&op.right);

        match op.op {
            Op::Add => self.builder.build_float_add(&left, &right, "add"),
            Op::Subtract => self.builder.build_float_sub(&left, &right, "sub"),
            Op::Multiply => self.builder.build_float_mul(&left, &right, "mul"),
            Op::Divide => self.builder.build_float_div(&left, &right, "div"),
        }
    }
```

Compiling function calls requires us to do a type-checking pass beforehand, if
you skip type-checking there's a good chance someone will use the wrong number 
of parameters and leave the world in an inconsistent state (usually resulting in
a segfault).

Type-checking and symbol table generation will be done in a later chapter, so we
can leave it `unimplemented!()` for now.

```rust
    fn compile_function_call(&self, call: &FunctionCall) -> FloatValue {
        unimplemented!()
    }
```

## Testing The Code Generation

So far we can support binary operations and `double` constants. It's not overly
much, but our `calc` tool can already do everything a normal desk calculator
can. We just need to ask LLVM to JIT-compile and execute our code.

Once we've parsed the source text into an AST, the JIT-compilation process 
consists of:

- Initialize a `Target`
- Create an LLVM `Context`
- Compile the AST into a `Module`
- Create a JIT execution engine based on the module
- Get a pointer to the JIT-compiled `calc_main` function
- Run it

While this may sound long and compilicated, it's maybe a dozen lines and one
`unsafe` block at most.

```rust
pub type CalcMain = unsafe extern "C" fn() -> f64;

fn execute(src: &str) -> Result<f64, Error> {
    Target::initialize_native(&InitializationConfig::default())?;

    let ast = ::syntax::parse(src)?;
    let ctx = Context::create();
    let module = Compiler::new(&ctx).compile(&ast);

    let ee = module
        .create_jit_execution_engine(OptimizationLevel::None)?;

    unsafe {
        let calc_main = ee.get_function::<CalcMain>("calc_main")?;

        Ok(calc_main())
    }
}
```

While it's not 100% production-ready yet, we can use the above `execute()` 
function to start testing some basic inputs.

```rust
#[test]
fn execute_some_binary_ops() {
    let inputs = vec![
        ("1+1", 2.0),
        ("1-1", 0.0),
        ("2*4.5", 9.0),
        ("100.0/3", 100.0 / 3.0),
    ];

    for (src, should_be) in inputs {
        let got = execute(src).unwrap();
        assert_eq!(got, should_be);
    }
}

#[test]
fn execute_a_more_complex_statement() {
    let src = "5 * (100 + 3) / 9 - 2.5";
    let should_be = 5.0 * (100.0 + 3.0) / 9.0 - 2.5;

    let got = execute(src).unwrap();
    assert_eq!(got, should_be);
}
```


[`Module`]: http://llvm.org/doxygen/classllvm_1_1Module.html#details
[earlier]: parse/visit.html