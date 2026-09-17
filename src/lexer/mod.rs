use std::{fs::File, io::read_to_string};

use crate::lexer::{error::error, scanner::Scanner};

pub mod error;
pub mod scanner;
pub mod token;

#[cfg(test)]
pub mod test;

pub struct Lexer {}

impl Lexer {
    pub fn run(script: File) -> Self {
        // let source = read_to_string(script).unwrap();
        // let mut scanner = Scanner::new(source);
        // if let Err((msg, line)) = scanner.tokenize() {
        //     error(line, msg);
        // }
        // dbg!(scanner.tokens);

        Self {}
    }

    pub fn runtime() {
        let input = std::io::stdin();

        loop {
            let mut line = String::new();
            let result = input.read_line(&mut line);

            if line.is_empty() {
                continue;
            };

            //run(line)
        }
    }
}
