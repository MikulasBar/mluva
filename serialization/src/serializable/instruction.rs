use crate::BytecodeSerializable;
use common::{instruction::Instruction, word::Word};

struct InstructionId;

impl InstructionId {
    const STORE: u8 = 0;
    const LOAD_LOCAL: u8 = 1;
    const LOAD_CONST: u8 = 2;
    const POP: u8 = 3;
    const WORD_EQUAL: u8 = 4;
    const WORD_NOT_EQUAL: u8 = 5;
    const JUMP: u8 = 6;
    const JUMP_IF_FALSE: u8 = 7;
    const LOCAL_CALL: u8 = 8;
    const FOREIGN_CALL: u8 = 9;
    const METHOD_CALL: u8 = 10;
    const RETURN: u8 = 11;
    const CREATE_STRING: u8 = 12;
    const CREATE_LIST: u8 = 13;
    const LIST_GET: u8 = 14;
    const LIST_SET: u8 = 15;
    const RC_INC: u8 = 16;
    const RC_DEC: u8 = 17;

    const INTRINSIC_PRINT: u8 = 18;

    const I32_ADD: u8 = 19;
    const I32_SUB: u8 = 20;
    const I32_MUL: u8 = 21;
    const I32_DIV: u8 = 22;
    const I32_MOD: u8 = 23;
    const I32_LESS: u8 = 24;
    const I32_LESS_EQUAL: u8 = 25;
    const I32_GREATER: u8 = 26;
    const I32_GREATER_EQUAL: u8 = 27;
    const I32_NEGATE: u8 = 28;

    const F32_ADD: u8 = 29;
    const F32_SUB: u8 = 30;
    const F32_MUL: u8 = 31;
    const F32_DIV: u8 = 32;
    const F32_MOD: u8 = 33;
    const F32_LESS: u8 = 34;
    const F32_LESS_EQUAL: u8 = 35;
    const F32_GREATER: u8 = 36;
    const F32_GREATER_EQUAL: u8 = 37;
    const F32_NEGATE: u8 = 38;

    const BOOL_AND: u8 = 39;
    const BOOL_OR: u8 = 40;
    const BOOL_NOT: u8 = 41;
}

fn get_id(instruction: &Instruction) -> u8 {
    match instruction {
        Instruction::Store { .. } => InstructionId::STORE,
        Instruction::LoadLocal { .. } => InstructionId::LOAD_LOCAL,
        Instruction::LoadConst(..) => InstructionId::LOAD_CONST,
        Instruction::Pop => InstructionId::POP,
        Instruction::WordEqual => InstructionId::WORD_EQUAL,
        Instruction::WordNotEqual => InstructionId::WORD_NOT_EQUAL,
        Instruction::Jump(..) => InstructionId::JUMP,
        Instruction::JumpIfFalse(..) => InstructionId::JUMP_IF_FALSE,
        Instruction::LocalCall { .. } => InstructionId::LOCAL_CALL,
        Instruction::ForeignCall { .. } => InstructionId::FOREIGN_CALL,
        Instruction::MethodCall { .. } => InstructionId::METHOD_CALL,
        Instruction::Return => InstructionId::RETURN,
        Instruction::CreateString { .. } => InstructionId::CREATE_STRING,
        Instruction::CreateList { .. } => InstructionId::CREATE_LIST,
        Instruction::ListGet => InstructionId::LIST_GET,
        Instruction::ListSet => InstructionId::LIST_SET,
        Instruction::RcInc => InstructionId::RC_INC,
        Instruction::RcDec => InstructionId::RC_DEC,

        Instruction::IntrinsicPrint => InstructionId::INTRINSIC_PRINT,

        Instruction::I32Add => InstructionId::I32_ADD,
        Instruction::I32Sub => InstructionId::I32_SUB,
        Instruction::I32Mul => InstructionId::I32_MUL,
        Instruction::I32Div => InstructionId::I32_DIV,
        Instruction::I32Mod => InstructionId::I32_MOD,
        Instruction::I32Less => InstructionId::I32_LESS,
        Instruction::I32LessEqual => InstructionId::I32_LESS_EQUAL,
        Instruction::I32Greater => InstructionId::I32_GREATER,
        Instruction::I32GreaterEqual => InstructionId::I32_GREATER_EQUAL,
        Instruction::I32Negate => InstructionId::I32_NEGATE,

        Instruction::F32Add => InstructionId::F32_ADD,
        Instruction::F32Sub => InstructionId::F32_SUB,
        Instruction::F32Mul => InstructionId::F32_MUL,
        Instruction::F32Div => InstructionId::F32_DIV,
        Instruction::F32Mod => InstructionId::F32_MOD,
        Instruction::F32Less => InstructionId::F32_LESS,
        Instruction::F32LessEqual => InstructionId::F32_LESS_EQUAL,
        Instruction::F32Greater => InstructionId::F32_GREATER,
        Instruction::F32GreaterEqual => InstructionId::F32_GREATER_EQUAL,
        Instruction::F32Negate => InstructionId::F32_NEGATE,

        Instruction::BoolAnd => InstructionId::BOOL_AND,
        Instruction::BoolOr => InstructionId::BOOL_OR,
        Instruction::BoolNot => InstructionId::BOOL_NOT,
    }
}

