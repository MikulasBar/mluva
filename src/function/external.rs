use crate::{
    compiler::DataType,
    errors::InterpreterError,
    value::Value,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExternalFunctionDefinition {
    pub name: String,
    pub return_type: DataType,
    pub params: Vec<DataType>,
}

impl ExternalFunctionDefinition {
    pub fn new(name: impl Into<String>, return_type: DataType, params: Vec<DataType>) -> Self {
        Self {
            name: name.into(),
            return_type,
            params,
        }
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
