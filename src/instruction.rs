use std::fmt::Display;

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
    Negate,
    Not,
    Jump(usize),
    JumpIfTrue(usize),
    JumpIfFalse(usize),
    Call {
        slot: usize,
        arg_count: usize,
    },
    Return,
}

// impl Instruction {
//     pub fn to_bytecode(&self) -> Vec<u8> {
//         match self {
//             Instruction::Store(slot) => vec![0x01, *slot as u8],
//             Instruction::Load(slot) => vec![0x02, *slot as u8],
//             Instruction::Push(value) => {
//                 let mut bytecode = vec![0x03];
//                 bytecode.extend_from_slice(&value.to_bytes());
//                 bytecode
//             },
//             Instruction::Pop => vec![0x04],
//             Instruction::Add => vec![0x05],
//             Instruction::Sub => vec![0x06],
//             Instruction::Mul => vec![0x07],
//             Instruction::Div => vec![0x08],
//             Instruction::Modulo => vec![0x09],
//             Instruction::Equal => vec![0x0A],
//             Instruction::NotEqual => vec![0x0B],
//             Instruction::Negate => vec![0x0C],
//             Instruction::Not => vec![0x0D],
//             Instruction::Jump(offset) => {
//                 let mut bytecode = vec![0x0E];
//                 bytecode.extend_from_slice(&(*offset as u16).to_le_bytes());
//                 bytecode
//             },
//             Instruction::JumpIfTrue(offset) => {
//                 let mut bytecode = vec![0x0F];
//                 bytecode.extend_from_slice(&(*offset as u16).to_le_bytes());
//                 bytecode
//             },
//             Instruction::JumpIfFalse(offset) => {
//                 let mut bytecode = vec![0x10];
//                 bytecode.extend_from_slice(&(*offset as u16).to_le_bytes());
//                 bytecode
//             },
//             Instruction::Call { slot, arg_count } => {
//                 let mut bytecode = vec![0x11, *slot as u8, *arg_count as u8];
//                 bytecode
//             },
//             Instruction::Return => vec![0x12],
//         }
//     }
// }