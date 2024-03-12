use brainfuck_interpreter::run;

use std::io::{stdin, stdout};

fn main() {
    let program = "
        # Hello World in BF
        +++++++[>++++++++++<-]>++.        # H
        <++[>++++++++++<-]>+++++++++.     # e
        +++++++..                         # ll
        +++.                              # o
        >++++[>++++++++++<-]>++++.        # (comma)
        <+++[>----<-]>.                   # 
        >++++++++[>++++++++++<-]>+++++++. # W
        <<<<.                             # o
        +++.                              # r
        ------.                           # l
        <++[>----<-]>.                    # d
        >>+.                              # !
    ";

    run(program, &mut stdin(), &mut stdout());
}
