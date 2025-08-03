use crate::value::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Store(u32),
    Load(u32),
    Push(Value),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    Negate,

    /// Jumps to a specific index in instruction list
    Jump(u32),
    JumpIfFalse(u32),
    Call {
        slot: u32,
        arg_count: u32,
    },
    Return,
}
