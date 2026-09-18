mod lexer;
#[cfg(test)]
mod test;
mod utils;

fn main() {}

pub fn error<S: AsRef<str>>(msg: S) -> ! {
    println!("{}", msg.as_ref());
    std::process::exit(1);
}
