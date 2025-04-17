use super::internal::InternalFunctionDefinition;
use super::external::ExternalFunctionDefinition;
use crate::errors::CompileError;

use crate::compiler::DataType;

#[derive(Debug, Clone)]
pub enum FunctionDefinitionRef<'a> {
    External(&'a ExternalFunctionDefinition),
    Internal(&'a InternalFunctionDefinition),
}

impl<'a> FunctionDefinitionRef<'a> {
    pub fn name(&self) -> &str {
        match *self {
            Self::External(external) => &external.name,
            Self::Internal(internal) => &internal.name,
        }
    }

    pub fn return_type(&self) -> DataType {
        match *self {
            Self::External(external) => external.return_type,
            Self::Internal(internal) => internal.return_type,
        }
    }

    pub fn arg_count(&self) -> usize {
        match *self {
            Self::External(external) => external.params.len(),
            Self::Internal(internal) => internal.params.len(),
        }
    }

    pub fn check_arg_types(&self, types: &[DataType]) -> Result<(), CompileError> {
        check_arg_count(self.arg_count(), types.len())?;

        match self {
            Self::External(f) => {
                for (&expected, &found) in f.params.iter().zip(types.into_iter()) {
                    check_single_arg(expected, found)?;
                }
            }
            Self::Internal(f) => {
                for (&(_, expected), &found) in f.params.iter().zip(types.into_iter()) {
                    check_single_arg(expected, found)?;
                }
            }
        }

        Ok(())
    }
}


fn check_arg_count(expected: usize, found: usize) -> Result<(), CompileError> {
    if expected != found {
        return Err(CompileError::WrongNumberOfArguments {
            expected,
            found,
        });
    }

    Ok(())
}

fn check_single_arg(expected: DataType, found: DataType) -> Result<(), CompileError> {
    if expected != found {
        return Err(CompileError::WrongType {
            expected,
            found,
        });
    }

    Ok(())
}