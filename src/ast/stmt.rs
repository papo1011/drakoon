use super::expr::Expr;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    VarDef {
        name: String,
        annot: Option<Type>,
        value: Box<Expr>,
        mutable: bool,
    },
    VarAssign {
        name: String,
        value: Box<Expr>,
    },
    PrintExpr {
        value: Box<Expr>,
    },
    PrintString {
        value: String,
    },
}
