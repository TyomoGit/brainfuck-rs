#![feature(test)]
extern crate test;

use std::io::{stdin, stdout};

use ast::opt::Optimizer;
use bytecode_backend::interpreter::Interpreter;
// use inkwell::{
//     context::Context,
//     targets::{self, TargetMachine},
// };
// use llvm_backend::compiler::{host_machine, Compiler};
use parser::parser::Parser;
use parser::scanner::Scanner;
use test::Bencher;

#[bench]
fn bench_mandelbrot(b: &mut Bencher) {
    const PROGRAM: &str = include_str!("../../../programs/mandelbrot.bf");

    let mut scanner = Scanner::new(PROGRAM.chars().collect());
    let tokens = scanner.scan_tokens();

    let parser = Parser::new(tokens);
    let parse_result = parser.parse_tokens();
    let Ok(program) = parse_result else {
        for error in parse_result.unwrap_err() {
            eprintln!("{}", error);
        }
        panic!("failed to parse tokens");
    };

    let optimizer = Optimizer::new();
    let insts = optimizer.optimize(program);

    let op_code = insts.into();

    let mut interpreter = Interpreter::new(op_code, stdin(), stdout());

    b.iter(|| interpreter.run());

    // let context = Context::create();
    // let machine = host_machine().unwrap();
    // let mut compiler = Compiler::new(&context, machine);
    // compiler.compile(insts);

    // b.iter(|| compiler.run_jit().unwrap())
}
