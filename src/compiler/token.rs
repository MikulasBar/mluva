use crate::diagnostics::Span;

use super::data_type::DataType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    EOF,
    EOL,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    NotEqual,
    Modulo,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Assign,
    And,
    Or,
    Not,

    If,
    Else,
    Let,
    While,
    Return,
    External, // NOT USED
    Import,

    ParenL,
    ParenR,
    BracketL,
    BracketR,
    BraceL,
    BraceR,
    Comma,
    DotDot,
    Colon,

    DataType(DataType),
    Ident(String),
    StringLiteral(String),
    Int(i32),
    Float(f64),
    Bool(bool),
}
