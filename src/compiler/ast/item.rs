use crate::compiler::DataType;

use super::Stmt;


#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    FnDef(FunctionDef),
}


#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub return_type: DataType,
    pub args: Vec<(String, DataType)>,
    pub body: Vec<Stmt>,
}

impl FunctionDef {
    pub fn new(name: String, return_type: DataType, args: Vec<(String, DataType)>, body: Vec<Stmt>) -> Self {
        FunctionDef {
            name,
            return_type,
            args,
            body,
        }
    }
}