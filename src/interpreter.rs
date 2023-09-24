use std::io::{Read, Write};

use crate::token::{TokenKind, Token, IncompleteLoopError};

pub struct Interpreter {
    pub memory: Vec<u8>,
    pub pointer: usize,

    pub tokens: Vec<Token>,
    pub token_pointer: usize,
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            memory: vec![0; 30000],
            pointer: 0,
            tokens,
            token_pointer: 0,
        }
    }

    pub fn check_token_pointer(&self) -> bool {
        self.token_pointer < self.tokens.len() && self.tokens[self.token_pointer].kind != TokenKind::End
    }

    pub fn step(
        &mut self,
        read: &mut impl Read,
        write: &mut impl Write
    ) -> Result<(), IncompleteLoopError> {
        if !self.check_token_pointer() {
            return Ok(());
        }

        match self.tokens[self.token_pointer].kind {
            TokenKind::InclementPointer => self.inclement_pointer(),
            TokenKind::DecrementPointer => self.decrement_pointer(),
            TokenKind::InclementValue => self.inclement_value(),
            TokenKind::DecrementValue => self.decrement_value(),
            TokenKind::Output => self.output(write),
            TokenKind::Input => self.input(read),
            TokenKind::LoopStart => self.loop_start()?,
            TokenKind::LoopEnd => self.loop_end()?,
            TokenKind::End => return  Ok(()),
        }
        self.token_pointer += 1;

        Ok(())
    }

    pub fn run(
        &mut self,
        read: &mut impl Read,
        write: &mut impl Write
    ) -> Result<(), IncompleteLoopError> {
        while self.check_token_pointer() {
            self.step(read, write)?;
        }

        Ok(())
    }

    #[cfg(feature="pointer_flow")]
    fn inclement_pointer(&mut self) {
        if self.pointer == self.memory.len() - 1 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
    }

    #[cfg(not(feature="pointer_flow"))]
    fn inclement_pointer(&mut self) {
        self.pointer += 1;
    }

    #[cfg(feature="pointer_flow")]
    fn decrement_pointer(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.memory.len() - 1;
        } else {
            self.pointer -= 1;
        }
    }

    #[cfg(not(feature="pointer_flow"))]
    fn decrement_pointer(&mut self) {
        self.pointer -= 1;
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

    fn loop_start(&mut self) -> Result<(), IncompleteLoopError> {
        if self.memory[self.pointer] != 0 {
            return Ok(());
        }

        self.token_pointer = match self.tokens[self.token_pointer].jump {
            Some(jump) => jump,
            None =>return Err(IncompleteLoopError),
        };

        Ok(())
    }

    fn loop_end(&mut self) -> Result<(), IncompleteLoopError>{
        if self.memory[self.pointer] == 0 {
            return Ok(());
        }

        self.token_pointer = match self.tokens[self.token_pointer].jump {
            Some(jump) => jump,
            None => return Err(IncompleteLoopError),
        };

        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use crate::token::*;

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
        let mut interpreter = Interpreter::new(vec![Token::from(TokenKind::InclementPointer)]);
        interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty()).unwrap();
        assert_eq!(interpreter.pointer, 1);
    }

    #[test]
    pub fn decrement_pointer() {
        let mut interpreter = Interpreter::new(vec![Token::from(TokenKind::DecrementPointer)]);
        interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty()).unwrap();
        assert_eq!(interpreter.pointer, 29999);
    }

    #[test]
    pub fn inclement_value() {
        let mut interpreter = Interpreter::new(vec![Token::from(TokenKind::InclementValue)]);
        interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty()).unwrap();
        assert_eq!(interpreter.memory[0], 1);
    }

    #[test]
    pub fn decrement_value() {
        let mut interpreter = Interpreter::new(vec![Token::from(TokenKind::DecrementValue)]);
        interpreter.step(&mut MyReader::empty(), &mut MyWriter::empty()).unwrap();
        assert_eq!(interpreter.memory[0], 255);
    }

    #[test]
    pub fn output() {
        let mut writer = MyWriter::empty();
        let mut interpreter = Interpreter::new(vec![Token::from(TokenKind::Output)]);
        interpreter.memory[0] = 65;
        interpreter.step(&mut MyReader::empty(), &mut writer).unwrap();
        assert_eq!(writer.output, vec![65]);
    }

    #[test]
    pub fn input() {
        let mut reader = MyReader { input: vec![65] };
        let mut interpreter = Interpreter::new(vec![Token::from(TokenKind::Input)]);
        interpreter.step(&mut reader, &mut MyWriter::empty()).unwrap();
        assert_eq!(interpreter.memory[0], 65);
    }

    #[test]
    pub fn loop_basic() {
        // ++[>-<-]
        let mut interpreter = Interpreter::new(tokenize("++[>-<-]").unwrap());
        interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty()).unwrap();
        assert_eq!(interpreter.memory[0], 0);
        assert_eq!(interpreter.memory[1], 254);
        assert_eq!(interpreter.pointer, 0);
    }

    #[test]
    // [[[[[]]]]]
    pub fn loop_deep_zero() {
        let mut interpreter = Interpreter::new(tokenize("[[[[[]]]]]").unwrap());
        interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty()).unwrap();
        assert_eq!(interpreter.memory[0], 0);
    }

    #[test]
    pub fn loop_error_1() {
        let tokens = tokenize("[[]").unwrap();
        let mut interpreter = Interpreter::new(tokens);
        assert!(
            interpreter.run(&mut MyReader::empty(), &mut MyWriter::empty()).is_err()
        )
    }

    #[test]
    // []]
    pub fn loop_error_2() {
        assert!(
            tokenize("[]]").is_err()
        )
    }

}