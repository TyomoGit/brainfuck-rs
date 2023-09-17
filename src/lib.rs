use std::io::{Read, Write};

mod token;
mod interpreter;

pub fn run(string: &str, read: &mut impl Read, write: &mut impl Write) {
    let tokens = token::tokenize(string);
    let mut interpreter = interpreter::Interpreter::new(tokens);
    interpreter.run(read, write);
}