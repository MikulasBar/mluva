use std::iter::Peekable;
use std::vec::IntoIter;

use crate::ast::*;
use crate::token::Token;
use crate::expect_pat;

pub type TokenIter = Peekable<IntoIter<Token>>;


pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut stmts = vec![];
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.peek() {
        let stmt = match token {
            // lonely semicolon 
            Token::Semi => {
                tokens.next();
                continue;
            },

            // var assign
            Token::Ident(_) => {
                expect_pat!(Token::Ident(ident)    in tokens);
                expect_pat!(Token::Assign          in tokens);

                let expr = parse_expr(&mut tokens);
                expect_pat!(Token::Semi            in tokens);
                
                Stmt::var_assign(ident, expr)
            },
            
            // print
            Token::Print => {
                expect_pat!(Token::Print           in tokens);
                expect_pat!(Token::Ident(ident)    in tokens);
                expect_pat!(Token::Semi            in tokens);

                Stmt::print(ident)
            },
            
            _ => panic!(),
        };

        stmts.push(stmt);
    }

    stmts
}


fn parse_expr(tokens: &mut TokenIter) -> Expr {
    let expr = match tokens.peek().unwrap() {
        Token::Num(_) => {
            expect_pat!(Token::Num(value) in tokens);
            NumExpr::parse(tokens, NumExpr::Num(value)).into()
        },

        Token::Bool(_) => {
            expect_pat!(Token::Bool(value) in tokens);
            BoolExpr::parse(tokens, value).into()
        }

        Token::Ident(_) => {
            expect_pat!(Token::Ident(ident) in tokens);
            parse_ident(tokens, ident)
        },

        _ => panic!(),
    };

    expr
}


fn parse_ident(tokens: &mut TokenIter, prev: String) -> Expr {
    match tokens.peek().unwrap() {
        Token::Plus => {
            NumExpr::parse(tokens, NumExpr::Var(prev)).into()
        },
        
        // same as in num expr
        // we dont care what token is this
        // we care about that the expression cannot continue to grow
        // the parse function check if the statement is ended by semicolon
        _ => return Expr::Var(prev),
    }
}

