#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    InclementPointer,
    DecrementPointer,

    InclementValue,
    DecrementValue,

    Output,
    Input,

    LoopStart { if_zero: usize },
    LoopEnd { if_non_zero: usize },
}
