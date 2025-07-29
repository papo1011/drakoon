use super::token::Token;
use logos::{Logos, SpannedIter};

pub struct Lexer<'source> {
    token_stream: SpannedIter<'source, Token>,
}

#[derive(Debug)]
pub enum LexerError {
    InvalidToken(usize, usize), // span start/end
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            token_stream: Token::lexer(source).spanned(),
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<(usize, Token, usize), LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(result, span)| match result {
            Ok(token) => Ok((span.start, token, span.end)),
            Err(()) => Err(LexerError::InvalidToken(span.start, span.end)),
        })
    }
}
