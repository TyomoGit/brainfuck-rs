use std::io::{Read, Write};

use ast::opt::Optimizer;
use bytecode_backend::interpreter::Interpreter;
use parser::{parser::Parser, scanner::Scanner};

pub fn run(string: &str, read: impl Read, write: impl Write) {
    let mut scanner = Scanner::new(string.chars().collect());
    let tokens = scanner.scan_tokens();

    let parser = Parser::new(tokens);
    let parse_result = parser.parse_tokens();
    let Ok(program) = parse_result else {
        for error in parse_result.unwrap_err() {
            eprintln!("{}", error);
        }
        return;
    };

    // println!("{:?}\n\n", &program.vec()[..100]);

    let optimizer = Optimizer::new();
    let program = optimizer.optimize(program);

    // println!("{:?}", &program.vec()[..]);

    let code: ast::inst::OpCode = program.into();

    // println!("{:?}", code);

    let mut interpreter = Interpreter::new(code, read, write);
    interpreter.run();
}
