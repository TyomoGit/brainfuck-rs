use std::io::{Read, Write};

use crate::token::TokenKind;

pub struct Interpreter {
    pub memory: Vec<u8>,
    pub pointer: usize,
    pub tokens: Vec<TokenKind>,
    pub token_pointer: usize,
}

impl Interpreter {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self {
            memory: vec![0; 30000],
            pointer: 0,
            tokens,
            token_pointer: 0,
        }
    }

    pub fn step(
        &mut self,
        read: &mut impl Read,
        write: &mut impl Write
    ) {
        if self.token_pointer >= self.tokens.len() {
            return;
        }
        
        match self.tokens[self.token_pointer] {
            TokenKind::InclementPointer => self.inclement_pointer(),
            TokenKind::DecrementPointer => self.decrement_pointer(),
            TokenKind::InclementValue => self.inclement_value(),
            TokenKind::DecrementValue => self.decrement_value(),
            TokenKind::Output => self.output(write),
            TokenKind::Input => self.input(read),
            TokenKind::LoopStart => self.loop_start(),
            TokenKind::LoopEnd => self.loop_end(),
            _ => (),
        }
        self.token_pointer += 1;
    }

    pub fn run(
        &mut self,
        read: &mut impl Read,
        write: &mut impl Write
    ) {
        while self.token_pointer < self.tokens.len() {
            self.step(read, write);
        }
    }

    fn inclement_pointer(&mut self) {
        if self.pointer == self.memory.len() - 1 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
    }

    fn decrement_pointer(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.memory.len() - 1;
        } else {
            self.pointer -= 1;
        }
    }

    fn inclement_value(&mut self) {
        self.memory[self.pointer] += 1;
    }

    fn decrement_value(&mut self) {
        self.memory[self.pointer] -= 1;
    }

    fn output(&mut self, write: &mut impl Write) {
        write.write_all(&[self.memory[self.pointer]]).unwrap();
    }

    fn input(&mut self, read: &mut impl Read) {
        read.read_exact(&mut self.memory[self.pointer..self.pointer + 1]).unwrap();
    }

    fn loop_start(&mut self) {
        if self.memory[self.pointer] != 0 {
            return;
        }

        let mut depth = 1;
        while depth > 0 {
            self.token_pointer += 1;

            if self.token_pointer >= self.tokens.len() {
                break;
            }

            match self.tokens[self.token_pointer] {
                TokenKind::LoopStart => depth += 1,
                TokenKind::LoopEnd => depth -= 1,
                _ => (),
            }
        }
    }

    fn loop_end(&mut self) {
        if self.memory[self.pointer] == 0 {
            return;
        }

        let mut depth = 1;
        while depth > 0 {
            self.token_pointer -= 1;

            if self.token_pointer >= self.tokens.len() {
                break;
            }

            match self.tokens[self.token_pointer] {
                TokenKind::LoopStart => depth -= 1,
                TokenKind::LoopEnd => depth += 1,
                _ => (),
            }
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MyReader {
        pub input: Vec<u8>,
    }

    #[derive(Clone)]
    struct MyWriter {
        pub output: Vec<u8>,
    }

    impl Read for MyReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.input.as_slice().read(buf)
        }
    }

    impl MyReader {
        pub fn empty() -> Self {
            Self { input: vec![] }
        }
    }

    impl Write for MyWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.output.flush()
        }
    }

    impl MyWriter {
        pub fn empty() -> Self {
            Self { output: vec![] }
        }
    }

    #[test]
    pub fn inclement_pointer() {
        let mut interpreter = Interpreter::new(vec![TokenKind::InclementPointer]);
        interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty());
        assert_eq!(interpreter.pointer, 1);
    }

    #[test]
    pub fn decrement_pointer() {
        let mut interpreter = Interpreter::new(vec![TokenKind::DecrementPointer]);
        interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty());
        assert_eq!(interpreter.pointer, 29999);
    }

    #[test]
    pub fn inclement_value() {
        let mut interpreter = Interpreter::new(vec![TokenKind::InclementValue]);
        interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty());
        assert_eq!(interpreter.memory[0], 1);
    }

    #[test]
    pub fn decrement_value() {
        let mut interpreter = Interpreter::new(vec![TokenKind::DecrementValue]);
        interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty());
        assert_eq!(interpreter.memory[0], 255);
    }

    #[test]
    pub fn output() {
        let mut writer = MyWriter::empty();
        let mut interpreter = Interpreter::new(vec![TokenKind::Output]);
        interpreter.memory[0] = 65;
        interpreter.step(&mut MyReader::empty(), &mut writer);
        assert_eq!(writer.output, vec![65]);
    }

    #[test]
    pub fn input() {
        let mut reader = MyReader { input: vec![65] };
        let mut interpreter = Interpreter::new(vec![TokenKind::Input]);
        interpreter.step(&mut reader, &mut MyWriter::empty());
        assert_eq!(interpreter.memory[0], 65);
    }

    #[test]
    pub fn loop_basic() {
        let mut interpreter = Interpreter::new(vec![
            TokenKind::InclementValue,
            TokenKind::InclementValue,
            TokenKind::LoopStart,
            TokenKind::InclementPointer,
            TokenKind::DecrementValue,
            TokenKind::DecrementPointer,
            TokenKind::DecrementValue, 
            TokenKind::LoopEnd,
        ]);
        interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty());
        assert_eq!(interpreter.memory[0], 0);
        assert_eq!(interpreter.memory[1], 254);
        assert_eq!(interpreter.pointer, 0);
    }

    #[test]
    pub fn loop_deep_zero() {
        let mut interpreter = Interpreter::new(vec![
            TokenKind::LoopStart,
            TokenKind::LoopStart,
            TokenKind::LoopStart,
            TokenKind::LoopStart,
            TokenKind::LoopStart,
            TokenKind::LoopEnd,
            TokenKind::LoopEnd,
            TokenKind::LoopEnd,
            TokenKind::LoopEnd,
            TokenKind::LoopEnd,
        ]);
        interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty());
        assert_eq!(interpreter.memory[0], 0);
    }

    #[test]
    pub fn loop_error() {
        let mut interpreter = Interpreter::new(vec![
            TokenKind::LoopEnd,
        ]);
        interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty());
        assert_eq!(interpreter.memory[0], 0);
    }

    #[test]
    pub fn loop_error_nested() {
        let mut interpreter = Interpreter::new(vec![
            TokenKind::LoopStart,
            TokenKind::LoopEnd,
            TokenKind::LoopEnd,
        ]);
        interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty());
        assert_eq!(interpreter.memory[0], 0);
    }

}