use super::token::Token;
use logos::{Logos, SpannedIter};

pub struct Lexer<'source> {
    token_stream: SpannedIter<'source, Token>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            token_stream: Token::lexer(source).spanned(),
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = (usize, Token, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
            .next()
            .map(|(token, span)| (span.start, token.unwrap(), span.end))
    }
}
