use crate::data_type::DataType;


#[derive(Debug, Clone)]
pub enum Token {
    Semi,
    Assign,
    Plus,
    Print,
    DataType(DataType),
    Ident(String),
    Num(u64),
    Bool(bool),
}