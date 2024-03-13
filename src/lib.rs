use std::io::{Read, Write};

use parser::Parser;
use scanner::Scanner;

pub mod converter;
pub mod interpreter;
pub mod op;
pub mod parser;
pub mod scanner;
pub mod token;

pub fn run(string: &str, read: impl Read + 'static, write: impl Write + 'static) {
    let mut scanner = Scanner::new(string.chars().collect());
    let tokens = scanner.scan_tokens();

    let parser = Parser::new(tokens);
    let parse_result = parser.parse_tokens();
    let Ok(code) = parse_result else {
        for error in parse_result.unwrap_err() {
            eprintln!("{}", error);
        }

        return;
    };

    for (i, op) in code.iter().enumerate() {
        println!("{}: {:?}", i, op);
    }

    let mut interpreter = interpreter::Interpreter::new(code, read, write);
    interpreter.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MyReader {
        pub input: Vec<u8>,
    }

    #[derive(Clone)]
    struct MyWriter {
        pub output: Vec<u8>,
    }

    impl Read for MyReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.input.as_slice().read(buf)
        }
    }

    impl MyReader {
        pub fn empty() -> Self {
            Self {
                input: [0; 255].to_vec(),
            }
        }
    }

    impl Write for MyWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.output.flush()
        }
    }

    impl MyWriter {
        pub fn empty() -> Self {
            Self {
                output: [0; 255].to_vec(),
            }
        }
    }
}
