use std::{fmt::Display, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    End,

    InclementPointer,
    DecrementPointer,

    InclementValue,
    DecrementValue,

    Output,
    Input,

    LoopStart,
    LoopEnd,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub jump: Option<usize>,
}

impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Self {
        Token {
            kind,
            jump: None,
        }
    }
}


fn parse(c: char, tokens: &mut Vec<Token>, jump_stack: &mut Vec<usize>) -> Result<()> {
    match c {
        '>' => tokens.push(Token::from(TokenKind::InclementPointer)),
        '<' => tokens.push(Token::from(TokenKind::DecrementPointer)),
        '+' => tokens.push(Token::from(TokenKind::InclementValue)),
        '-' => tokens.push(Token::from(TokenKind::DecrementValue)),
        '.' => tokens.push(Token::from(TokenKind::Output)),
        ',' => tokens.push(Token::from(TokenKind::Input)),
        '[' => {
            jump_stack.push(tokens.len());
            tokens.push(Token::from(TokenKind::LoopStart));
        }
        ']' => {
            let start = jump_stack.pop().ok_or(IncompleteLoopError)?;
            tokens[start].jump = Some(tokens.len());
            let token = Token {
                kind: TokenKind::LoopEnd,
                jump: Some(start-1),
            };
            tokens.push(token);
        }
        _ => {
            return Err(Box::new(IllegalCharacterError(c)));
        },
    };

    Ok(())
}


#[derive(Debug, PartialEq, Eq)]
pub struct IllegalCharacterError(char);
#[derive(Debug, PartialEq, Eq)]
pub struct IncompleteLoopError;

impl Error for IllegalCharacterError {}
impl Display for IllegalCharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is illegal character", self.0)
    }
}

impl Error for IncompleteLoopError {}
impl Display for IncompleteLoopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "incomplete loop")
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut jump_stack = Vec::<usize>::new();

    #[cfg(feature="comment")]
    let mut is_comment = false;

    for char in src.chars() {
        #[cfg(feature="comment")]
        {
            if char == '#' {
                is_comment = true;
            }
            if char == '\n' {
                is_comment = false;
                continue;
            }
            if is_comment {
                continue;
            }
        }

        if char.is_whitespace() {
            continue;
        }

        parse(char, &mut tokens, &mut jump_stack)?;
    }

    tokens.push(Token::from(TokenKind::End));

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = "><+-.,[]\n#comment...<<>{]\n";
        let tokens = tokenize(src).unwrap().iter().map(|e| {
            e.kind.clone()
        }).collect::<Vec<TokenKind>>();
        assert_eq!(tokens, vec![
            TokenKind::InclementPointer,
            TokenKind::DecrementPointer,
            TokenKind::InclementValue,
            TokenKind::DecrementValue,
            TokenKind::Output,
            TokenKind::Input,
            TokenKind::LoopStart,
            TokenKind::LoopEnd,
            TokenKind::End,
        ]);
    }
    #[test]
    fn crlf() {
        let src = "+++.#.\r\n";
        let tokens = tokenize(src).unwrap().iter().map(|e| {
            e.kind.clone()
        }).collect::<Vec<TokenKind>>();
        assert_eq!(tokens, vec![
            TokenKind::InclementValue,
            TokenKind::InclementValue,
            TokenKind::InclementValue,
            TokenKind::Output,
            TokenKind::End,
        ]);
    }
}