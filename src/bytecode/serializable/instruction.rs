use std::str::FromStr as _;

use crate::{
    ast::BuiltinFunction, bytecode::BytecodeSerializable, instruction::Instruction, value::Value,
};

struct InstructionId;

impl InstructionId {
    const RETURN: u8 = 0;
    const ADD: u8 = 1;
    const SUB: u8 = 2;
    const MUL: u8 = 3;
    const DIV: u8 = 4;
    const MODULO: u8 = 5;
    const EQUAL: u8 = 6;
    const NOT_EQUAL: u8 = 7;
    const LESS: u8 = 8;
    const LESS_EQUAL: u8 = 9;
    const GREATER: u8 = 10;
    const GREATER_EQUAL: u8 = 11;
    const AND: u8 = 12;
    const OR: u8 = 13;
    const NOT: u8 = 14;
    const NEGATE: u8 = 15;
    const JUMP: u8 = 16;
    const JUMP_IF_FALSE: u8 = 17;
    const CALL: u8 = 18;
    const LOAD_COPY: u8 = 19;
    const STORE: u8 = 20;
    const POP: u8 = 21;
    const LOAD_CONST: u8 = 22;
    const FOREIGN_CALL: u8 = 23;
    const BUILTIN_CALL: u8 = 24;
    const METHOD_CALL: u8 = 25;
    const CREATE_LIST: u8 = 26;
    const INDEX_COPY: u8 = 27;
    const INDEX_REF: u8 = 28;
    const DEREF_COPY: u8 = 29;
    const LOAD_REF: u8 = 30;
    const INDEX_STORE: u8 = 31;
    const STORE_DEREF: u8 = 32;
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
        Instruction::NotEqual => InstructionId::NOT_EQUAL,
        Instruction::Less => InstructionId::LESS,
        Instruction::LessEqual => InstructionId::LESS_EQUAL,
        Instruction::Greater => InstructionId::GREATER,
        Instruction::GreaterEqual => InstructionId::GREATER_EQUAL,
        Instruction::And => InstructionId::AND,
        Instruction::Or => InstructionId::OR,
        Instruction::Not => InstructionId::NOT,
        Instruction::Negate => InstructionId::NEGATE,
        Instruction::Jump(_) => InstructionId::JUMP,
        Instruction::JumpIfFalse(_) => InstructionId::JUMP_IF_FALSE,
        Instruction::Call { .. } => InstructionId::CALL,
        Instruction::LoadCopy { .. } => InstructionId::LOAD_COPY,
        Instruction::Store { .. } => InstructionId::STORE,
        Instruction::Pop => InstructionId::POP,
        Instruction::LoadConst(_) => InstructionId::LOAD_CONST,
        Instruction::ForeignCall { .. } => InstructionId::FOREIGN_CALL,
        Instruction::BuiltinFunctionCall { .. } => InstructionId::BUILTIN_CALL,
        Instruction::MethodCall { .. } => InstructionId::METHOD_CALL,
        Instruction::CreateList { .. } => InstructionId::CREATE_LIST,
        Instruction::IndexCopy { .. } => InstructionId::INDEX_COPY,
        Instruction::IndexRef { .. } => InstructionId::INDEX_REF,
        Instruction::DerefCopy => InstructionId::DEREF_COPY,
        Instruction::LoadRef { .. } => InstructionId::LOAD_REF,
        Instruction::IndexStore { .. } => InstructionId::INDEX_STORE,
        Instruction::StoreDeref => InstructionId::STORE_DEREF,
    }
}

impl BytecodeSerializable for Instruction {
    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        get_id(self).write_bytecode(buffer);

