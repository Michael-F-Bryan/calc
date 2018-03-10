#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    FunctionCall(FunctionCall),
    Atom(Atom),
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

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOp {
    pub op: BinaryOpKind,
    pub left: Expr,
    pub right: Expr,
}

impl BinaryOp {
    pub fn new( left: Expr, right: Expr, op: BinaryOpKind) -> BinaryOp {
        BinaryOp { left, right, op }
    }

    pub fn add(left: Expr, right: Expr) -> BinaryOp {
        BinaryOp::new(left, right, BinaryOpKind::Add)
    }

    pub fn sub(left: Expr, right: Expr) -> BinaryOp {
        BinaryOp::new(left, right, BinaryOpKind::Subtract)
    }

    pub fn mult(left: Expr, right: Expr) -> BinaryOp {
        BinaryOp::new(left, right, BinaryOpKind::Multiply)
    }

    pub fn div(left: Expr, right: Expr) -> BinaryOp {
        BinaryOp::new(left, right, BinaryOpKind::Divide)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOpKind {
    Add,
    Divide,
    Multiply,
    Subtract,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Number(f64),
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

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}

impl FunctionCall {
    pub fn new<S, A>(name: S, args: A) -> FunctionCall 
    where S: Into<String>,
    A: IntoIterator<Item=Expr>,
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

    #[test]
    fn parse_a_multiply() {
        let src = "a * 5";
        let should_be = BinaryOp::mult(Atom::Ident(String::from("a")).into(), Atom::Number(5.0).into());
        let should_be = Expr::from(should_be);

        let got = grammar::parse_Expr(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_a_function_call() {
        let src = "sin(90.0)";
        let should_be = FunctionCall::new("sin", vec![Expr::Atom(Atom::Number(90.0))]);

        let got = grammar::parse_FunctionCall(src).unwrap();
        assert_eq!(got, should_be);
    }
}