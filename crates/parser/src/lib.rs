use std::io::{Read, Write};

use bytecode_backend::interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

pub mod parser;
pub mod scanner;
pub mod token;
