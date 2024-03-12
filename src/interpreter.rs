use std::io::{Read, Write};

use crate::op::Op;

pub struct Interpreter {
    pub memory: Vec<u8>,
    pub mem_pointer: usize,

    pub code: Vec<Op>,
    pub ip: usize,
}

impl Interpreter {
    pub fn new(code: Vec<Op>) -> Self {
        Self {
            memory: vec![0; 30000],
            mem_pointer: 0,
            code,
            ip: 0,
        }
    }

    pub fn check_token_pointer(&self) -> bool {
        self.ip < self.code.len()
    }

    pub fn step(&mut self, read: &mut impl Read, write: &mut impl Write) {
        if !self.check_token_pointer() {
            return;
        }

        match *self.advance() {
            Op::InclementPointer => self.inclement_pointer(),
            Op::DecrementPointer => self.decrement_pointer(),
            Op::InclementValue => self.inclement_value(),
            Op::DecrementValue => self.decrement_value(),
            Op::Output => self.output(write),
            Op::Input => self.input(read),
            Op::LoopStart { if_zero } => self.loop_start(if_zero),
            Op::LoopEnd { if_non_zero } => self.loop_end(if_non_zero),
        }
    }

    pub fn run(&mut self, read: &mut impl Read, write: &mut impl Write) {
        while self.check_token_pointer() {
            self.step(read, write);
        }
    }

    fn advance(&mut self) -> &Op {
        let op = &self.code[self.ip];
        self.ip += 1;
        op
    }

    fn inclement_pointer(&mut self) {
        if self.mem_pointer == self.memory.len() - 1 {
            self.mem_pointer = 0;
        } else {
            self.mem_pointer += 1;
        }
    }

    fn decrement_pointer(&mut self) {
        if self.mem_pointer == 0 {
            self.mem_pointer = self.memory.len() - 1;
        } else {
            self.mem_pointer -= 1;
        }
    }

    fn inclement_value(&mut self) {
        self.memory[self.mem_pointer] = self.memory[self.mem_pointer].wrapping_add(1);
    }

    fn decrement_value(&mut self) {
        self.memory[self.mem_pointer] = self.memory[self.mem_pointer].wrapping_sub(1);
    }

    fn output(&mut self, write: &mut impl Write) {
        write.write_all(&[self.memory[self.mem_pointer]]).unwrap();
    }

    fn input(&mut self, read: &mut impl Read) {
        read.read_exact(&mut self.memory[self.mem_pointer..self.mem_pointer + 1])
            .unwrap();
    }

    fn loop_start(&mut self, index: usize) {
        if self.memory[self.mem_pointer] != 0 {
            return;
        }

        self.ip = index;
    }

    fn loop_end(&mut self, index: usize) {
        if self.memory[self.mem_pointer] == 0 {
            return;
        }

        self.ip = index;
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::token::*;

//     use super::*;

//     #[derive(Clone)]
//     struct MyReader {
//         pub input: Vec<u8>,
//     }

//     #[derive(Clone)]
//     struct MyWriter {
//         pub output: Vec<u8>,
//     }

//     impl Read for MyReader {
//         fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//             self.input.as_slice().read(buf)
//         }
//     }

//     impl MyReader {
//         pub fn empty() -> Self {
//             Self { input: vec![] }
//         }
//     }

//     impl Write for MyWriter {
//         fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//             self.output.write(buf)
//         }

//         fn flush(&mut self) -> std::io::Result<()> {
//             self.output.flush()
//         }
//     }

//     impl MyWriter {
//         pub fn empty() -> Self {
//             Self { output: vec![] }
//         }
//     }

//     #[test]
//     pub fn inclement_pointer() {
//         let mut interpreter = Interpreter::new(vec![TokenType::InclementPointer)]);
//         interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty());
//         assert_eq!(interpreter.pointer, 1);
//     }

//     #[test]
//     pub fn decrement_pointer() {
//         let mut interpreter = Interpreter::new(vec![TokenType::DecrementPointer)]);
//         interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty());
//         assert_eq!(interpreter.pointer, 29999);
//     }

//     #[test]
//     pub fn inclement_value() {
//         let mut interpreter = Interpreter::new(vec![TokenType::InclementValue)]);
//         interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty());
//         assert_eq!(interpreter.memory[0], 1);
//     }

//     #[test]
//     pub fn decrement_value() {
//         let mut interpreter = Interpreter::new(vec![TokenType::DecrementValue)]);
//         interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty());
//         assert_eq!(interpreter.memory[0], 255);
//     }

//     #[test]
//     pub fn output() {
//         let mut writer = MyWriter::empty();
//         let mut interpreter = Interpreter::new(vec![TokenType::Output)]);
//         interpreter.memory[0] = 65;
//         interpreter.step(&mut MyReader::empty(), &mut writer);
//         assert_eq!(writer.output, vec![65]);
//     }

//     #[test]
//     pub fn input() {
//         let mut reader = MyReader { input: vec![65] };
//         let mut interpreter = Interpreter::new(vec![TokenType::Input)]);
//         interpreter.step(&mut reader, &mut MyWriter::empty());
//         assert_eq!(interpreter.memory[0], 65);
//     }

//     #[test]
//     pub fn loop_basic() {
//         // ++[>-<-]
//         let mut interpreter = Interpreter::new(tokenize("++[>-<-]").unwrap());
//         interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty());
//         assert_eq!(interpreter.memory[0], 0);
//         assert_eq!(interpreter.memory[1], 254);
//         assert_eq!(interpreter.pointer, 0);
//     }

//     #[test]
//     // [[[[[]]]]]
//     pub fn loop_deep_zero() {
//         let mut interpreter = Interpreter::new(tokenize("[[[[[]]]]]").unwrap());
//         interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty());
//         assert_eq!(interpreter.memory[0], 0);
//     }
// }