impl BytecodeSerializable for Instruction {
    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        get_id(self).write_bytecode(buffer);

        match self {
            Instruction::Jump(target) => target.write_bytecode(buffer),
            Instruction::JumpIfFalse(target) => target.write_bytecode(buffer),
            Instruction::LocalCall { slot } => slot.write_bytecode(buffer),
            Instruction::LoadLocal { slot } => slot.write_bytecode(buffer),
            Instruction::Store { slot } => slot.write_bytecode(buffer),
            Instruction::LoadConst(word) => word.data.write_bytecode(buffer),
            Instruction::ForeignCall {
                module_name,
                call_slot,
            } => {
                module_name.write_bytecode(buffer);
                call_slot.write_bytecode(buffer);
            }
            Instruction::MethodCall { type_id, slot } => {
                type_id.write_bytecode(buffer);
                slot.write_bytecode(buffer);
            }
            Instruction::CreateString { pool_slot } => {
                pool_slot.write_bytecode(buffer);
            }
            Instruction::CreateList {
                item_count,
                type_id,
            } => {
                type_id.write_bytecode(buffer);
                item_count.write_bytecode(buffer);
            }
            _ => (),
        }
    }

    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let id = u8::from_bytecode(bytes, cursor)?;

        match id {
            InstructionId::STORE => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Store { slot })
            }
            InstructionId::LOAD_LOCAL => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::LoadLocal { slot })
            }
            InstructionId::LOAD_CONST => {
                let data = u64::from_bytecode(bytes, cursor)?;
                let word = Word::new(data);
                Ok(Instruction::LoadConst(word))
            }
            InstructionId::POP => Ok(Instruction::Pop),
            InstructionId::WORD_EQUAL => Ok(Instruction::WordEqual),
            InstructionId::WORD_NOT_EQUAL => Ok(Instruction::WordNotEqual),
            InstructionId::JUMP => {
                let target = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Jump(target))
            }
            InstructionId::JUMP_IF_FALSE => {
                let target = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::JumpIfFalse(target))
            }
            InstructionId::LOCAL_CALL => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::LocalCall { slot })
            }
            InstructionId::FOREIGN_CALL => {
                let module_name = String::from_bytecode(bytes, cursor)?;
                let call_slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::ForeignCall {
                    module_name,
                    call_slot,
                })
            }
            InstructionId::METHOD_CALL => {
                let type_id = u32::from_bytecode(bytes, cursor)?;
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::MethodCall { type_id, slot })
            }
            InstructionId::RETURN => Ok(Instruction::Return),
            InstructionId::CREATE_STRING => {
                let pool_slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::CreateString { pool_slot })
            }
            InstructionId::CREATE_LIST => {
                let type_id = u32::from_bytecode(bytes, cursor)?;
                let item_count = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::CreateList {
                    item_count,
                    type_id,
                })
            }
            InstructionId::LIST_GET => Ok(Instruction::ListGet),
            InstructionId::LIST_SET => Ok(Instruction::ListSet),
            InstructionId::RC_INC => Ok(Instruction::RcInc),
            InstructionId::RC_DEC => Ok(Instruction::RcDec),
            InstructionId::INTRINSIC_PRINT => Ok(Instruction::IntrinsicPrint),
            InstructionId::I32_ADD => Ok(Instruction::I32Add),
            InstructionId::I32_SUB => Ok(Instruction::I32Sub),
            InstructionId::I32_MUL => Ok(Instruction::I32Mul),
            InstructionId::I32_DIV => Ok(Instruction::I32Div),
            InstructionId::I32_MOD => Ok(Instruction::I32Mod),
            InstructionId::I32_LESS => Ok(Instruction::I32Less),
            InstructionId::I32_LESS_EQUAL => Ok(Instruction::I32LessEqual),
            InstructionId::I32_GREATER => Ok(Instruction::I32Greater),
            InstructionId::I32_GREATER_EQUAL => Ok(Instruction::I32GreaterEqual),
            InstructionId::I32_NEGATE => Ok(Instruction::I32Negate),
            InstructionId::F32_ADD => Ok(Instruction::F32Add),
            InstructionId::F32_SUB => Ok(Instruction::F32Sub),
            InstructionId::F32_MUL => Ok(Instruction::F32Mul),
            InstructionId::F32_DIV => Ok(Instruction::F32Div),
            InstructionId::F32_MOD => Ok(Instruction::F32Mod),
            InstructionId::F32_LESS => Ok(Instruction::F32Less),
            InstructionId::F32_LESS_EQUAL => Ok(Instruction::F32LessEqual),
            InstructionId::F32_GREATER => Ok(Instruction::F32Greater),
            InstructionId::F32_GREATER_EQUAL => Ok(Instruction::F32GreaterEqual),
            InstructionId::F32_NEGATE => Ok(Instruction::F32Negate),
            InstructionId::BOOL_AND => Ok(Instruction::BoolAnd),
            InstructionId::BOOL_OR => Ok(Instruction::BoolOr),
            InstructionId::BOOL_NOT => Ok(Instruction::BoolNot),
            _ => Err(format!("Unknown instruction ID: {}", id)),
        }
    }
}
