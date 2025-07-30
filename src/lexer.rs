use super::token::Token;
use logos::{Logos, SpannedIter};

pub struct Lexer<'source> {
    token_stream: SpannedIter<'source, Token>,
    source: &'source str,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum LexerError {
    InvalidToken {
        start: Position,
        invalid_token: String,
        end: Position,
    },
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            token_stream: Token::lexer(source).spanned(),
            source,
        }
    }

    fn char_to_position(&self, char_index: usize) -> Position {
        let mut line = 1;
        let mut column = 1;

        for (i, ch) in self.source.char_indices() {
            if i >= char_index {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }

        Position { line, column }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<(Position, Token, Position), LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(result, span)| {
            let start = self.char_to_position(span.start);
            let end = self.char_to_position(span.end);

            match result {
                Ok(token) => Ok((start, token, end)),
                Err(()) => {
                    let invalid_token = self.source[span.start..span.end].to_string();
                    Err(LexerError::InvalidToken {
                        start,
                        invalid_token,
                        end,
                    })
                }
            }
        })
    }
}
