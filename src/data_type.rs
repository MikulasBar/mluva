use std::fmt::Display;

use crate::{diagnostics::Span, errors::CompileError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Void,
    Int,
    Float,
    Bool,
    String,
}

impl DataType {
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }

    pub fn check_method_call(
        &self,
        method_name: &str,
        span: Span,
        args: &[DataType],
    ) -> Result<DataType, CompileError> {
        match (self, method_name, args) {
            (DataType::String, "length", args) => {
                if !args.is_empty() {
                    return Err(CompileError::wrong_number_of_arguments_at(
                        0,
                        args.len(),
                        span,
                    ));
                }

                Ok(DataType::Int)
            }

            _ => Err(CompileError::method_not_found_at(*self, method_name, span)),
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Void => write!(f, "void"),
            DataType::Int => write!(f, "int"),
            DataType::Float => write!(f, "float"),
            DataType::Bool => write!(f, "bool"),
            DataType::String => write!(f, "string"),
        }
    }
}
