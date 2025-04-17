use crate::{compiler::DataType, instruction::Instruction};
use crate::ast::Stmt;

#[derive(Debug, Clone, PartialEq)]
pub struct InternalFunctionDefinition {
    pub name: String,
    pub return_type: DataType,
    pub params: Vec<(String, DataType)>,
    pub body: Vec<Stmt>,
}

impl InternalFunctionDefinition {
    pub fn new(
        name: String,
        return_type: DataType,
        params: Vec<(String, DataType)>,
        body: Vec<Stmt>,
    ) -> Self {
        InternalFunctionDefinition {
            name,
            return_type,
            params,
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
        Self { slot_count, body }
    }
}
