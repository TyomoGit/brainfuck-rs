#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    InclementPointer,
    DecrementPointer,
    InclementValue,
    DecrementValue,
    Output,
    Input,
    Loop(Vec<Instruction>),
}
