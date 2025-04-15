use crate::{compiler::DataType, errors::{InterpreterError, CompileError}, value::Value};


#[derive(Debug, Clone)]
pub struct ExternalFunction {
    pub def: ExternalFunctionDefinition,
    pub source: ExternalFunctionSource,
}


#[derive(Debug, Clone)]
pub struct ExternalFunctionDefinition {
    pub name: &'static str,
    pub return_type: DataType,
    pub check_types: fn(&[DataType]) -> Result<(), CompileError>,
}

impl ExternalFunctionDefinition {
    pub fn check_types(&self, types: &[DataType]) -> Result<(), CompileError> {
        (self.check_types)(types)
    }
}

#[derive(Debug, Clone)]
pub struct ExternalFunctionSource {
    pub call: fn(Vec<Value>) -> Result<Value, InterpreterError>,
}

impl ExternalFunctionSource {
    pub fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        (self.call)(args)
    }
}