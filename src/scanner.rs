use crate::token::{Token, TokenType};

/// 字句解析器
#[derive(Debug, Clone, Default)]
pub struct Scanner {
    /// ソースコード
    source: Vec<char>,
    /// 現在の位置
    current: usize,
    /// 現在の列
    column: usize,
    /// 現在の行
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

    /// 全てのトークンをスキャンする
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

    /// 現在のトークンをスキャンする
    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();
        let token_type = match c {
            '>' => TokenType::RightAngle,
            '<' => TokenType::LeftAngle,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '.' => TokenType::Dot,
            ',' => TokenType::Comma,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
