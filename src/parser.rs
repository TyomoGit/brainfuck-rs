use std::{error::Error, fmt::Display};

use crate::{
    op::Op,
    token::{Token, TokenType},
};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
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

pub struct Parser {
    tokens: Vec<Token>,
    jump_stack: Vec<usize>,
    current: usize,

    result: Vec<Op>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            jump_stack: Vec::new(),
            result: Vec::new(),
        }
    }

    pub fn parse_tokens(mut self) -> Result<Vec<Op>, Vec<ParseError>> {
        let mut errors = Vec::new();
        for _ in 0..self.tokens.len() {
            match self.parse_token() {
                Ok(op) => self.result.push(op),
                Err(e) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(self.result)
        } else {
            Err(errors)
        }
    }

    fn parse_token(&mut self) -> Result<Op, ParseError> {
        let token = self.advance();
        let token_type = token.token_type();
        let op = match token_type {
            TokenType::InclementValue => Op::InclementValue,
            TokenType::DecrementValue => Op::DecrementValue,
            TokenType::InclementPointer => Op::InclementPointer,
            TokenType::DecrementPointer => Op::DecrementPointer,
            TokenType::Input => Op::Input,
            TokenType::Output => Op::Output,
            TokenType::LoopStart => {
                self.jump_stack.push(self.current);
                Op::LoopStart { if_zero: 0 }
            }
            TokenType::LoopEnd => {
                let loop_start = self.jump_stack.pop().ok_or(ParseError::IncompleteLoop)?;
                let loop_end = self.current;
                if let Op::LoopStart { if_zero } = &mut self.result[loop_start - 1] {
                    *if_zero = loop_end;
                } else {
                    return Err(ParseError::IncompleteLoop);
                };

                Op::LoopEnd {
                    if_non_zero: loop_start,
                }
            }
        };

        Ok(op)
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }
}
