use std::vec;

use crate::{
    error,
    lexer::token::{Token, Tokenized},
};

pub struct Scanner {
    pub source: String,
    buffer: String,
    position: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            buffer: String::new(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut result = vec![];
        let source = std::mem::take(&mut self.source);
        let mut chars = source.chars();

        while let Some(c) = chars.next() {
            match c {
                ';' => {
                    handle_result(self.read_buffer(), &mut result);
                    result.push(Token::semicolon(self.position));
                }
                ' ' | '\x09'..='\x0d' => handle_result(self.read_buffer(), &mut result),
                _ => {
                    self.buffer.push(c);
                }
            };

            self.position += 1;
        }

        result.push(Token::without(
            super::token::TokenKind::EOF,
            "EOF",
            self.position..self.position,
        ));

        result
    }

    fn read_buffer(&mut self) -> Result<Token, BufferError> {
        if self.buffer.is_empty() {
            return Err(BufferError::BufferEmpty);
        }

        self.buffer.token(self.position).ok_or_else(|| {
            BufferError::TokenUndefined(format!("Token is undefined:\n\t>>>{}<<<", self.buffer))
        })
    }
}

#[derive(Debug, Clone)]
pub enum BufferError {
    BufferEmpty,
    TokenUndefined(String),
}

fn handle_result(result: Result<Token, BufferError>, vec: &mut Vec<Token>) {
    match result {
        Ok(token) => vec.push(token),
        Err(err) => match err {
            BufferError::BufferEmpty => {}
            BufferError::TokenUndefined(msg) => error(msg),
        },
    }
}
