use logos::Logos;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    InvalidFloat(ParseFloatError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

impl From<ParseFloatError> for LexicalError {
    fn from(err: ParseFloatError) -> Self {
        LexicalError::InvalidFloat(err)
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"//[^\n]*",skip r"///[^\n]*", skip r"///\|[^\n]*", error = LexicalError)]
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
    #[token("Unit")]
    UnitType,

    #[token("let")]
    Let,
    #[token("const")]
    Const,

    #[token("mut")]
    Mut,

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

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*", |lex| lex.slice().to_string())]
    Id(String),
    #[regex(r"[1-9][0-9]*|0", |lex| lex.slice().parse())]
    Int(i32),
    #[regex(r"[0-9]+\.[0-9]*([eE][+-]?[0-9]+)?", |lex| lex.slice().parse())]
    Double(f64),
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice()[1..lex.slice().len() - 1].to_string())]
    #[regex(r#"'([^'\\]|\\.)*'"#, |lex| lex.slice()[1..lex.slice().len() - 1].to_string())]
    String(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
