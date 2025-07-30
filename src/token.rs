use logos::{Lexer, Logos, Skip};

/// Callback to update line count and char index
fn newline_callback(lex: &mut Lexer<Token>) -> Skip {
    lex.extras.0 += 1;
    lex.extras.1 = lex.span().end;
    Skip
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\f]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"///[^\n]*")]
#[logos(skip r"///\|[^\n]*")]
#[logos(extras = (usize, usize))] // (line, line_start_char)
pub enum Token {
    #[regex(r"\n", newline_callback)]
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

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Id,
    #[regex(r"[1-9][0-9]*|0")]
    Integer,
    #[regex(r"([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([eE][+-]?[0-9]+)?")]
    Double,
}
