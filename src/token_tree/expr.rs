use crate::expect_pat;
use crate::errors::*;
use crate::parser::TokenIter;
use crate::scope::MemoryScope;
use crate::token::Token;
use crate::token_tree::operator::BinOp;
use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Int(u64),
    Float(f64),
    Bool(bool),
    Var(String),
    BinOp(BinOp, Box<Self>, Box<Self>),
    StringLiteral(String),
}

impl Expr {
    pub fn new_bin_op(op: BinOp, lhs: Self, rhs: Self) -> Self {
        Self::BinOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn eval(&self, mem: &MemoryScope) -> Result<Value, InterpreterError> {
        let result = match self {
            &Self::Int(num) => num.into(),
            &Self::Float(num) => num.into(),
            &Self::Bool(bool) => bool.into(),
            Self::Var(ident) => {
                if let Some(value) = mem.get(ident) {
                    value.clone()
                } else {
                    return Err(InterpreterError::UndefinedVariable(ident.clone()));
                }
            }
            Self::BinOp(op, lhs, rhs) => return op.apply(&*lhs, &*rhs, mem),
            Self::StringLiteral(string) => string.clone().into(),
        };

        Ok(result)
    }

    pub fn parse(tokens: &mut TokenIter) -> Result<Self, ParseError> {
        Self::parse_comp(tokens)
    }

    /// Parse eq and neq `BinOp`
    fn parse_comp(tokens: &mut TokenIter) -> Result<Self, ParseError> {
        let mut lhs = Self::parse_add(tokens)?;

        if let Some(token) = tokens.peek() {
            let Some(op) = token_to_comp_op(token) else {
                return Ok(lhs);
            };

            tokens.next();
            let rhs = Self::parse_add(tokens)?;
            lhs = Self::new_bin_op(op, lhs, rhs)
        }

        Ok(lhs)
    }

    /// Parse add and subtract `BinOp`
    fn parse_add(tokens: &mut TokenIter) -> Result<Self, ParseError> {
        let mut lhs = Self::parse_mul(tokens)?;

        while let Some(token) = tokens.peek() {
            let Some(op) = token_to_add_op(token) else {
                return Ok(lhs);
            };

            tokens.next();
            let rhs = Self::parse_mul(tokens)?;
            lhs = Self::new_bin_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// Parse multiply, divide and modulo `BinOp`
    fn parse_mul(tokens: &mut TokenIter) -> Result<Self, ParseError> {
        let mut lhs = Self::parse_atom(tokens)?;

        while let Some(token) = tokens.peek() {
            let Some(op) = token_to_mul_op(token) else {
                return Ok(lhs);
            };

            tokens.next();
            let rhs = Self::parse_atom(tokens)?;
            lhs = Self::new_bin_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// Parse atom expr such as Ident, Num, Bool, not ops.
    fn parse_atom(tokens: &mut TokenIter) -> Result<Self, ParseError> {
        let Some(token) = tokens.peek() else {
            return Err(ParseError::UnexpectedEndOfInput);
        };

        match token {
            Token::Bool(_) => {
                expect_pat!(Token::Bool(bool) in ITER tokens);
                Ok(Expr::Bool(bool))
            }

            Token::Int(_) => {
                expect_pat!(Token::Int(num) in ITER tokens);
                Ok(Expr::Int(num))
            }

            Token::Float(_) => {
                expect_pat!(Token::Float(num) in ITER tokens);
                Ok(Expr::Float(num))
            }

            Token::StringLiteral(_) => {
                expect_pat!(Token::StringLiteral(string) in ITER tokens);
                Ok(Expr::StringLiteral(string))
            }

            Token::Ident(_) => {
                expect_pat!(Token::Ident(ident) in ITER tokens);
                Ok(Expr::Var(ident))
            }

            Token::ParenL => {
                expect_pat!(Token::ParenL in ITER tokens);
                let inner = Expr::parse(tokens);
                expect_pat!(Token::ParenR in ITER tokens);
                inner
            }

            _ => {
                return Err(ParseError::UnexpectedToken(tokens.next().unwrap()));
            }
        }
    }
}

fn token_to_comp_op(token: &Token) -> Option<BinOp> {
    match token {
        Token::Eq => Some(BinOp::Eq),
        Token::Neq => Some(BinOp::Neq),
        _ => None,
    }
}

fn token_to_add_op(token: &Token) -> Option<BinOp> {
    match token {
        Token::Plus => Some(BinOp::Add),
        Token::Minus => Some(BinOp::Sub),
        _ => None,
    }
}

fn token_to_mul_op(token: &Token) -> Option<BinOp> {
    match token {
        Token::Asterisk => Some(BinOp::Mul),
        Token::Slash => Some(BinOp::Div),
        Token::Percentage => Some(BinOp::Modulo),
        _ => None,
    }
}
