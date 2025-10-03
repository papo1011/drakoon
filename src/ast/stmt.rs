use super::expr::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Var { name: String, value: Box<Expr> },
    Print { value: Box<Expr> },
}
