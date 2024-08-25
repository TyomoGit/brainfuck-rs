use std::io::{stdin, stdout, Write};

use runner::run;

fn main() {
    let program = include_str!("../../../programs/mandelbrot.bf");
    run(program, stdin(), stdout());
    stdout().flush().unwrap();
}
