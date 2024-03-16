use brainfuck_rs::run;

use std::io::{stdin, stdout};

fn main() {
    let program = "
        ,.,.,.
    ";

    run(program, stdin(), stdout());
}
