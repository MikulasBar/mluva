use crate::data_type::DataType;


#[derive(Debug, Clone, PartialEq)]
pub enum TypeCheckError {
    WrongType {
        expected: DataType,
        found: DataType,
    },
    VariableNotFound(String),
    FunctionNotFound(String),
}