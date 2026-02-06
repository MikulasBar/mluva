use bincode::{Decode, Encode};

use crate::word::Word;

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
#[repr(u8)]
pub enum Instruction {
    Store {
        slot: u32,
    },
    LoadLocal {
        slot: u32,
    },
    LoadConst(Word),
    Drop,
    WordEqual,
    WordNotEqual,
    Jump(u32),
    JumpIfFalse(u32),
    Return,

    NewObject(u32),
    FunctionCall(u32),
    InterfaceCall {
        interface: u32,
        function_slot: u32,
    },
    FieldGet(u32),
    FieldSet(u32),


    I32Add,
    I32Sub,
    I32Mul,
    I32Div,
    I32Mod,
    I32Less,
    I32LessEqual,
    I32Greater,
    I32GreaterEqual,
    I32Negate,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Mod,
    F32Less,
    F32LessEqual,
    F32Greater,
    F32GreaterEqual,
    F32Negate,

    BoolAnd,
    BoolOr,
    BoolNot,
}
