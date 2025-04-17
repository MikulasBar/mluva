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

mod froms {
    use crate::function::{ExternalFunctionSource, InternalFunctionSource};
    use super::FunctionSource;

    impl From<ExternalFunctionSource> for FunctionSource {
        fn from(source: ExternalFunctionSource) -> Self {
            FunctionSource::External(source)
        }
    }

    impl From<InternalFunctionSource> for FunctionSource {
        fn from(source: InternalFunctionSource) -> Self {
            FunctionSource::Internal(source)
        }
    }
}