        match self {
            Instruction::Jump(target) => target.write_bytecode(buffer),
            Instruction::JumpIfFalse(target) => target.write_bytecode(buffer),
            Instruction::Call { call_slot } => call_slot.write_bytecode(buffer),
            Instruction::LoadCopy { slot } => slot.write_bytecode(buffer),
            Instruction::Store { slot } => slot.write_bytecode(buffer),
            Instruction::LoadConst(value) => value.write_bytecode(buffer),
            Instruction::ForeignCall {
                module_name,
                call_slot,
            } => {
                module_name.write_bytecode(buffer);
                call_slot.write_bytecode(buffer);
            }
            Instruction::BuiltinFunctionCall {
                function,
                arg_count,
            } => {
                let function_name = function.as_str().to_string();
                function_name.write_bytecode(buffer);
                arg_count.write_bytecode(buffer);
            }

            Instruction::MethodCall {
                method_name,
                arg_count,
            } => {
                method_name.write_bytecode(buffer);
                arg_count.write_bytecode(buffer);
            }

            Instruction::CreateList { item_count } => {
                item_count.write_bytecode(buffer);
            }

            Instruction::IndexCopy { depth } => {
                depth.write_bytecode(buffer);
            }

            Instruction::IndexRef { depth } => {
                depth.write_bytecode(buffer);
            }

            Instruction::IndexStore { slot, depth } => {
                slot.write_bytecode(buffer);
                depth.write_bytecode(buffer);
            }

            Instruction::LoadRef { slot } => {
                slot.write_bytecode(buffer);
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
            InstructionId::NOT_EQUAL => Ok(Instruction::NotEqual),
            InstructionId::LESS => Ok(Instruction::Less),
            InstructionId::LESS_EQUAL => Ok(Instruction::LessEqual),
            InstructionId::GREATER => Ok(Instruction::Greater),
            InstructionId::GREATER_EQUAL => Ok(Instruction::GreaterEqual),
            InstructionId::AND => Ok(Instruction::And),
            InstructionId::OR => Ok(Instruction::Or),
            InstructionId::NOT => Ok(Instruction::Not),
            InstructionId::NEGATE => Ok(Instruction::Negate),
            InstructionId::POP => Ok(Instruction::Pop),

            InstructionId::JUMP => {
                let target = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Jump(target))
            }
            InstructionId::JUMP_IF_FALSE => {
                let target = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::JumpIfFalse(target))
            }
            InstructionId::CALL => {
                let call_slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Call { call_slot })
            }
            InstructionId::LOAD_COPY => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::LoadCopy { slot })
            }
            InstructionId::STORE => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::Store { slot })
            }
            InstructionId::LOAD_CONST => {
                let value = Value::from_bytecode(bytes, cursor)?;
                Ok(Instruction::LoadConst(value))
            }
            InstructionId::FOREIGN_CALL => {
                let module_name = String::from_bytecode(bytes, cursor)?;
                let call_slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::ForeignCall {
                    module_name,
                    call_slot,
                })
            }
            InstructionId::BUILTIN_CALL => {
                let function_name = String::from_bytecode(bytes, cursor)?;
                let arg_count = u32::from_bytecode(bytes, cursor)?;
                let function = BuiltinFunction::from_str(&function_name)?;
                Ok(Instruction::BuiltinFunctionCall {
                    function,
                    arg_count,
                })
            }
            InstructionId::METHOD_CALL => {
                let method_name = String::from_bytecode(bytes, cursor)?;
                let arg_count = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::MethodCall {
                    method_name,
                    arg_count,
                })
            }
            InstructionId::CREATE_LIST => {
                let item_count = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::CreateList { item_count })
            }
            InstructionId::INDEX_COPY => {
                let depth = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::IndexCopy { depth })
            }
            InstructionId::INDEX_REF => {
                let depth = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::IndexRef { depth })
            }
            InstructionId::INDEX_STORE => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                let depth = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::IndexStore { slot, depth })
            }
            InstructionId::DEREF_COPY => Ok(Instruction::DerefCopy),
            InstructionId::LOAD_REF => {
                let slot = u32::from_bytecode(bytes, cursor)?;
                Ok(Instruction::LoadRef { slot })
            }
            InstructionId::STORE_DEREF => Ok(Instruction::StoreDeref),
            _ => Err(format!("Unknown instruction ID: {}", id)),
        }
    }
}
