use std::vec;

use crate::lexer::{
    error::Message,
    token::{Token, Tokenized},
};

pub struct Scanner {
    pub source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>, Message> {
        let mut result = vec![];
        let mut chars = self.source.chars();

        let mut buffer: String = String::new();
        let mut line = 0;
        let mut ready = (false, false);
        while let Some(c) = chars.next() {
            if c.is_whitespace() {
                ready.0 = true;
            }

            if c == ';' {
                ready = (true, true);
            }

            if ready.0 {
                if !buffer.is_empty() {
                    let token = buffer.token(line);

                    result.push(token.unwrap());
                }

                if ready.1 {
                    result.push(Token::semicolon(line));
                    ready.1 = false;
                } else if c == '\n' {
                    line += 1;
                }

                ready.0 = false;
            } else {
                buffer.push(c);
            }
        }

        Ok(result)
    }
}
