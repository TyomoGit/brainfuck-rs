use std::io::{Read, Write};

pub mod token;
pub mod interpreter;

pub fn run(string: &str, read: &mut impl Read, write: &mut impl Write) {
    let tokens = token::tokenize(string).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    let mut interpreter = interpreter::Interpreter::new(tokens);
    interpreter.run(read, write);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MyReader {
        pub input: Vec<u8>,
    }

    #[derive(Clone)]
    struct MyWriter {
        pub output: Vec<u8>,
    }

    impl Read for MyReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.input.as_slice().read(buf)
        }
    }

    impl MyReader {
        pub fn empty() -> Self {
            Self { input: [0; 255].to_vec() }
        }
    }

    impl Write for MyWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.output.write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.output.flush()
        }
    }

    impl MyWriter {
        pub fn empty() -> Self {
            Self { output: [0; 255].to_vec() }
        }
    }

    #[test]
    fn incomplete_loop() {
        let program = "[+.";
        let tokens = token::tokenize(program).unwrap();
        let mut reader = MyReader::empty();
        let mut writer = MyWriter::empty();
        let mut interpreter = interpreter::Interpreter::new(tokens);
        interpreter.run(&mut reader, &mut writer);
        assert_eq!(writer.output[0], 0);
    }
}