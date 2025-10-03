use super::expr::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Variable {
        name: String,
        value: Box<Expression>,
    },
    Print {
        value: Box<Expression>,
    },
}
