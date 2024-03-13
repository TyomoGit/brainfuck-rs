use std::io::{Read, Write};

use crate::op::Op;

pub struct Interpreter {
    memory: Vec<u8>,
    mem_pointer: usize,

    code: Vec<Op>,
    ip: usize,

    reader: Box<dyn Read>,
    writer: Box<dyn Write>,
}

impl Interpreter {
    pub fn new(code: Vec<Op>, read: impl Read + 'static, write: impl Write + 'static) -> Self {
        Self {
            memory: vec![0; 30000],
            mem_pointer: 0,
            code,
            ip: 0,
            reader: Box::new(read),
            writer: Box::new(write),
        }
    }

    pub fn check_token_pointer(&self) -> bool {
        self.ip < self.code.len()
    }

    pub fn step(&mut self) {
        if !self.check_token_pointer() {
            return;
        }

        match *self.advance() {
            Op::InclementPointer => self.inclement_pointer(),
            Op::DecrementPointer => self.decrement_pointer(),
            Op::InclementValue => self.inclement_value(),
            Op::DecrementValue => self.decrement_value(),
            Op::Output => self.output(),
            Op::Input => self.input(),
            Op::LoopStart { if_zero } => self.loop_start(if_zero),
            Op::LoopEnd { if_non_zero } => self.loop_end(if_non_zero),
        }
    }

    pub fn run(&mut self) {
        while self.check_token_pointer() {
            self.step();
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

    fn output(&mut self) {
        self.writer
            .write_all(&[self.memory[self.mem_pointer]])
            .unwrap();
    }

    fn input(&mut self) {
        self.reader
            .read_exact(&mut self.memory[self.mem_pointer..self.mem_pointer + 1])
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
