use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use brainfuck_rs::llvm;
use brainfuck_rs::parser::Parser;
use brainfuck_rs::scanner::Scanner;
use brainfuck_rs::vm;
use brainfuck_rs::vm::interpreter::Interpreter;
use inkwell::context::Context;
use inkwell::targets::{self, CodeModel, RelocMode, Target, TargetMachine};
use inkwell::OptimizationLevel;

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
    let mut compiler = llvm::compiler::Compiler::new(&context, machine);
    compiler.compile(program);
    compiler.write_to_file(Path::new("a.o")).unwrap();

    let object = link(Path::new("a.o")).unwrap();
    println!("output: {}", object.display());
    compiler.run_jit().unwrap();
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

// https://github.com/TheDan64/inkwell/issues/184
// https://qiita.com/_53a/items/d7d4e4fc250bfd945d9e
fn host_machine() -> Result<targets::TargetMachine> {
    Target::initialize_native(&targets::InitializationConfig::default())
        .map_err(|e| anyhow!("failed to initialize native target: {}", e))?;

    let triple = TargetMachine::get_default_triple();
    let target =
        Target::from_triple(&triple).map_err(|e| anyhow!("failed to create target: {}", e))?;

    let cpu = TargetMachine::get_host_cpu_name();
    let features = TargetMachine::get_host_cpu_features();

    let opt_level = OptimizationLevel::Aggressive;
    let reloc_mode = RelocMode::Default;
    let code_model = CodeModel::Default;

    target
        .create_target_machine(
            &triple,
            cpu.to_str()?,
            features.to_str()?,
            opt_level,
            reloc_mode,
            code_model,
        )
        .ok_or(anyhow!("failed to create target machine"))
}

// https://qiita.com/_53a/items/d7d4e4fc250bfd945d9e
fn link(object: &Path) -> anyhow::Result<PathBuf> {
    let mut output = PathBuf::from(object);
    output.set_extension("out");

    let process = std::process::Command::new("gcc")
        .args(vec![
            object.to_str().unwrap(),
            "-o",
            output.to_str().unwrap(),
        ])
        .output()?;

    if !process.status.success() {
        anyhow::bail!("{}", String::from_utf8_lossy(&process.stderr));
    }

    Ok(output)
}
