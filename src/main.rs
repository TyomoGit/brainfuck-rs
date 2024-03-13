use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::{env, path};

use anyhow::Result;
use brainfuck_interpreter::llvm::compiler::{host_machine, Compiler};
use brainfuck_interpreter::parser::Parser;
use brainfuck_interpreter::scanner::Scanner;
use brainfuck_interpreter::vm::interpreter::{self, Interpreter};
use brainfuck_interpreter::vm;
use brainfuck_interpreter::llvm;
use inkwell::context::Context;

fn main() {
    let Some(file_name) = env::args().nth(1) else {
        repl();
        return;
    };
    let mut file = File::open(file_name).expect("failed to open file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("failed to read file");

    let mut scanner = Scanner::new(src.chars().collect());
    let tokens = scanner.scan_tokens();

    let parser = Parser::new(tokens);
    let parse_result = parser.parse_tokens();
    let Ok(program) = parse_result else {
        for error in parse_result.unwrap_err() {
            eprintln!("{}", error);
        }
        panic!("failed to parse tokens");
    };

    // let compiler = vm::compiler::Compiler::new();
    // let code = compiler.compile(program);
    // let mut interpreter = Interpreter::new(code, stdin(), stdout());
    // interpreter.run();


    let context = Context::create();
    let machine = host_machine().expect("failed to create machine");
    let mut converter = llvm::compiler::Compiler::new(&context, machine);
    converter.compile(program);
    converter.write_to_file(Path::new("a.o")).unwrap();

    let object = link(Path::new("a.o")).unwrap();
    println!("output: {}", object.display());
    converter.run_jit().unwrap();
}

fn repl() {
    let mut interpreter = Interpreter::new(vec![], stdin(), stdout());
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut src = String::new();
        stdin().read_line(&mut src).unwrap();
        let mut scanner = Scanner::new(src.chars().collect());
        let tokens = scanner.scan_tokens();
        let parser = Parser::new(tokens);
        let parse_result = parser.parse_tokens();
        let Ok(program) = parse_result else {
            for error in parse_result.unwrap_err() {
                eprintln!("{}", error);
            }
            continue;
        };
        
        let compiler = vm::compiler::Compiler::new();
        let code = compiler.compile(program);
        interpreter.update(code);
        
        interpreter.run();
    }
}

fn link(object: &Path) -> Result<PathBuf> {
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
