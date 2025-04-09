use crate::{compiler::DataType, errors::{InterpreterError, CompileError}, value::Value};



pub struct ExternalFunction {
    pub name: &'static str,
    pub return_type: DataType,
    check_types: fn(&[DataType]) -> Result<(), CompileError>,
    call: fn(Vec<Value>) -> Result<Value, InterpreterError>,
}

impl ExternalFunction {
    pub fn check_types(&self, args: &[DataType]) -> Result<(), CompileError> {
        (self.check_types)(args)
    }

    pub fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        (self.call)(args)
    }
}

pub const PRINT_FUNCTION: ExternalFunction = ExternalFunction {
    name: "print",
    return_type: DataType::Void,
    check_types: |_args| Ok(()),
    call: |args| {
        for a in args {
            println!("{}", a);
        }
        Ok(Value::Void)
    },
};