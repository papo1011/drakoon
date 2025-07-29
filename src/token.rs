use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"///[^\n]*")]
#[logos(skip r"///|[^\n]*")]
pub enum Token {
    #[token("fn")]
    Fn,

    #[token("return")]
    Return,

    #[token("println")]
    Println,

    #[token("Int")]
    IntType,

    #[token("Double")]
    DoubleType,

    #[token("let")]
    Let,

    #[token("const")]
    Const,

    #[token("while")]
    While,

    #[token("for")]
    For,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Id,

    #[regex(r"[1-9][0-9]*|0")]
    Integer,

    #[regex(r"([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([eE][+-]?[0-9]+)?")]
    Double,
}
