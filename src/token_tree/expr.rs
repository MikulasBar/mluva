use crate::value::Value;
use crate::token::Token;
use crate::interpreter::Memory;
use crate::parser::TokenIter;
use crate::expect_pat;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(NumExpr),
    Bool(BoolExpr),
    Var(String),
}

impl Expr {
    pub fn eval(&self, mem: &Memory) -> Value {
        match self {
            Self::Num(expr)     => expr.eval(mem).into(),
            Self::Bool(expr)    => expr.eval(mem).into(),
            Self::Var(ident)    => *mem.get(ident).unwrap()
        }
    } 
}

mod expr_froms {
    use super::*;

    impl From<BoolExpr> for Expr {
        fn from(value: BoolExpr) -> Self {
            Self::Bool(value)
        }
    }

    impl From<NumExpr> for Expr {
        fn from(value: NumExpr) -> Self {
            Self::Num(value)
        }
    }  
}

#[derive(Debug, Clone)]
pub enum NumExpr {
    Num(u64),
    Var(String),
    Add(Box<NumExpr>, Box<NumExpr>)
}

impl NumExpr {
    pub fn eval(&self, mem: &Memory) -> u64 {
        match self {
            Self::Num(num) => *num,
            Self::Var(ident) => mem.get(ident).unwrap().get_num(),
            Self::Add(lhs, rhs) => lhs.eval(mem) + rhs.eval(mem),
        }
    }

    pub fn add(lhs: Self, rhs: Self) -> Self {
        Self::Add(Box::new(lhs), Box::new(rhs))
    }

    pub fn parse(tokens: &mut TokenIter, prev: NumExpr) -> NumExpr {
        match tokens.peek().unwrap() {
            Token::Plus => {
                expect_pat!(Token::Plus in tokens);
            },

            // we dont care what the token is
            // its just that the possibilities of expression ends here
            // in the end of parse function we check that the semicolon is at the end of statement
            // so we dont have to check here
            _ => return prev,
        }

        let expr = match tokens.peek().unwrap() {
            Token::Ident(_) => {
                expect_pat!(Token::Ident(ident) in tokens);
                ident.into()
            },

            Token::Num(_) => {
                expect_pat!(Token::Num(num) in tokens);
                num.into()
            },

            _ => panic!(),
        };

        let rest = Self::parse(tokens, expr);
        Self::add(prev, rest)
    }
}

mod num_expr_froms {
    use super::*;

    impl From<u64> for NumExpr {
        fn from(value: u64) -> Self {
            Self::Num(value)
        }
    }

    impl From<String> for NumExpr {
        fn from(value: String) -> Self {
            Self::Var(value)
        }
    }
}



#[derive(Debug, Clone)]
pub enum BoolExpr {
    Bool(bool),
}

impl BoolExpr {
    pub fn eval(&self, mem: &Memory) -> bool {
        match *self {
            Self::Bool(bool) => bool,
        }
    }

    pub fn parse(tokens: &mut TokenIter, prev: bool) -> BoolExpr {
        Self::Bool(prev)
    }
}




