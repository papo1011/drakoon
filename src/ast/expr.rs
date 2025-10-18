#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Int(i32),
    Double(f64),
    Var(String),
    BinaryOp {
        lhs: Box<Expr>,
        operator: Op,
        rhs: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}
