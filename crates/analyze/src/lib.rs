use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use ast::inst::{Ast, AstCode, OpCode};

#[derive(Debug, Default)]
pub struct Analyzer {
    pub patterns_counter: HashMap<AstCode, usize>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            patterns_counter: HashMap::new(),
        }
    }

    pub fn count_patterns(&mut self, code: &AstCode) {
        *self.patterns_counter.entry(code.clone()).or_insert(0) += 1;

        for ast in code.vec() {
            if let Ast::Loop(code) = ast {
                self.count_patterns(code);
            }
        }
    }
}

impl Display for Analyzer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut counter = self.patterns_counter.iter().collect::<Vec<_>>();
        counter.sort_by(|a, b| b.1.cmp(a.1));

        for (key, value) in counter {
            writeln!(f, "[{}]: {}", key, value)?;
        }
        Ok(())
    }
}
