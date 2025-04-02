use crate::scope::MemoryScope;
use crate::token::Token;
use crate::parser::TokenIter;
use crate::value::Value;
use crate::expect_pat;
use crate::token_tree::operator::BinOp;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(u64),
    Bool(bool),
    Var(String),
    BinOp(BinOp, Box<Self>, Box<Self>),
}

impl Expr {
    pub fn new_bin_op(op: BinOp, lhs: Self, rhs: Self) -> Self {
        Self::BinOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn eval(&self, mem: &MemoryScope) -> Value {
        match self {
            &Self::Num(num)              => num.into(),
            &Self::Bool(bool)            => bool.into(),
            Self::Var(ident)             => mem[ident],
            Self::BinOp(op, lhs, rhs)    => op.apply(&*lhs, &*rhs, mem),
        }
    }
    
    pub fn parse(tokens: &mut TokenIter) -> Self {
        Self::parse_comp(tokens)
    }

    /// Parse eq and neq `BinOp`
    fn parse_comp(tokens: &mut TokenIter) -> Self {
        let mut lhs = Self::parse_add(tokens);

        if let Some(token) = tokens.peek() {
            let Some(op) = token_to_comp_op(token) else {return lhs};
            
            tokens.next();
            let rhs = Self::parse_add(tokens);
            lhs = Self::new_bin_op(op, lhs, rhs)
        }

        lhs
    }

    /// Parse add and subtract `BinOp`
    fn parse_add(tokens: &mut TokenIter) -> Self {
        let mut lhs = Self::parse_mul(tokens);

        while let Some(token) = tokens.peek() {
            let Some(op) = token_to_add_op(token) else {return lhs};

            tokens.next();
            let rhs = Self::parse_mul(tokens); 
            lhs = Self::new_bin_op(op, lhs, rhs);
        }

        lhs
    }

    /// Parse multiply, divide and modulo `BinOp`
    fn parse_mul(tokens: &mut TokenIter) -> Self {
        let mut lhs = Self::parse_atom(tokens);
        
        while let Some(token) = tokens.peek() {
            let Some(op) = token_to_mul_op(token) else {return lhs};

            tokens.next();
            let rhs = Self::parse_atom(tokens);
            lhs = Self::new_bin_op(op, lhs, rhs);
        }

        lhs
    }

    /// Parse atom expr such as Ident, Num, Bool, not ops.
    fn parse_atom(tokens: &mut TokenIter) -> Self {
        match tokens.peek().unwrap() {
            Token::Bool(_) => {
                expect_pat!(Token::Bool(bool) in ITER tokens);
                Expr::Bool(bool)
            },

            Token::Num(_) => {
                expect_pat!(Token::Num(num) in ITER tokens);
                Expr::Num(num)
            },

            Token::Ident(_) => {
                expect_pat!(Token::Ident(ident) in ITER tokens);
                Expr::Var(ident)
            },

            Token::ParenL => {
                expect_pat!(Token::ParenL in ITER tokens);
                let inner = Expr::parse(tokens);
                expect_pat!(Token::ParenR in ITER tokens);
                inner
            },

            // most bottom parse method, we panic because we must have at least one of these
            _ => panic!(),
        }
    }
}

fn token_to_comp_op(token: &Token) -> Option<BinOp> {
    match token {
        Token::Eq   => Some(BinOp::Eq),
        Token::Neq  => Some(BinOp::Neq),
        _ => None,
    }
}

fn token_to_add_op(token: &Token) -> Option<BinOp> {
    match token {
        Token::Plus     => Some(BinOp::Add),
        Token::Minus    => Some(BinOp::Sub),
        _ => None,
    }
}

fn token_to_mul_op(token: &Token) -> Option<BinOp> {
    match token {
        Token::Asterisk     => Some(BinOp::Mul),
        Token::Slash        => Some(BinOp::Div),
        Token::Percentage   => Some(BinOp::Modulo),
        _ => None,
    }
}
