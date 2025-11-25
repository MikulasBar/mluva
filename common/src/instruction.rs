use crate::word::Word;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Store { slot: u32 },
    LoadLocal { slot: u32 },
    LoadConst(Word),
    Pop,

    I32Add,
    I32Sub,
    I32Mul,
    I32Div,
    I32Modulo,
    I32Less,
    I32LessEqual,
    I32Greater,
    I32GreaterEqual,
    I32Negate,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Modulo,
    F32Less,
    F32LessEqual,
    F32Greater,
    F32GreaterEqual,
    F32Negate,

    BoolAnd,
    BoolOr,
    BoolNot,

    Equal,
    NotEqual,
    Jump(u32),
    JumpIfFalse(u32),
    LocalCall { slot: u32 },
    ForeignCall { module_name: String, call_slot: u32 },
    Return,
    BuiltinFunctionCall { slot: u32, argc: u32 },
    MethodCall { type_id: u32, slot: u32 },
    CreateString { pool_slot: u32 },
    CreateList { item_count: u32, type_id: u32 },
    ListGet,
    ListSet,
    RcInc, // Increment reference count of the top value
    RcDec, // Decrement reference count of the top value
}
