use std::fs::File;

use crate::lexer::Lexer;

mod lexer;
#[cfg(test)]
mod test;
mod utils;

fn main() {
    Lexer::run(File::open("src/pseudo.via").unwrap());
}
