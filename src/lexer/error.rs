#[derive(Debug)]
pub struct Message {
    text: String,
    line: usize,
}

impl Message {
    pub fn error(text: &str, line: usize) -> Message {
        Message {
            text: String::from(text),
            line,
        }
    }
}

pub fn error(msg: Message) {
    println!("[{}] Error: {}", msg.line, msg.text);
}
