use std::fmt::Display;

use crate::{diagnostics::Span, errors::CompileError};

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Void,
    Int,
    Float,
    Bool,
    String,
    List { item_type: Option<Box<DataType>> },
}

impl DataType {
    pub fn unknow_list() -> Self {
        Self::List { item_type: None }
    }
    pub fn list_of(item_type: DataType) -> Self {
        Self::List {
            item_type: Some(Box::new(item_type)),
        }
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }

    pub fn matches_type(&self, other: &DataType) -> bool {
        match (self, other) {
            (DataType::List { item_type: None }, DataType::List { .. }) => true,
            (
                DataType::List {
                    item_type: Some(t1),
                },
                DataType::List {
                    item_type: Some(t2),
                },
            ) => t1.matches_type(t2),
            _ => self == other,
        }
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

            _ => Err(CompileError::method_not_found_at(
                self.clone(),
                method_name,
                span,
            )),
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
            DataType::List { item_type } => {
                write!(
                    f,
                    "list<{}>",
                    item_type
                        .as_ref()
                        .map(|t| t.to_string())
                        .unwrap_or("unknown".to_string())
                )
            }
        }
    }
}
