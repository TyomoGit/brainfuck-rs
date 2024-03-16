use std::{error::Error, fmt::Display};

use crate::{
    ast::Instruction,
    token::{Token, TokenType},
};

/// 構文解析エラー
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    /// LeftBracketとRightBracketの対応関係の不備
    IncompleteLoop,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IncompleteLoop => write!(f, "incomplete loop"),
        }
    }
}

/// 構文解析器
pub struct Parser {
    /// トークン化済みのソースコード
    tokens: Vec<Token>,
    /// 現在の位置
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            // jump_stack: Vec::new(),
            // result: Vec::new(),
        }
    }

    pub fn parse_tokens(mut self) -> Result<Vec<Instruction>, Vec<ParseError>> {
        let mut errors = Vec::new();
        let mut result: Vec<Instruction> = Vec::new();
        while !self.is_at_end() {
            match self.parse_instruction() {
                Ok(op) => result.push(op),
                Err(e) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(result)
        } else {
            Err(errors)
        }
    }

    fn parse_instruction(&mut self) -> Result<Instruction, ParseError> {
        let token = self.advance();
        let token_type = token.token_type();
        let op = match token_type {
            TokenType::Plus => Instruction::InclementValue,
            TokenType::Minus => Instruction::DecrementValue,
            TokenType::RightAngle => Instruction::InclementPointer,
            TokenType::LeftAngle => Instruction::DecrementPointer,
            TokenType::Comma => Instruction::Input,
            TokenType::Dot => Instruction::Output,
            TokenType::LeftBracket => {
                // self.jump_stack.push(self.current);
                // Op::LoopStart { if_zero: 0 }
                Instruction::Loop(self.parse_loop())
            }
            TokenType::RightBracket => {
                // let loop_start = self.jump_stack.pop().ok_or(ParseError::IncompleteLoop)?;
                // let loop_end = self.current;
                // if let Op::LoopStart { if_zero } = &mut self.result[loop_start - 1] {
                //     *if_zero = loop_end;
                // } else {
                //     return Err(ParseError::IncompleteLoop);
                // };

                // Op::LoopEnd {
                //     if_non_zero: loop_start,
                // }
                return Err(ParseError::IncompleteLoop);
            }
        };

        Ok(op)
    }

    fn parse_loop(&mut self) -> Vec<Instruction> {
        let mut result: Vec<Instruction> = Vec::new();

        while *self.peek().token_type() != TokenType::RightBracket {
            result.push(self.parse_instruction().unwrap());
        }

        self.advance();

        result
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}
