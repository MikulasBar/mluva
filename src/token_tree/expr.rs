use std::panic::PanicInfo;

use crate::data_type::DataType;
use crate::scope::DataTypeScope;
use crate::value::Value;
use crate::token::Token;
use crate::parser::TokenIter;
use crate::{data_type, expect_pat};
use crate::token_tree::operator::BinOp;

use super::TypedExpr;


#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(u64),
    Bool(bool),
    Var(String),
    BinOp(BinOp, Box<Self>, Box<Self>),
}

impl Expr {
    pub fn bin_op(op: BinOp, lhs: Self, rhs: Self) -> Self {
        Self::BinOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn is_data_type(&self,  data_type: DataType, scope: &DataTypeScope) -> bool {
        match data_type {
            DataType::Bool => self.is_bool_expr(scope),
            DataType::Num => self.is_num_expr(scope),
        }
    }

    pub fn is_num_expr(&self, scope: &DataTypeScope) -> bool {
        match self {
            Self::Num(_)
            | Self::BinOp(BinOp::Add, _, _)
                => true,

            Self::Var(var) => scope.get(var).unwrap().is_num(),

            _ => false,
        }
    }

    /// Checks if the expr is representing bool value.
    /// That is not only the [`Expr::Bool`], but as well as `BinOp(_, Eq, _)` and so on ...
    pub fn is_bool_expr(&self, scope: &DataTypeScope) -> bool {
        match self {
            Self::Bool(_)
            | Self::BinOp(BinOp::Eq, _, _)
                => true,

            Self::Var(var) => scope.get(var).unwrap().is_bool(),

            _ => false,
        }
    }
    
    pub fn parse(tokens: &mut TokenIter) -> Self {
        Self::parse_eq(tokens)
    }

    fn parse_eq(tokens: &mut TokenIter) -> Self {
        let mut lhs = Self::parse_add(tokens);

        if let Some(token) = tokens.peek() {
            match token {
                Token::Eq => {
                    expect_pat!(Token::Eq in ITER tokens);

                    let rhs = Self::parse_add(tokens);
                    lhs = Self::bin_op(BinOp::Eq, lhs, rhs)
                },

                _ => (),
            }
        }

        lhs
    }

    fn parse_add(tokens: &mut TokenIter) -> Self {
        let mut lhs = Self::parse_atom(tokens);

        while let Some(Token::Plus) = tokens.peek() {
            expect_pat!(Token::Plus in ITER tokens);

            let rhs = Self::parse_atom(tokens);
            lhs = Self::bin_op(BinOp::Add, lhs, rhs);
        }

        lhs
    }

    // will parse atom expr such as Ident, Num, Bool, not ops
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

            // most bottom parse method, we panic because we must have at least one of these
            _ => panic!(),
        }
    }

    /// Convert `Expr` to `TypedExpr`.
    /// And check the types.
    pub fn to_typed(self, scope: &DataTypeScope) -> TypedExpr {
        match self {
            Self::Bool(bool) => TypedExpr::Bool(bool, DataType::Bool),
            Self::Num(num) => TypedExpr::Num(num, DataType::Num),
            Self::Var(var) => {
                let data_type = scope.get(&var).unwrap();
                TypedExpr::Var(var, *data_type)
            },

            Self::BinOp(op, lhs, rhs) => {
                let lhs = lhs.to_typed(scope);
                let rhs = rhs.to_typed(scope);

                let lhs_t = lhs.get_type();
                let rhs_t = rhs.get_type();

                match (op, lhs_t, rhs_t) {
                    (BinOp::Add, DataType::Num, DataType::Num) => {
                        TypedExpr::bin_op(BinOp::Add, lhs, rhs, DataType::Num)
                    },

                    (BinOp::Eq, _, _) => {
                        TypedExpr::bin_op(BinOp::Eq, lhs, rhs, DataType::Bool)
                    },

                    _ => panic!(),
                }
            },
        }
    }
}
