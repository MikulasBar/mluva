use std::iter::Peekable;
use std::vec::IntoIter;

use crate::parse_error::ParseError;
use crate::token::Token;
use crate::token_tree::{Stmt, expr::*};
use crate::expect_pat;


pub type TokenIter = Peekable<IntoIter<Token>>;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
    let mut tokens = tokens.into_iter().peekable();

    parse_helper(&mut tokens, Token::EOF)
}

fn parse_helper(tokens: &mut TokenIter, critical_token: Token) -> Result<Vec<Stmt>, ParseError> {
    let mut stmts = vec![];

    while let Some(token) = tokens.peek() {
        if *token == critical_token {
            tokens.next();
            break;
        }

        let stmt = match token {
            // lonely EOL
            Token::EOL => {
                tokens.next();
                continue;
            },

            // var declaration with explicit type
            Token::DataType(_) => {
                expect_pat!(Token::DataType(data_type)  in ITER tokens);
                expect_pat!(Token::Ident(ident)         in ITER tokens);
                expect_pat!(Token::Assign               in ITER tokens);

                let expr = Expr::parse(tokens)?;

                expect_pat!(Token::EOL             in ITER tokens);

                Stmt::VarDeclare(Some(data_type), ident, expr)
            },

            Token::Let => {
                expect_pat!(Token::Let  in ITER tokens);
                expect_pat!(Token::Ident(ident)         in ITER tokens);
                expect_pat!(Token::Assign               in ITER tokens);

                let expr = Expr::parse(tokens)?;

                expect_pat!(Token::EOL             in ITER tokens);

                Stmt::VarDeclare(None, ident, expr)
            }

            // var assign
            Token::Ident(_) => {
                expect_pat!(Token::Ident(ident)     in ITER tokens);
                expect_pat!(Token::Assign           in ITER tokens);
                
                let expr = Expr::parse(tokens)?;

                expect_pat!(Token::EOL             in ITER tokens);
                
                Stmt::VarAssign(ident, expr)
            },
            
            // print
            Token::Print => {
                expect_pat!(Token::Print            in ITER tokens);

                let expr = Expr::parse(tokens)?;

                expect_pat!(Token::EOL             in ITER tokens);

                Stmt::Print(expr)
            },

            Token::If => {
                expect_pat!(Token::If               in ITER tokens);

                let cond = Expr::parse(tokens)?;

                expect_pat!(Token::BraceL           in ITER tokens);

                let stmts = parse_helper(tokens, Token::BraceR)?;
                Stmt::If(cond, stmts)
            },

            Token::While => {
                expect_pat!(Token::While in ITER tokens);

                let cond = Expr::parse(tokens)?;

                expect_pat!(Token::BraceL           in ITER tokens);

                let stmts = parse_helper(tokens, Token::BraceR)?;
                Stmt::While(cond, stmts)
            },
            
            _ => {
                return Err(ParseError::UnexpectedToken(token.clone()));
            },
        };

        stmts.push(stmt);
    }

    Ok(stmts)
}


