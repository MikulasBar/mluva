use super::internal::InternalFunctionDefinition;
use super::external::ExternalFunctionDefinition;
use crate::errors::CompileError;

use crate::compiler::DataType;

#[derive(Debug, Clone)]
pub enum FunctionDefinition {
    External(ExternalFunctionDefinition),
    Internal(InternalFunctionDefinition),
}


impl FunctionDefinition {
    pub fn name(&self) -> &str {
        match self {
            Self::External(external) => external.name,
            Self::Internal(internal) => &internal.name,
        }
    }

    pub fn return_type(&self) -> DataType {
        match self {
            Self::External(external) => external.return_type,
            Self::Internal(internal) => internal.return_type,
        }
    }

    pub fn check_types(&self, types: &[DataType]) -> Result<(), CompileError> {
        match self {
            Self::External(func) => func.check_types(types),
            Self::Internal(func) => {
                if func.args.len() != types.len() {
                    return Err(CompileError::WrongNumberOfArguments {
                        expected: func.args.len(),
                        found: types.len(),
                    });
                }

                for (arg, arg_type) in func.args.iter().zip(types) {
                    if arg_type != &arg.1 {
                        return Err(CompileError::WrongType {
                            expected: arg.1.clone(),
                            found: arg_type.clone(),
                        });
                    }
                }

                Ok(())
            },
        }
    }
}