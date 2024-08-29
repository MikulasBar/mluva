use crate::data_type::DataType;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EOL,
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Eq,
    Neq,
    Percentage,
    Print,
    If,
    Else,
    While,
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    EOF,
    DataType(DataType),
    Ident(String),
    Num(u64),
    Bool(bool),
}