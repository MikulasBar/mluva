use crate::{compiler::{DataType, Stmt}, errors::InterpreterError, instruction::Instruction, value::Value};





#[derive(Debug, Clone, PartialEq)]
pub struct InternalFunctionDefinition {
    pub name: String,
    pub return_type: DataType,
    pub args: Vec<(String, DataType)>,
    pub body: Vec<Stmt>,
}

impl InternalFunctionDefinition {
    pub fn new(name: String, return_type: DataType, args: Vec<(String, DataType)>, body: Vec<Stmt>) -> Self {
        InternalFunctionDefinition {
            name,
            return_type,
            args,
            body,
        }
    }
}


#[derive(Debug, Clone)]
pub struct InternalFunctionSource {
    pub slot_count: usize,
    pub body: Vec<Instruction>,
}

impl InternalFunctionSource {
    pub fn new(slot_count: usize, body: Vec<Instruction>) -> Self {
        Self {
            slot_count,
            body,
        }
    }

    pub fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        todo!()
    }
}