use crate::data_type::DataType;


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Semi,
    Assign,
    Plus,
    Print,
    If,
    BraceL,
    BraceR,
    EOF,
    DataType(DataType),
    Ident(String),
    Num(u64),
    Bool(bool),
}