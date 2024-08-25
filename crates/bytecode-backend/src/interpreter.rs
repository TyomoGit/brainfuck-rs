use std::io::{stdin, BufRead, BufReader, BufWriter, Read, Write};

use ast::inst::{Op, OpCode};

#[derive(Debug)]
pub struct Interpreter<R: Read, W: Write> {
    memory: Vec<u8>,
    mem_pointer: usize,

    code: OpCode,
    ip: usize,

    read: BufReader<R>,
    write: BufWriter<W>,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(code: OpCode, read: R, write: W) -> Self {
        Self {
            memory: vec![0; 30000],
            mem_pointer: 0,
            code,
            ip: 0,
            read: BufReader::new(read),
            write: BufWriter::new(write),
        }
    }

    pub fn reader(&self) -> &BufReader<R> {
        &self.read
    }

    pub fn writer(&mut self) -> &BufWriter<W> {
        &self.write
    }

    pub fn update(&mut self, code: OpCode) {
        self.code = code;
        self.ip = 0;
    }

    pub fn run(&mut self) {
        while self.check_token_pointer() {
            self.step();
        }

        self.write.flush().unwrap();
    }

    pub fn step(&mut self) {
        if !self.check_token_pointer() {
            return;
        }

        match *self.advance() {
            Op::InclementPointer(count) => self.inclement_pointer(count),
            Op::DecrementPointer(count) => self.decrement_pointer(count),
            Op::InclementValue(count) => self.inclement_value(count),
            Op::DecrementValue(count) => self.decrement_value(count),
            Op::Output => self.output(),
            Op::Input => self.input(),
            Op::LoopStart { if_zero_add } => self.loop_start(if_zero_add),
            Op::LoopEnd { if_non_zero_sub } => self.loop_end(if_non_zero_sub),
            Op::Load(n) => self.load(n),
            Op::SumRight(count) => self.sum_right(count),
            Op::SumLeft(count) => self.sum_left(count),
            Op::JumpZeroRight { per } => self.jump_zero_right(per),
            Op::JumpZeroLeft { per } => self.jump_zero_left(per),
        }
    }

    fn check_token_pointer(&self) -> bool {
        self.ip < self.code.vec().len()
    }

    fn advance(&mut self) -> &Op {
        let op = &self.code.vec()[self.ip];
        self.ip += 1;
        op
    }

    fn inclement_pointer(&mut self, count: usize) {
        self.mem_pointer += count;
    }

    fn decrement_pointer(&mut self, count: usize) {
        self.mem_pointer -= count;
    }

    fn inclement_value(&mut self, count: usize) {
        self.memory[self.mem_pointer] =
            self.memory[self.mem_pointer].wrapping_add((count % u8::MAX as usize) as u8);
    }

    fn decrement_value(&mut self, count: usize) {
        self.memory[self.mem_pointer] =
            self.memory[self.mem_pointer].wrapping_sub((count % u8::MAX as usize) as u8);
    }

    fn output(&mut self) {
        self.write
            .write_all(&[self.memory[self.mem_pointer]])
            .unwrap();

        self.write.flush().unwrap();
    }

    fn input(&mut self) {
        self.read
            .read_exact(std::slice::from_mut(&mut self.memory[self.mem_pointer]))
            // .read_line(&mut self.memory[self.mem_pointer..self.mem_pointer + 1])
            .unwrap();

        
        dbg!(self.memory[self.mem_pointer] as char);
    }

    fn loop_start(&mut self, if_zero_add: usize) {
        if self.memory[self.mem_pointer] != 0 {
            return;
        }

        self.ip += if_zero_add;
    }

    fn loop_end(&mut self, if_non_zero_sub: usize) {
        if self.memory[self.mem_pointer] == 0 {
            return;
        }

        self.ip -= if_non_zero_sub;
    }

    fn load(&mut self, n: u8) {
        self.memory[self.mem_pointer] = n;
    }

    fn sum_right(&mut self, count: usize) {
        let value = self.memory[self.mem_pointer];
        let target_index = self.mem_pointer + count;
        self.memory[target_index] = self.memory[target_index].wrapping_add(value);
    }

    fn sum_left(&mut self, count: usize) {
        let value = self.memory[self.mem_pointer];
        let target_index = self.mem_pointer - count;
        self.memory[target_index] = self.memory[target_index].wrapping_add(value);
    }

    fn jump_zero_right(&mut self, per: usize) {
        while self.memory[self.mem_pointer] != 0 {
            self.mem_pointer += per;
        }
    }

    fn jump_zero_left(&mut self, per: usize) {
        while self.memory[self.mem_pointer] != 0 {
            self.mem_pointer -= per;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Default, Debug)]
    struct MyReader {
        pub input: Vec<u8>,
    }

    #[derive(Clone, Default, Debug)]
    struct MyWriter {
        pub output: Vec<u8>,
    }

    impl Read for MyReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.input.as_slice().read(buf)
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

    #[test]
    fn incl() {
        let mut interpreter = Interpreter::new(
            OpCode::new(vec![
                Op::InclementValue('H' as usize),
                Op::Output,
                Op::InclementValue('e' as usize - 'H' as usize),
                Op::Output,
                Op::InclementValue('l' as usize - 'e' as usize),
                Op::Output,
                Op::Output,
                Op::InclementValue('o' as usize - 'l' as usize),
                Op::Output,
            ]),
            MyReader::default(),
            MyWriter::default(),
        );
        interpreter.run();

        assert_eq!(
            interpreter.write.into_inner().unwrap().output,
            vec![b'H', b'e', b'l', b'l', b'o']
        );
    }

    #[test]
    fn includes_loop() {
        // +++++++[>++++++++++<-]>++.
        let code = OpCode::new(vec![
            Op::InclementValue(7),
            Op::LoopStart { if_zero_add: 6 },
            Op::InclementPointer(1),
            Op::InclementValue(10),
            Op::DecrementPointer(1),
            Op::DecrementValue(1),
            Op::LoopEnd { if_non_zero_sub: 5 },
            Op::InclementPointer(1),
            Op::InclementValue(2),
            Op::Output,
        ]);

        let mut interpreter = Interpreter::new(code, MyReader::default(), MyWriter::default());
        interpreter.run();

        assert_eq!(
            interpreter.write.into_inner().unwrap().output,
            String::from("H").into_bytes()
        )
    }

    #[test]
    fn input() {
        let input_string = "hello";
        let mut interpreter = Interpreter::new(
            OpCode::new(vec![
                Op::Input,
                Op::InclementPointer(1),
                Op::Input,
                Op::InclementPointer(1),
                Op::Input,
                Op::InclementPointer(1),
                Op::Input,
                Op::InclementPointer(1),
                Op::Input,
                Op::DecrementPointer(4),
                Op::Output,
                Op::InclementPointer(1),
                Op::Output,
                Op::InclementPointer(1),
                Op::Output,
                Op::InclementPointer(1),
                Op::Output,
                Op::InclementPointer(1),
                Op::Output,
            ]),
            MyReader {
                input: input_string.as_bytes().to_vec(),
            },
            MyWriter::default(),
        );
        interpreter.run();

        assert_eq!(
            interpreter.write.into_inner().unwrap().output,
            input_string.as_bytes()
        );
    }
}
