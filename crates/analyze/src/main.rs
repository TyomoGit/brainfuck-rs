use analyze::Analyzer;
use parser::{parser::Parser, scanner::Scanner};

fn main() {
    let program = include_str!("../../../programs/mandelbrot.bf");

    let mut scanner = Scanner::new(program.chars().collect());
    let tokens = scanner.scan_tokens();
    let parser = Parser::new(tokens);
    let ast_code = parser.parse_tokens().unwrap();

    let mut analyzer = Analyzer::new();
    analyzer.count_patterns(&ast_code);
    analyzer.patterns_counter.remove(&ast_code);

    println!("{}", analyzer);
}
