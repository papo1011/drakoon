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
    FixedArrayDef {
        name: String,
        annot: Type,
        values: Vec<Expr>,
        mutable: bool,
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
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    While {
        cond: Box<Expr>,
        body: Vec<Stmt>,
    },
    For {
        init: Option<Box<Stmt>>,
        cond: Option<Box<Expr>>,
        step: Option<Box<Stmt>>,
        body: Vec<Stmt>,
    },
}
