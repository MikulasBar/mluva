use std::iter::Peekable;
use std::vec::IntoIter;

use crate::data_type::{self, DataType, DataTypeMap};
use crate::token::Token;
use crate::token_tree::{statement::Stmt, expr::*};
use crate::expect_pat;


pub type TokenIter = Peekable<IntoIter<Token>>;

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut stmts = vec![];
    let mut tokens = tokens.into_iter().peekable();
    let mut datatypes: DataTypeMap = DataTypeMap::new();

    while let Some(token) = tokens.peek() {
        let stmt = match token {
            // lonely semicolon 
            Token::Semi => {
                tokens.next();
                continue;
            },

            // var declaration
            Token::DataType(_) => {
                expect_pat!(Token::DataType(data_type)  in ITER tokens);
                expect_pat!(Token::Ident(ident)         in ITER tokens);
                expect_pat!(Token::Assign               in ITER tokens);

                datatypes.insert(ident.clone(), data_type);
                let expr = parse_expr(&mut tokens, &datatypes, data_type);

                expect_pat!(Token::Semi                 in ITER tokens);

                Stmt::var_assign(ident, expr)
            },

            // var assign
            Token::Ident(_) => {
                expect_pat!(Token::Ident(ident)    in ITER tokens);
                expect_pat!(Token::Assign          in ITER tokens);

                expect_pat!(&data_type             in MAP datatypes; &ident);
                
                let expr = parse_expr(&mut tokens, &datatypes, data_type);

                expect_pat!(Token::Semi            in ITER tokens);
                
                Stmt::var_assign(ident, expr)
            },
            
            // print
            Token::Print => {
                expect_pat!(Token::Print           in ITER tokens);
                expect_pat!(Token::Ident(ident)    in ITER tokens);
                expect_pat!(Token::Semi            in ITER tokens);

                Stmt::print(ident)
            },
            
            _ => panic!(),
        };

        stmts.push(stmt);
    }

    stmts
}


fn parse_expr(tokens: &mut TokenIter, datatypes: &DataTypeMap, data_type: DataType) -> Expr {
    match data_type {
        DataType::Num => {
            let expr = NumExpr::expect_single(tokens, datatypes);
            NumExpr::parse(tokens, datatypes, expr).into()
        },

        DataType::Bool => {
            let expr = BoolExpr::expect_single(tokens, datatypes);
            BoolExpr::parse(tokens, datatypes, expr).into()
        },
    }
}

