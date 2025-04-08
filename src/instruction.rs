use crate::value::Value;



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
    Negate,
    Not,
    Jump(usize),
    JumpIfTrue(usize),
    Call(usize),
    Return,
}