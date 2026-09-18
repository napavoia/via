pub mod scanner;
#[cfg(test)]
pub mod test;
pub mod token;

pub use scanner::Scanner as Lexer;
