#[derive(Debug, PartialEq)]
pub enum Token {
    Space,
    Illegal,
    Eof,

    InclementPointer,
    DecrementPointer,

    InclementValue,
    DecrementValue,

    Output,
    Input,

    LoopStart,
    LoopEnd,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '>' => Token::InclementPointer,
            '<' => Token::DecrementPointer,
            '+' => Token::InclementValue,
            '-' => Token::DecrementValue,
            '.' => Token::Output,
            ',' => Token::Input,
            '[' => Token::LoopStart,
            ']' => Token::LoopEnd,
            '\0' => Token::Eof,
            ' ' | '\n' => Token::Space,
            _ => Token::Illegal,
        }
    }
}

pub fn tokenize(src: &str) -> Vec<Token> {
    src.chars().map(Token::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = "><+-.,[]あ\0\n ";
        let tokens = tokenize(src);
        assert_eq!(tokens, vec![
            Token::InclementPointer,
            Token::DecrementPointer,
            Token::InclementValue,
            Token::DecrementValue,
            Token::Output,
            Token::Input,
            Token::LoopStart,
            Token::LoopEnd,
            Token::Illegal,
            Token::Eof,
            Token::Space,
            Token::Space,
        ]);
    }
}