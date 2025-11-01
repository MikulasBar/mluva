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
    const FOREIGNCALL: u8 = 23;
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
        Instruction::Call { .. } => InstructionId::CALL,
        Instruction::Load { .. } => InstructionId::LOAD,
        Instruction::Store { .. } => InstructionId::STORE,
        Instruction::Pop => InstructionId::POP,
        Instruction::Push(_) => InstructionId::PUSH,
        Instruction::ForeignCall { .. } => InstructionId::FOREIGNCALL,
    }
}

impl BytecodeSerializable for Instruction {
    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        get_id(self).write_bytecode(buffer);

        match self {
            Instruction::Jump(target) => target.write_bytecode(buffer),
            Instruction::JumpIfFalse(target) => target.write_bytecode(buffer),
            Instruction::Call { call_slot } => call_slot.write_bytecode(buffer),
            Instruction::Load { slot } => slot.write_bytecode(buffer),
            Instruction::Store { slot } => slot.write_bytecode(buffer),
            Instruction::Push(value) => value.write_bytecode(buffer),
            Instruction::ForeignCall {
                module_name,
                call_slot,
            } => {
                module_name.write_bytecode(buffer);
                call_slot.write_bytecode(buffer);
            }
            _ => (),
        }
    }

    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        let id = u8::from_bytecode(bytes, cursor)?;

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
                let target = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Jump(target))
            }
            InstructionId::JUMPIFFALSE => {
                let target = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::JumpIfFalse(target))
            }
            InstructionId::CALL => {
                let call_slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Call { call_slot })
            }
            InstructionId::LOAD => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Load { slot })
            }
            InstructionId::STORE => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Store { slot })
            }
            InstructionId::PUSH => {
                let value = Value::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Push(value))
            }
            InstructionId::FOREIGNCALL => {
                let module_name = String::from_bytecode(bytes, cursor)?;
                let call_slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::ForeignCall {
                    module_name,
                    call_slot,
                })
            }
            _ => Err(format!("Unknown instruction ID: {}", id)),
        }
    }
}
