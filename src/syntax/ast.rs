/// A `calc` expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A `FunctionCall` node.
    FunctionCall(FunctionCall),
    /// An `Atom` node.
    Atom(Atom),
    /// A `BinaryOp` node.
    BinaryOp(Box<BinaryOp>),
}

impl From<Atom> for Expr {
    fn from(other: Atom) -> Expr {
        Expr::Atom(other)
    }
}

impl From<BinaryOp> for Expr {
    fn from(other: BinaryOp) -> Expr {
        Expr::BinaryOp(Box::new(other))
    }
}

impl From<FunctionCall> for Expr {
    fn from(other: FunctionCall) -> Expr {
        Expr::FunctionCall(other)
    }
}

/// A binary operation.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOp {
    /// What kind of operation is this?
    pub op: Op,
    /// The left operand.
    pub left: Expr,
    /// The right operand.
    pub right: Expr,
}

impl BinaryOp {
    /// Create a new `BinaryOp`.
    pub fn new(left: Expr, right: Expr, op: Op) -> BinaryOp {
        BinaryOp { left, right, op }
    }

    /// Create an addition operation.
    pub fn add(left: Expr, right: Expr) -> BinaryOp {
        BinaryOp::new(left, right, Op::Add)
    }

    /// Create a subtract operation.
    pub fn sub(left: Expr, right: Expr) -> BinaryOp {
        BinaryOp::new(left, right, Op::Subtract)
    }

    /// Create a multiplication operation.
    pub fn mult(left: Expr, right: Expr) -> BinaryOp {
        BinaryOp::new(left, right, Op::Multiply)
    }

    /// Create a division operation.
    pub fn div(left: Expr, right: Expr) -> BinaryOp {
        BinaryOp::new(left, right, Op::Divide)
    }
}

/// The kind of operation in a `BinaryOp`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    /// Addition.
    Add,
    /// Division.
    Divide,
    /// Multiplication.
    Multiply,
    /// Subtraction.
    Subtract,
}

/// The most basic construct in the language.
#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    /// A floating point literal.
    Number(f64),
    /// An identifier (e.g. `foo`).
    Ident(String),
}

impl From<String> for Atom {
    fn from(other: String) -> Atom {
        Atom::Ident(other)
    }
}

impl<'a> From<&'a str> for Atom {
    fn from(other: &'a str) -> Atom {
        Atom::Ident(other.to_string())
    }
}

impl From<f64> for Atom {
    fn from(other: f64) -> Atom {
        Atom::Number(other)
    }
}

impl From<i32> for Atom {
    fn from(other: i32) -> Atom {
        Atom::Number(other as f64)
    }
}

/// A function call.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    /// The function being called.
    pub name: String,
    /// The list of arguments passed to the function call.
    pub arguments: Vec<Expr>,
}

impl FunctionCall {
    /// Create a new `FunctionCall`.
    pub fn new<S, A>(name: S, args: A) -> FunctionCall
    where
        S: Into<String>,
        A: IntoIterator<Item = Expr>,
    {
        FunctionCall {
            name: name.into(),
            arguments: args.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syntax::grammar;

    #[test]
    fn parse_a_number_atom() {
        let src = "3.14";
        let should_be = Atom::Number(3.14);

        let got = grammar::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_an_identifier() {
        let src = "x";
        let should_be = Atom::Ident(String::from(src));

        let got = grammar::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_a_multiply() {
        let src = "a * 5";
        let should_be = BinaryOp::mult(
            Atom::Ident(String::from("a")).into(),
            Atom::Number(5.0).into(),
        );
        let should_be = Expr::from(should_be);

        let got = grammar::ExprParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_a_function_call() {
        let src = "sin(90.0)";
        let should_be = FunctionCall::new("sin", vec![Expr::Atom(Atom::Number(90.0))]);

        let got = grammar::FunctionCallParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn complex_parse_tree() {
        let src = "5 + (3-2) * x - sin(90.0)";
        let should_be = BinaryOp::sub(
            BinaryOp::add(
                Atom::from(5).into(),
                BinaryOp::mult(
                    BinaryOp::sub(Atom::from(3).into(), Atom::from(2).into()).into(),
                    Atom::from("x").into(),
                ).into(),
            ).into(),
            FunctionCall::new("sin", vec![Atom::Number(90.0).into()]).into(),
        );
        let should_be = Expr::from(should_be);

        let got = grammar::ExprParser::new().parse(src).unwrap();

        assert_eq!(got, should_be);
    }
}
