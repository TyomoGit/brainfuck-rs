/// トークン
pub struct Token {
    pub token_type: TokenType,
    /// ソースコード上の列番号
    pub column: usize,
    /// ソースコード上の行番号
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, column: usize, line: usize) -> Self {
        Self {
            token_type,
            column,
            line,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
}

/// トークンの種類
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    /// >
    RightAngle,
    /// <
    LeftAngle,

    /// +
    Plus,
    /// -
    Minus,

    /// .
    Dot,
    /// ,
    Comma,

    /// [
    LeftBracket,
    /// ]
    RightBracket,
}
