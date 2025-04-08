use crate::data_type::DataType;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EOF,
    EOL,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    NotEqual,
    Percentage,

    Assign,
    If,
    Else,
    Let,
    While,

    ParenL,
    ParenR,
    BracketL,
    BracketR,
    BraceL,
    BraceR,
    Comma,

    DataType(DataType),
    Ident(String),
    StringLiteral(String),
    Int(u64),
    Float(f64),
    Bool(bool),
}