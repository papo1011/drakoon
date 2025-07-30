use logos::Logos;
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+", skip r"//[^\n]*",skip r"///[^\n]*", skip r"///\|[^\n]*", error = LexicalError)]
pub enum Token {
    #[regex(r"\n")]
    Newline,

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
    #[token("Unit")]
    UnitType,

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

    #[token(":")]
    Col,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lt,
    #[token("<=")]
    Le,
    #[token(">")]
    Gt,
    #[token(">=")]
    Ge,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("%")]
    Mod,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Not,
    #[token("->")]
    Arrow,
    #[token(".")]
    Dot,
    #[token("::")]
    DoubleColon,
    #[token("=>")]
    FatArrow,
    #[token("@")]
    At,

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Id,
    #[regex(r"[1-9][0-9]*|0")]
    Integer,
    #[regex(r"([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([eE][+-]?[0-9]+)?")]
    Double,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
