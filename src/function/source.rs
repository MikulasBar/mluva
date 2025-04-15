use crate::value::Value;
use crate::errors::InterpreterError;

use super::external::ExternalFunctionSource;
use super::internal::InternalFunctionSource;


#[derive(Debug, Clone)]
pub enum FunctionSource {
    External(ExternalFunctionSource),
    Internal(InternalFunctionSource),
}

impl FunctionSource {
    pub fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        match self {
            FunctionSource::External(source) => source.call(args),
            FunctionSource::Internal(source) => source.call(args),
        }
    }
}
