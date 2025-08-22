use crate::{bytecode::BytecodeSerializable, instruction::Instruction, value::Value};

struct InstructionId;

impl InstructionId {
    const RETURN: u8 = 0;
    const ADD: u8 = 1;
    const SUB: u8 = 2;
    const MUL: u8 = 3;
    const DIV: u8 = 4;
    const MODULO: u8 = 5;
    const EQUAL: u8 = 6;
    const NOTEQUAL: u8 = 7;
    const LESS: u8 = 8;
    const LESSEQUAL: u8 = 9;
    const GREATER: u8 = 10;
    const GREATEREQUAL: u8 = 11;
    const AND: u8 = 12;
    const OR: u8 = 13;
    const NOT: u8 = 14;
    const NEGATE: u8 = 15;
    const JUMP: u8 = 16;
    const JUMPIFFALSE: u8 = 17;
    const CALL: u8 = 18;
    const LOAD: u8 = 19;
    const STORE: u8 = 20;
    const POP: u8 = 21;
    const PUSH: u8 = 22;
}

fn get_id(instruction: &Instruction) -> u8 {
    match instruction {
        Instruction::Return => InstructionId::RETURN,
        Instruction::Add => InstructionId::ADD,
        Instruction::Sub => InstructionId::SUB,
        Instruction::Mul => InstructionId::MUL,
        Instruction::Div => InstructionId::DIV,
        Instruction::Modulo => InstructionId::MODULO,
        Instruction::Equal => InstructionId::EQUAL,
        Instruction::NotEqual => InstructionId::NOTEQUAL,
        Instruction::Less => InstructionId::LESS,
        Instruction::LessEqual => InstructionId::LESSEQUAL,
        Instruction::Greater => InstructionId::GREATER,
        Instruction::GreaterEqual => InstructionId::GREATEREQUAL,
        Instruction::And => InstructionId::AND,
        Instruction::Or => InstructionId::OR,
        Instruction::Not => InstructionId::NOT,
        Instruction::Negate => InstructionId::NEGATE,
        Instruction::Jump(_) => InstructionId::JUMP,
        Instruction::JumpIfFalse(_) => InstructionId::JUMPIFFALSE,
        Instruction::Call{..} => InstructionId::CALL,
        Instruction::Load{..} => InstructionId::LOAD,
        Instruction::Store{..} => InstructionId::STORE,
        Instruction::Pop => InstructionId::POP,
        Instruction::Push(_) => InstructionId::PUSH,
    }
}

impl BytecodeSerializable for Instruction {
    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.push(get_id(self));

        match self {
            Instruction::Jump(target) => {
                buffer.extend_from_slice(&target.to_le_bytes());
            },
            Instruction::JumpIfFalse(target) => {
                buffer.extend_from_slice(&target.to_le_bytes());
            },
            Instruction::Call { call_slot } => {
                buffer.extend_from_slice(&call_slot.to_le_bytes());
            },
            Instruction::Load { slot } => {
                buffer.extend_from_slice(&slot.to_le_bytes());
            },
            Instruction::Store { slot } => {
                buffer.extend_from_slice(&slot.to_le_bytes());
            },
            Instruction::Push(value) => {
                value.write_bytecode(buffer);
            },
            _ => {},
        }
    }

    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if cursor >= &mut bytes.len() {
            return Err("Cursor out of bounds".to_string());
        }

        let id = bytes[*cursor];
        *cursor += 1;

        match id {
            InstructionId::RETURN => Ok(Instruction::Return),
            InstructionId::ADD => Ok(Instruction::Add),
            InstructionId::SUB => Ok(Instruction::Sub),
            InstructionId::MUL => Ok(Instruction::Mul),
            InstructionId::DIV => Ok(Instruction::Div),
            InstructionId::MODULO => Ok(Instruction::Modulo),
            InstructionId::EQUAL => Ok(Instruction::Equal),
            InstructionId::NOTEQUAL => Ok(Instruction::NotEqual),
            InstructionId::LESS => Ok(Instruction::Less),
            InstructionId::LESSEQUAL => Ok(Instruction::LessEqual),
            InstructionId::GREATER => Ok(Instruction::Greater),
            InstructionId::GREATEREQUAL => Ok(Instruction::GreaterEqual),
            InstructionId::AND => Ok(Instruction::And),
            InstructionId::OR => Ok(Instruction::Or),
            InstructionId::NOT => Ok(Instruction::Not),
            InstructionId::NEGATE => Ok(Instruction::Negate),
            InstructionId::POP => Ok(Instruction::Pop),

            InstructionId::JUMP => {
                if *cursor + 4 > bytes.len() {
                    return Err("Insufficient bytes for Jump".to_string());
                }
                let target = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                *cursor += 4;
                Ok(Instruction::Jump(target))
            },
            InstructionId::JUMPIFFALSE => {
                if *cursor + 4 > bytes.len() {
                    return Err("Insufficient bytes for JumpIfFalse".to_string());
                }
                let target = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                *cursor += 4;
                Ok(Instruction::JumpIfFalse(target))
            },
            InstructionId::CALL => {
                if *cursor + 4 > bytes.len() {
                    return Err("Insufficient bytes for Call".to_string());
                }
                let call_slot = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                *cursor += 4;
                Ok(Instruction::Call { call_slot })
            },
            InstructionId::LOAD => {
                if *cursor + 4 > bytes.len() {
                    return Err("Insufficient bytes for Load".to_string());
                }
                let slot = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                *cursor += 4;
                Ok(Instruction::Load { slot })
            },
            InstructionId::STORE => {
                if *cursor + 4 > bytes.len() {
                    return Err("Insufficient bytes for Store".to_string());
                }
                let slot = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                *cursor += 4;
                Ok(Instruction::Store { slot })
            },
            InstructionId::PUSH => {
                let value = Value::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Push(value))
            },
            _ => Err(format!("Unknown instruction ID: {}", id)),
        }
    }
}
