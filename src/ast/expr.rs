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
        args: Vec<Box<Expr>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    // Arithmetic operators
    Add,
    Sub,
    Mul,
    Div,

    // Comparison operators
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,

    // Logical operators
    And,
    Or,
}
