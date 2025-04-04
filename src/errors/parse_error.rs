use crate::token::Token;


#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedChar(char),
    UnexpectedToken(Token),
    UnexpectedEndOfInput,
}