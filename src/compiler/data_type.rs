use std::fmt::Display;

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
