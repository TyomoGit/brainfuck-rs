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
        self.pointer += 1;
    }

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

    fn loop_start(&mut self) {
        if self.memory[self.pointer] == 0 {
            let mut depth = 1;
            while depth > 0 {
                self.token_pointer += 1;
                match self.tokens[self.token_pointer] {
                    TokenKind::LoopStart => depth += 1,
                    TokenKind::LoopEnd => depth -= 1,
                    _ => (),
                }
            }
        }
    }

    fn loop_end(&mut self) {
        if self.memory[self.pointer] != 0 {
            let mut depth = 1;
            while depth > 0 {
                self.token_pointer -= 1;
                match self.tokens[self.token_pointer] {
                    TokenKind::LoopStart => depth -= 1,
                    TokenKind::LoopEnd => depth += 1,
                    _ => (),
                }
            }
        }
    }


}