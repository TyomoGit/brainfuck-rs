pub struct Token {
    pub token_type: TokenType,
    pub column: usize,
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    InclementPointer,
    DecrementPointer,

    InclementValue,
    DecrementValue,

    Output,
    Input,

    LoopStart,
    LoopEnd,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_tokenize() {
//         let src = "><+-.,[]\n#comment...<<>{]\n";
//         let tokens = tokenize(src).unwrap();
//         assert_eq!(
//             tokens,
//             vec![
//                 TokenType::InclementPointer,
//                 TokenType::DecrementPointer,
//                 TokenType::InclementValue,
//                 TokenType::DecrementValue,
//                 TokenType::Output,
//                 TokenType::Input,
//                 TokenType::LoopStart(7),
//                 TokenType::LoopEnd(6),
//                 TokenType::End,
//             ]
//         );
//     }
//     #[test]
//     fn crlf() {
//         let src = "+++.#.\r\n";
//         let tokens = tokenize(src).unwrap();
//         assert_eq!(
//             tokens,
//             vec![
//                 TokenType::InclementValue,
//                 TokenType::InclementValue,
//                 TokenType::InclementValue,
//                 TokenType::Output,
//                 TokenType::End,
//             ]
//         );
//     }

//     #[test]
//     pub fn loop_error_1() {
//         assert!(tokenize("[[]").is_err());
//     }

//     #[test]
//     // []]
//     pub fn loop_error_2() {
//         assert!(tokenize("[]]").is_err())
//     }

//     #[test]
//     fn loop_error_3() {
//         let program = "[+.";
//         assert!(tokenize(program).is_err());
//     }
// }
