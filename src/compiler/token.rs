use std::fmt::Display;

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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::EOF => write!(f, "end of file"),
            TokenKind::EOL => write!(f, "end of line"),
            TokenKind::Plus => write!(f, "'+'"),
            TokenKind::Minus => write!(f, "'-'"),
            TokenKind::Asterisk => write!(f, "'*'"),
            TokenKind::Slash => write!(f, "'/'"),
            TokenKind::Equal => write!(f, "'=='"),
            TokenKind::NotEqual => write!(f, "'!='"),
            TokenKind::Modulo => write!(f, "'%'"),
            TokenKind::Less => write!(f, "'<'"),
            TokenKind::LessEqual => write!(f, "'<='"),
            TokenKind::Greater => write!(f, "'>'"),
            TokenKind::GreaterEqual => write!(f, "'>='"),
            TokenKind::Assign => write!(f, "'='"),
            TokenKind::And => write!(f, "'&&'"),
            TokenKind::Or => write!(f, "'||'"),
            TokenKind::Not => write!(f, "'!'"),
            TokenKind::If => write!(f, "'if'"),
            TokenKind::Else => write!(f, "'else'"),
            TokenKind::Let => write!(f, "'let'"),
            TokenKind::While => write!(f, "'while'"),
            TokenKind::Return => write!(f, "'return'"),
            TokenKind::Import => write!(f, "'import'"),
            TokenKind::ParenL => write!(f, "'('"),
            TokenKind::ParenR => write!(f, "')'"),
            TokenKind::BracketL => write!(f, "'['"),
            TokenKind::BracketR => write!(f, "']'"),
            TokenKind::BraceL => write!(f, "'{{'"),
            TokenKind::BraceR => write!(f, "'}}'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::DotDot => write!(f, "'..'"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::DataType(dt) => write!(f, "data type {}", dt),
            TokenKind::Ident(name) => write!(f, "identifier '{}'", name),
            TokenKind::StringLiteral(s) => write!(f, "string literal \"{}\"", s),
            TokenKind::Int(i) => write!(f, "integer literal {}", i),
            TokenKind::Float(fl) => write!(f, "float literal {}", fl),
            TokenKind::Bool(b) => write!(f, "boolean literal {}", b),
        }
    }
}
