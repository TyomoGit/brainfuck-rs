use crate::token::{Token, TokenType};

#[derive(Debug, Clone, Default)]
pub struct Scanner {
    source: Vec<char>,
    current: usize,
    column: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            current: 0,
            column: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut result: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            let Some(token) = self.scan_token() else {
                continue;
            };

            result.push(token);
        }

        result
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();
        let token_type = match c {
            '>' => TokenType::InclementPointer,
            '<' => TokenType::DecrementPointer,
            '+' => TokenType::InclementValue,
            '-' => TokenType::DecrementValue,
            '.' => TokenType::Output,
            ',' => TokenType::Input,
            '[' => TokenType::LoopStart,
            ']' => TokenType::LoopEnd,
            '\n' => {
                self.line += 1;
                self.column = 0;
                return None;
            }
            _ => return None,
        };

        Some(Token::new(token_type, self.column, self.line))
    }

    fn advance(&mut self) -> char {
        let result = self.source[self.current];
        self.current += 1;
        self.column += 1;
        result
    }

    fn peek(&self) -> char {
        *self.source.get(self.current).unwrap_or(&'\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
