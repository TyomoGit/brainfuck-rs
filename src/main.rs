mod token;
mod interpreter;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file_name = env::args().nth(1).expect("no file name");
    let mut file = File::open(file_name).expect("failed to open file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("failed to read file");

    let tokens = token::tokenize(&src);
    let mut interpreter = interpreter::Interpreter::new(tokens);
    interpreter.run(&mut std::io::stdin(), &mut std::io::stdout());
}
