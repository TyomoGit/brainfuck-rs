use std::fmt::{Debug, Display};

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct AstCode(Vec<Ast>);

impl AstCode {
    pub fn new(code: Vec<Ast>) -> Self {
        Self(code)
    }

    pub fn vec(&self) -> &Vec<Ast> {
        &self.0
    }

    pub fn vec_mut(&mut self) -> &mut Vec<Ast> {
        &mut self.0
    }
}

impl Display for AstCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ast in self.0.iter() {
            write!(f, "{}", ast)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ast {
    InclementPointer(usize),
    DecrementPointer(usize),
    InclementValue(usize),
    DecrementValue(usize),
    Output,
    Input,
    Loop(AstCode),

    /// 数を書き込む
    Load(u8),
    /// 現在の値をcount個右のセルに加える．
    SumRight(usize),
    /// 現在の値をcount個左のセルに加える．
    SumLeft(usize),
    /// per毎にメモリを右方向に見ていって，0なら終わる
    JumpZeroRight {
        per: usize,
    },
    /// per毎にメモリを左方向に見ていって，0なら終わる
    JumpZeroLeft {
        per: usize,
    },
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::InclementPointer(count) => write!(f, "> ({})", count),
            Ast::DecrementPointer(count) => write!(f, "< ({})", count),
            Ast::InclementValue(count) => write!(f, "+ ({})", count),
            Ast::DecrementValue(count) => write!(f, "- ({})", count),
            Ast::Output => write!(f, "Output"),
            Ast::Input => write!(f, "Input"),
            Ast::Loop(code) => write!(f, "[{}]", code),
            Ast::Load(n) => write!(f, "Load({})", n),
            Ast::SumRight(count) => write!(f, "SumRight({})", count),
            Ast::SumLeft(count) => write!(f, "SumLeft({})", count),
            Ast::JumpZeroRight { per } => write!(f, "JumpZeroRight(per:{})", per),
            Ast::JumpZeroLeft { per } => write!(f, "JumpZeroLeft(per:{})", per),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default, Hash)]
pub struct OpCode(Vec<Op>);

impl OpCode {
    pub fn new(code: Vec<Op>) -> Self {
        Self(code)
    }

    pub fn vec(&self) -> &Vec<Op> {
        &self.0
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Op {
    InclementPointer(usize),
    DecrementPointer(usize),
    InclementValue(usize),
    DecrementValue(usize),
    Output,
    Input,
    LoopStart {
        if_zero_add: usize,
    },
    LoopEnd {
        if_non_zero_sub: usize,
    },
    /// 数を書き込む
    Load(u8),
    /// 現在の値をcount個右のセルに加える
    SumRight(usize),
    /// 現在の値をcount個左のセルに加える
    SumLeft(usize),
    /// per毎にメモリを右方向に見ていって，0なら終わる
    JumpZeroRight {
        per: usize,
    },
    /// per毎にメモリを左方向に見ていって，0なら終わる
    JumpZeroLeft {
        per: usize,
    },
}

impl From<AstCode> for OpCode {
    fn from(value: AstCode) -> Self {
        let mut result: Vec<Op> = Vec::with_capacity(value.0.len());
        for instruction in value.0 {
            match instruction {
                Ast::InclementPointer(count) => result.push(Op::InclementPointer(count)),
                Ast::DecrementPointer(count) => result.push(Op::DecrementPointer(count)),
                Ast::InclementValue(count) => result.push(Op::InclementValue(count)),
                Ast::DecrementValue(count) => result.push(Op::DecrementValue(count)),
                Ast::Output => result.push(Op::Output),
                Ast::Input => result.push(Op::Input),
                Ast::Loop(code) => {
                    result.push(Op::LoopStart {
                        if_zero_add: 0,
                    });
                    let loop_start_index = result.len() - 1;
                    let loop_code: OpCode = code.into();
                    result.extend(loop_code.0);
                    result.push(Op::LoopEnd {
                        if_non_zero_sub: result.len() - loop_start_index,
                    });
                    result[loop_start_index] = Op::LoopStart {
                        if_zero_add: result.len() - loop_start_index - 1,
                    };
                }
                Ast::Load(n) => result.push(Op::Load(n)),
                Ast::SumRight(count) => result.push(Op::SumRight(count)),
                Ast::SumLeft(count) => result.push(Op::SumLeft(count)),
                Ast::JumpZeroRight { per } => result.push(Op::JumpZeroRight { per }),
                Ast::JumpZeroLeft { per } => result.push(Op::JumpZeroLeft { per }),
            }
        }

        Self(result)
    }
}

impl Debug for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut indent: usize = 0;
        writeln!(f, "OpCode(")?;

        for (i, op) in self.0.iter().enumerate() {
            if let Op::LoopEnd { .. } = op {
                indent -= 1;
            }

            writeln!(f, "{:4}:{}{:?}", i, "    ".repeat(indent), op)?;

            if let Op::LoopStart { .. } = op {
                indent += 1;
            }
        }

        writeln!(f, ")")?;

        Ok(())
    }
}

impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::InclementPointer(count) => write!(f, "> ({})", count),
            Op::DecrementPointer(count) => write!(f, "< ({})", count),
            Op::InclementValue(count) => write!(f, "+ ({})", count),
            Op::DecrementValue(count) => write!(f, "- ({})", count),
            Op::Output => write!(f, "Output"),
            Op::Input => write!(f, "Input"),
            Op::LoopStart { if_zero_add } => write!(f, "[ ({})", if_zero_add),
            Op::LoopEnd { if_non_zero_sub } => write!(f, "] ({})", if_non_zero_sub),
            Op::Load(n) => write!(f, "Load({})", n),
            Op::SumRight(count) => write!(f, "SumRight({})", count),
            Op::SumLeft(count) => write!(f, "SumLeft({})", count),
            Op::JumpZeroRight { per } => write!(f, "JumpZeroRight(per:{})", per),
            Op::JumpZeroLeft { per } => write!(f, "JumpZeroLeft(per:{})", per),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ast_to_op() {
        let code = AstCode::new(vec![
            Ast::InclementPointer(2),
            Ast::InclementValue(1),
            Ast::Loop(AstCode::new(vec![Ast::InclementValue(1)])),
        ]);
        let op = OpCode::from(code);
        // >>+[+]
        assert_eq!(
            op.0,
            vec![
                Op::InclementPointer(2),
                Op::InclementValue(1),
                Op::LoopStart { if_zero_add: 3 },
                Op::InclementValue(1),
                Op::LoopEnd { if_non_zero_sub: 2 },
            ]
        );
    }
}
