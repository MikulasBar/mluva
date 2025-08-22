use crate::value::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Store { slot: u32 },
    Load { slot: u32 },
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
    Call { call_slot: u32 },
    Return,
}
