#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    CommentStart,
    NewLine,

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
            '\n' => Token::NewLine,
            '#' => Token::CommentStart,
            _ => Token::Illegal,
        }
    }
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut result = Vec::new();
    for char in src.chars() {
        let token = Token::from(char);

        if token != Token::Illegal {
            result.push(token);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = "><+-.,[]あ\n#comment\n";
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
            Token::NewLine,
        ]);
    }
}