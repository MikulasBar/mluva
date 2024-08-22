use std::collections::HashMap;

pub type DataTypeMap = HashMap<String, DataType>;


#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Num,
    Bool,
}