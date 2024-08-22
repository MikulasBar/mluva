use std::collections::HashMap;

pub type DataTypeMap = HashMap<String, DataType>;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataType {
    Num,
    Bool,
}