use crate::value::Value;


#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Store(usize),
    Load(usize),
    Push(Value),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    Equal,
    NotEqual,
    Jump(usize),
    JumpIfFalse(usize),
    Call {
        slot: usize,
        arg_count: usize,
    },
    Return,
}
