use std::iter::Peekable;
use std::slice::IterMut;
use std::vec::IntoIter;


use crate::data_type::{self, DataType, DataTypeMap};
use crate::token::Token;
use crate::token_tree::{statement::Stmt, expr::*};
use crate::scope::DataTypeScope as Scope;
use crate::expect_pat;


pub type TokenIter = Peekable<IntoIter<Token>>;

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut tokens = tokens.into_iter().peekable();
    let mut scope = Scope::new();

    parse_helper(&mut tokens, &mut scope, Token::EOF)
}

pub fn parse_helper(tokens: &mut TokenIter, scope: &mut Scope, critical_token: Token) -> Vec<Stmt> {
    let mut stmts = vec![];
    scope.enter();

    while let Some(token) = tokens.peek() {
        if *token == critical_token {
            tokens.next();
            break;
        }

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

                scope.insert(ident.clone(), data_type);
                let expr = parse_expr(tokens, scope, data_type);

                expect_pat!(Token::Semi             in ITER tokens);

                Stmt::var_assign(ident, expr)
            },

            // var assign
            Token::Ident(_) => {
                expect_pat!(Token::Ident(ident)     in ITER tokens);
                expect_pat!(Token::Assign           in ITER tokens);

                expect_pat!(&data_type              in MAP scope; &ident);
                
                let expr = parse_expr(tokens, scope, data_type);

                expect_pat!(Token::Semi             in ITER tokens);
                
                Stmt::var_assign(ident, expr)
            },
            
            // print
            Token::Print => {
                expect_pat!(Token::Print            in ITER tokens);
                expect_pat!(Token::Ident(ident)     in ITER tokens);
                expect_pat!(Token::Semi             in ITER tokens);

                Stmt::print(ident)
            },

            Token::If => {
                expect_pat!(Token::If               in ITER tokens);

                let cond = BoolExpr::parse(tokens, scope);

                expect_pat!(Token::BraceL           in ITER tokens);

                let stmts = parse_helper(tokens, scope, Token::BraceR);
                Stmt::if_statement(cond, stmts)
            }
            
            _ => panic!(),
        };

        stmts.push(stmt);
    }
    scope.exit();
    stmts
}


fn parse_expr(tokens: &mut TokenIter, scope: &mut Scope, data_type: DataType) -> Expr {
    match data_type {
        DataType::Num => {
            NumExpr::parse(tokens, scope).into()
        },

        DataType::Bool => {
            BoolExpr::parse(tokens, scope).into()
        },
    }
}

