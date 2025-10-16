use super::expr::Expr;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Var {
        name: String,
        annot: Option<Type>,
        value: Box<Expr>,
        mutable: bool,
    },
    Print {
        value: Box<Expr>,
    },
}
