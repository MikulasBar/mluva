
#[derive(Debug, Clone)]
pub enum Token {
    Semi,
    Assign,
    Plus,
    Print,
    Ident(String),
    Num(u64),
    Bool(bool),
}