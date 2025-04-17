use crate::ast::*;
use crate::errors::CompileError;
use crate::function::{ExternalFunctionDefinition, InternalFunctionDefinition};
use super::token::Token;
use super::DataType;
use crate::expect_pat;
use crate::value::Value;


pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            index: 0,
        }
    }

    /// Returns the next token and advances the index by one.
    /// If there are no more tokens, it returns None.
    fn next(&mut self) -> Option<Token> {
        if self.index < self.tokens.len() {
            let token = self.tokens[self.index].clone();
           self.index += 1;
            Some(token)
        } else {
            None
        }
    }

    /// Advances the index by one, but does not return the token.
    /// This is useful for skipping over tokens that are not needed.
    fn skip(&mut self) {
        if self.index < self.tokens.len() {
           self.index += 1;
        }
    }

    /// Returns the current token as ref without advancing the index.
    fn peek(&self) -> Option<&Token> {
        if self.index < self.tokens.len() {
            Some(&self.tokens[self.index])
        } else {
            None
        }
    }

    /// Shift the index back by one, but does not return the token.
    fn back(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }


    pub fn parse(&mut self) -> Result<Vec<Item>, CompileError> {
       self.parse_items()
    }

    fn parse_items(&mut self) -> Result<Vec<Item>, CompileError> {
        let mut items = vec![];

        while let Some(token) = self.peek() {
            match token {
                // lonely EOL
                Token::EOL => {
                    self.skip();
                    continue;
                },

                Token::DataType(_) => {
                    expect_pat!(Token::DataType(return_type) in self);
                    expect_pat!(Token::Ident(fn_name) in self);
                    expect_pat!(Token::ParenL in self);

                    let params = self.parse_named_parameters()?;

                    expect_pat!(Token::ParenR in self);
                    expect_pat!(Token::BraceL in self);

                    let stmts = self.parse_stmts(Token::BraceR)?;
                    items.push(Item::FunctionDef(
                        InternalFunctionDefinition::new(fn_name, return_type, params, stmts)
                    ));
                },

                Token::External => {
                    expect_pat!(Token::External in self);
                    expect_pat!(Token::DataType(return_type) in self);
                    expect_pat!(Token::Ident(fn_name) in self);
                    expect_pat!(Token::ParenL in self);

                    let params = self.parse_unnamed_parameters()?;

                    expect_pat!(Token::ParenR in self);
                    expect_pat!(Token::EOL in self);

                    items.push(Item::ExternalFunctionDef(
                        ExternalFunctionDefinition::new(fn_name, return_type, params)
                    ));
                }

                _ => return Err(CompileError::UnexpectedToken(token.clone())),
            }
        }

        Ok(items)
    }

    fn parse_unnamed_parameters(&mut self) -> Result<Vec<DataType>, CompileError> {
        let mut params = vec![];
        while let Some(token) = self.peek() {
            if token == &Token::ParenR {
                break;
            }

            expect_pat!(Token::DataType(data_type) in self);
            params.push(data_type);

            if self.peek() == Some(&Token::Comma) {
               self.skip();
            } else {
                break;
            }
        }

        Ok(params)
    }

    fn parse_named_parameters(&mut self) -> Result<Vec<(String, DataType)>, CompileError> {
        let mut params = vec![];
        while let Some(token) = self.peek() {
            if token == &Token::ParenR {
                break;
            }

            expect_pat!(Token::DataType(data_type) in self);
            expect_pat!(Token::Ident(ident) in self);
            params.push((ident, data_type));

            if self.peek() == Some(&Token::Comma) {
               self.skip();
            } else {
                break;
            }
        }

        Ok(params)
    }

    /// Parses a list of statements until the critical token is found.
    /// The critical token is not included in the returned statements.
    fn parse_stmts(&mut self, critical_token: Token) -> Result<Vec<Stmt>, CompileError> {
        let mut stmts = vec![];

        while let Some(token) = self.peek() {
            if *token == critical_token {
               self.skip();
                break;
            }

            let stmt = match token {
                // lonely EOL
                Token::EOL => {
                   self.skip();
                    continue;
                },

                Token::Return => {
                    expect_pat!(Token::Return in self);
                    if let Some(Token::EOL) = self.peek() {
                        self.skip();
                        Stmt::Return(Expr::Literal(Value::Void))
                    } else {
                        let expr = self.parse_expr()?;
                        expect_pat!(Token::EOL in self);
                        Stmt::Return(expr)
                    }
                }

                // var declaration with explicit type
                Token::DataType(_) => {
                    expect_pat!(Token::DataType(data_type) in self);
                    expect_pat!(Token::Ident(ident) in self);
                    expect_pat!(Token::Assign in self);

                    let expr = self.parse_expr()?;

                    expect_pat!(Token::EOL in self);

                    Stmt::VarDeclare(Some(data_type), ident, expr)
                },

                Token::Let => {
                    expect_pat!(Token::Let in self);
                    expect_pat!(Token::Ident(ident) in self);
                    expect_pat!(Token::Assign in self);

                    let expr = self.parse_expr()?;

                    expect_pat!(Token::EOL in self);

                    Stmt::VarDeclare(None, ident, expr)
                }

                // var assign / function call in expr stmt
                Token::Ident(_) => {
                   self.parse_ident_statement()?
                },

                Token::If => self.parse_if_statement()?,

                Token::While => {
                    expect_pat!(Token::While in self);

                    let cond = self.parse_expr()?;

                    expect_pat!(Token::BraceL in self);

                    let stmts = self.parse_stmts(Token::BraceR)?;
                    Stmt::While(cond, stmts)
                },

                _ => {
                    Stmt::Expr(self.parse_expr()?)
                },
            };

            stmts.push(stmt);
        }

        Ok(stmts)
    }

    fn parse_ident_statement(&mut self) -> Result<Stmt, CompileError> {
        expect_pat!(Token::Ident(ident) in self);

        if let Some(Token::Assign) = self.peek() {
            expect_pat!(Token::Assign in self);

            let expr = self.parse_expr()?;

            expect_pat!(Token::EOL in self);
            Ok(Stmt::VarAssign(ident, expr))
        } else {
            // if the next token is not an assign, it must be a function call
            // so we need to backtrack the ident token
            // and parse it as function call
            self.back();
            let expr = self.parse_expr()?;
            expect_pat!(Token::EOL in self);

            Ok(Stmt::Expr(expr))
        }
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, CompileError> {
        expect_pat!(Token::If in self);

        let cond = self.parse_expr()?;

        expect_pat!(Token::BraceL in self);

        let stmts = self.parse_stmts(Token::BraceR)?;

        let else_branch = if let Some(Token::Else) = self.peek() {
            expect_pat!(Token::Else in self);
        
            if let Some(Token::If) = self.peek() {
                Some(vec![self.parse_if_statement()?])
            } else {
                expect_pat!(Token::BraceL in self);
                let stmts = self.parse_stmts(Token::BraceR)?;
                Some(stmts)
            }
        } else {
            None
        };

        Ok(Stmt::If(cond, stmts, else_branch))
    }


    ////////////////// Expression parsing methods ////////////////

    fn parse_expr(&mut self) -> Result<Expr, CompileError> {
       self.parse_logical_expr()
    }

    /// Parse logical `BinaryOp` such as and, or
    fn parse_logical_expr(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_comp_expr()?;

        while let Some(token) = self.peek() {
            let Some(op) = token_to_logical_op(token) else {
                return Ok(lhs);
            };

            self.skip();
            let rhs = self.parse_comp_expr()?;
            lhs = Expr::new_binary_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// Parse eq and neq `BinaryOp`
    fn parse_comp_expr(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_add_expr()?;

        if let Some(token) = self.peek() {
            let Some(op) = token_to_comp_op(token) else {
                return Ok(lhs);
            };

           self.skip();
            let rhs = self.parse_add_expr()?;
            lhs = Expr::new_binary_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// Parse add and subtract `BinaryOp`
    fn parse_add_expr(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_mul_expr()?;

        while let Some(token) = self.peek() {
            let Some(op) = token_to_add_op(token) else {
                return Ok(lhs);
            };

           self.skip();
            let rhs = self.parse_mul_expr()?;
            lhs = Expr::new_binary_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// Parse multiply, divide and modulo `BinaryOp`
    fn parse_mul_expr(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_unary_op_expr()?;

        while let Some(token) = self.peek() {
            let Some(op) = token_to_mul_op(token) else {
                return Ok(lhs);
            };

           self.skip();
            let rhs = self.parse_unary_op_expr()?;
            lhs = Expr::new_binary_op(op, lhs, rhs);
        }

        Ok(lhs)
    }

    /// Parse unary `UnaryOp` such as not
    fn parse_unary_op_expr(&mut self) -> Result<Expr, CompileError> {
        let token = self.peek().ok_or(CompileError::UnexpectedEndOfInput)?;

        let Some(op) = token_to_unary_op(token) else {
            return self.parse_atom_expr();
        };

        self.skip();
        let expr = self.parse_unary_op_expr()?;
        return Ok(Expr::new_unary_op(op, expr));
    }

    /// Parse atom expr such as Ident, Num, Bool, not ops.
    fn parse_atom_expr(&mut self) -> Result<Expr, CompileError> {
        let Some(token) = self.peek() else {
            return Err(CompileError::UnexpectedEndOfInput);
        };

        match token {
            Token::Bool(_) => {
                expect_pat!(Token::Bool(bool) in self);
                Ok(Expr::Literal(Value::Bool(bool)))
            }

            Token::Int(_) => {
                expect_pat!(Token::Int(int) in self);
                Ok(Expr::Literal(Value::Int(int)))
            }

            Token::Float(_) => {
                expect_pat!(Token::Float(float) in self);
                Ok(Expr::Literal(Value::Float(float)))
            }

            Token::StringLiteral(_) => {
                expect_pat!(Token::StringLiteral(string) in self);
                Ok(Expr::Literal(Value::String(string)))
            }

            Token::Ident(_) => {
               self.parse_ident_expr()
            }

            Token::ParenL => {
                expect_pat!(Token::ParenL in self);
                let inner = self.parse_expr();
                expect_pat!(Token::ParenR in self);
                inner
            }

            _ => {
                return Err(CompileError::UnexpectedToken(self.next().unwrap()));
            }
        }
    }

    fn parse_ident_expr(&mut self) -> Result<Expr, CompileError> {
        expect_pat!(Token::Ident(ident) in self);

        if let Some(Token::ParenL) = self.peek() {
            expect_pat!(Token::ParenL in self);
            let mut args = Vec::new();

            while let Some(token) = self.peek() {
                if token == &Token::ParenR {
                    break;
                }

                args.push(self.parse_expr()?);

                if self.peek() == Some(&Token::Comma) {
                   self.skip();
                } else {
                    break;
                }
            }

            expect_pat!(Token::ParenR in self);
            Ok(Expr::FuncCall(ident, args))
        } else {
            Ok(Expr::Var(ident))
        }
    }
}


fn token_to_logical_op(token: &Token) -> Option<BinaryOp> {
    match token {
        Token::And => Some(BinaryOp::And),
        Token::Or => Some(BinaryOp::Or),
        _ => None,
    }
}

fn token_to_comp_op(token: &Token) -> Option<BinaryOp> {
    match token {
        Token::Equal => Some(BinaryOp::Equal),
        Token::NotEqual => Some(BinaryOp::NotEqual),
        Token::Less => Some(BinaryOp::Less),
        Token::LessEqual => Some(BinaryOp::LessEqual),
        Token::Greater => Some(BinaryOp::Greater),
        Token::GreaterEqual => Some(BinaryOp::GreaterEqual),
        _ => None,
    }
}

fn token_to_add_op(token: &Token) -> Option<BinaryOp> {
    match token {
        Token::Plus => Some(BinaryOp::Add),
        Token::Minus => Some(BinaryOp::Sub),
        _ => None,
    }
}

fn token_to_mul_op(token: &Token) -> Option<BinaryOp> {
    match token {
        Token::Asterisk => Some(BinaryOp::Mul),
        Token::Slash => Some(BinaryOp::Div),
        Token::Modulo => Some(BinaryOp::Modulo),
        _ => None,
    }
}

fn token_to_unary_op(token: &Token) -> Option<UnaryOp> {
    match token {
        Token::Not => Some(UnaryOp::Not),
        Token::Minus => Some(UnaryOp::Negate),
        _ => None,
    }
}