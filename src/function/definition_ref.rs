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

    // if the params are None, it means that the function is variadic
    pub fn arg_count(&self) -> Option<usize> {
        match *self {
            Self::External(external) => Some(external.params.as_deref()?.len()),
            Self::Internal(internal) => Some(internal.params.len()),
        }
    }

    pub fn check_arg_types(&self, types: &[DataType]) -> Result<(), CompileError> {
        check_arg_count(self.arg_count(), types.len())?;

        match self {
            Self::External(f) => {
                if f.params.is_none() {
                    return Ok(());
                }
                
                let params = f.params.as_deref().unwrap().iter();

                for (&expected, &found) in params.zip(types.into_iter()) {
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


fn check_arg_count(expected: Option<usize>, found: usize) -> Result<(), CompileError> {
    // If the function is variadic, we don't care about the number of arguments
    let Some(expected) = expected else {
        return Ok(());
    };

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