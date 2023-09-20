#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Illegal,

    InclementPointer,
    DecrementPointer,

    InclementValue,
    DecrementValue,

    Output,
    Input,

    LoopStart,
    LoopEnd,
}

impl From<char> for TokenKind {
    fn from(c: char) -> Self {
        match c {
            '>' => TokenKind::InclementPointer,
            '<' => TokenKind::DecrementPointer,
            '+' => TokenKind::InclementValue,
            '-' => TokenKind::DecrementValue,
            '.' => TokenKind::Output,
            ',' => TokenKind::Input,
            '[' => TokenKind::LoopStart,
            ']' => TokenKind::LoopEnd,
            _ => TokenKind::Illegal,
        }
    }
}

pub fn tokenize(src: &str) -> Vec<TokenKind> {
    let mut result = Vec::new();
    let mut is_comment = false;
    for char in src.chars() {
        if cfg!(feature="comment") {
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

        let token = match TokenKind::from(char) {
            TokenKind::Illegal => {
                eprintln!("illegal character: {}", char);
                std::process::exit(1);
            },
            token => token,
        };
        result.push(token);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = "><+-.,[]\n#comment...<<>{]\n";
        let tokens = tokenize(src);
        assert_eq!(tokens, vec![
            TokenKind::InclementPointer,
            TokenKind::DecrementPointer,
            TokenKind::InclementValue,
            TokenKind::DecrementValue,
            TokenKind::Output,
            TokenKind::Input,
            TokenKind::LoopStart,
            TokenKind::LoopEnd,
        ]);
    }
}