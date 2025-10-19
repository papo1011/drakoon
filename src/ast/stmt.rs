use super::expr::Expr;
use crate::types::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    MainDef {
        body: Vec<Stmt>,
    },
    GlobalVarDef {
        name: String,
        annot: Option<Type>,
        value: Box<Expr>,
        is_const: bool,
    },
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
    FnDef {
        name: String,
        params: Vec<(String, Type)>,
        ret_type: Type,
        body: Vec<Stmt>,
    },
    FnCall {
        name: String,
        args: Vec<Expr>,
    },
    Return {
        value: Option<Box<Expr>>,
    },
    If {
        cond: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
}
