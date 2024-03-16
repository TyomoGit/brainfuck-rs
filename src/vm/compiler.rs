use crate::ast::Instruction;

use super::op::Op;

pub struct Compiler {
    result: Vec<Op>,
}

impl Compiler {
    pub fn new() -> Self {
        Self { result: Vec::new() }
    }

    pub fn compile(mut self, program: Vec<Instruction>) -> Vec<Op> {
        for instruction in program {
            self.compile_instruction(instruction);
        }

        self.result
    }

    fn compile_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::InclementPointer => self.emit(Op::InclementPointer),
            Instruction::DecrementPointer => self.emit(Op::DecrementPointer),
            Instruction::InclementValue => self.emit(Op::InclementValue),
            Instruction::DecrementValue => self.emit(Op::DecrementValue),
            Instruction::Output => self.emit(Op::Output),
            Instruction::Input => self.emit(Op::Input),
            Instruction::Loop(instructions) => {
                self.emit(Op::LoopStart { if_zero: 0 });
                let loop_start = self.result.len() - 1;

                for instruction in instructions {
                    self.compile_instruction(instruction);
                }

                let loop_end = self.result.len();
                if let Op::LoopStart { if_zero } = &mut self.result[loop_start] {
                    *if_zero = loop_end + 1;
                } else {
                    unreachable!("Invalid LoopStart instruction");
                }

                self.emit(Op::LoopEnd {
                    if_non_zero: loop_start + 1,
                });
            }
        };
    }

    fn emit(&mut self, op: Op) {
        self.result.push(op);
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
