mod converter;
mod interpreter;
mod op;
mod parser;
mod scanner;
mod token;

use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::{env, path};

use anyhow::Result;
use converter::{host_machine, Converter};
use inkwell::context::Context;
use parser::Parser;
use scanner::Scanner;

fn main() {
    let file_name = env::args().nth(1).expect("no file name");
    let mut file = File::open(file_name).expect("failed to open file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("failed to read file");

    let mut scanner = Scanner::new(src.chars().collect());
    let tokens = scanner.scan_tokens();

    let parser = Parser::new(tokens);
    let parse_result = parser.parse_tokens();
    let Ok(code) = parse_result else {
        for error in parse_result.unwrap_err() {
            eprintln!("{}", error);
        }

        return;
    };

    // let mut interpreter = interpreter::Interpreter::new(code);
    // interpreter.run(&mut stdin(), &mut stdout());

    let context = Context::create();
    let machine = host_machine().expect("failed to create machine");

    let mut converter = Converter::new(&context, machine, code);
    converter.convert();
    converter.write_to_file(Path::new("a.o")).unwrap();

    let object = link(Path::new("a.o")).unwrap();
    println!("output: {}", object.display());
}

pub fn link(object: &Path) -> Result<PathBuf> {
    let mut output = PathBuf::from(object);
    output.set_extension("out");

    let _compile_process = std::process::Command::new("gcc")
        .args(vec![
            object.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .output()?;

    Ok(output)
}
