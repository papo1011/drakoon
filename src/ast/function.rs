use super::Stmt;
use crate::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Type,
    pub body: Vec<Stmt>,
}
