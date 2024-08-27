use std::collections::HashMap;

pub type DataTypeMap = HashMap<String, DataType>;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Num,
    Bool,
}


impl DataType {
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool)
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Self::Num)
    }
}