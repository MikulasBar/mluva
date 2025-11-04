use std::str::FromStr as _;

use super::token::{Token, TokenKind};
use super::DataType;
use crate::ast::*;
use crate::diagnostics::Span;
use crate::errors::CompileError;
use crate::expect_token;
use crate::function::InternalFunctionSigniture;
use crate::value::Value;

pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
    ast: Ast,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            index: 0,
            ast: Ast::empty(),
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

    fn span_between(&self, first_idx: usize, last_idx: usize) -> crate::diagnostics::Span {
        let first = self.tokens[first_idx].span;
        let last = self.tokens[last_idx].span;
        first.join(last)
    }

    fn span_current(&self) -> Option<crate::diagnostics::Span> {
        self.peek().map(|t| t.span)
    }

    pub fn parse(mut self) -> Result<Ast, CompileError> {
        self.parse_top_level()?;
        Ok(self.ast)
    }

    fn parse_top_level(&mut self) -> Result<(), CompileError> {
        while let Some(token) = self.next() {
            match token.kind {
                TokenKind::EOL => {
                    self.skip();
                    continue;
                }

                TokenKind::DataType(_) => {
                    expect_token!(TokenKind::DataType(return_type) in self);
                    expect_token!(TokenKind::Ident(name), name_span in self);
                    expect_token!(TokenKind::ParenL in self);

                    let params = self.parse_named_parameters()?;

                    expect_token!(TokenKind::ParenR in self);
                    expect_token!(TokenKind::BraceL in self);

                    if BuiltinFunction::str_variants().contains(name.as_str()) {
                        return Err(CompileError::reserved_function_name_at(name, name_span));
                    }

                    let (body, _) = self.parse_statements(TokenKind::BraceR)?;
                    let signiture = InternalFunctionSigniture::new(return_type, params);
                    self.ast.add_function(name, signiture, body);
                }

                TokenKind::Import => {
                    expect_token!(TokenKind::Import in self);
                    expect_token!(TokenKind::Ident(module_name) in self);
                    expect_token!(TokenKind::EOL in self);

                    let import_path = Path::single(module_name);
                    self.ast.add_import(import_path);
                }

                _ => return Err(CompileError::unexpected_token_at(token.kind, token.span)),
            }
        }

        Ok(())
    }

    fn parse_named_parameters(&mut self) -> Result<Vec<(String, DataType)>, CompileError> {
        let mut params = vec![];
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::ParenR {
                break;
            }

            expect_token!(TokenKind::DataType(data_type) in self);
            expect_token!(TokenKind::Ident(ident) in self);
            params.push((ident, data_type));

            if let Some(&TokenKind::Comma) = self.peek().map(|t| &t.kind) {
                self.skip();
            } else {
                break;
            }
        }

        Ok(params)
    }

    /// Parses a list of statements until the critical token is found.
    /// The critical token is not included in the returned statements.
    fn parse_statements(
        &mut self,
        critical_kind: TokenKind,
    ) -> Result<(Vec<Statement>, Span), CompileError> {
        let mut statements = vec![];
        let mut end_span: Option<Span> = None;

        while let Some(token) = self.peek() {
            if token.kind == critical_kind {
                self.skip();
                end_span = Some(token.span);
                break;
            }

            let statement = match token.kind {
                // lonely EOL -> skip
                TokenKind::EOL => {
                    self.skip();
                    continue;
                }

                TokenKind::Return => {
                    expect_token!(TokenKind::Return in self);
                    if let Some(TokenKind::EOL) = self.peek() {
                        self.skip();
                        Statement::return_statement(Expr::literal(Value::Void))
                    } else {
                        let expr = self.parse_expr()?;
                        expect_token!(TokenKind::EOL in self);
                        Statement::Return(expr)
                    }
                }

                // var declaration with explicit type
                TokenKind::DataType(_) => {
                    expect_token!(TokenKind::DataType(data_type) in self);
                    expect_token!(TokenKind::Ident(ident) in self);
                    expect_token!(TokenKind::Assign in self);

                    let expr = self.parse_expr()?;

                    expect_token!(TokenKind::EOL in self);

                    Statement::var_declare(Some(data_type), ident, expr)
                }

                TokenKind::Let => {
                    expect_token!(TokenKind::Let in self);
                    expect_token!(TokenKind::Ident(ident) in self);
                    expect_token!(TokenKind::Assign in self);

                    let expr = self.parse_expr()?;

                    expect_token!(TokenKind::EOL in self);

                    Statement::var_declare(None, ident, expr)
                }

                // var assign / function call in expr stmt
                TokenKind::Ident(_) => self.parse_ident_statement()?,

                TokenKind::If => self.parse_if_statement()?,

                TokenKind::While => {
                    expect_token!(TokenKind::While in self);

                    let cond = self.parse_expr()?;

                    expect_token!(TokenKind::BraceL in self);

                    let stmts = self.parse_stmts(TokenKind::BraceR)?;
                    Statement::while_statement(cond, stmts)
                }

                _ => Statement::expr_statement(self.parse_expr()?),
            };

            statements.push(statement);
        }

        // ensure we have an end span to return (critical token must have been found)
        let end = end_span.unwrap_or_else(|| Span::new(0, 0, 0));
        Ok((statements, end))
    }

    fn parse_ident_statement(&mut self) -> Result<Statement, CompileError> {
        expect_token!(TokenKind::Ident(ident) in self);

        if let Some(TokenKind::Assign) = self.peek().map(|t| t.kind) {
            expect_token!(TokenKind::Assign in self);

            let expr = self.parse_expr()?;

            expect_token!(TokenKind::EOL in self);
            Ok(Statement::var_assign(ident, expr))
        } else {
            // if the next token is not an assign, it must be a function call
            // so we need to backtrack the ident token
            // and parse it as function call
            self.back();
            let expr = self.parse_expr()?;
            expect_token!(TokenKind::EOL in self);

            Ok(Statement::Expr(expr))
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, CompileError> {
        expect_token!(TokenKind::If in self);

        let cond = self.parse_expr()?;

        expect_token!(TokenKind::BraceL in self);

        let stmts = self.parse_stmts(TokenKind::BraceR)?;

        let else_branch = if let Some(TokenKind::Else) = self.peek() {
            expect_token!(TokenKind::Else in self);

            if let Some(TokenKind::If) = self.peek() {
                Some(vec![self.parse_if_statement()?])
            } else {
                expect_token!(TokenKind::BraceL in self);
                let if_block = self.parse_stmts(TokenKind::BraceR)?;
                Some(if_block)
            }
        } else {
            None
        };

        Ok(Statement::if_statement(cond, if_block, else_block))
    }

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
        let token = self.peek().ok_or(CompileError::UnexpectedEndOfFile)?;

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
            return Err(CompileError::UnexpectedEndOfFile);
        };

        match token {
            TokenKind::Bool(_) => {
                expect_token!(TokenKind::Bool(bool) in self);
                Ok(Expr::Literal(Value::Bool(bool)))
            }

            TokenKind::Int(_) => {
                expect_token!(TokenKind::Int(int) in self);
                Ok(Expr::Literal(Value::Int(int)))
            }

            TokenKind::Float(_) => {
                expect_token!(TokenKind::Float(float) in self);
                Ok(Expr::Literal(Value::Float(float)))
            }

            TokenKind::StringLiteral(_) => {
                expect_token!(TokenKind::StringLiteral(string) in self);
                Ok(Expr::Literal(Value::String(string)))
            }

            TokenKind::Ident(_) => self.parse_ident_expr(),

            TokenKind::ParenL => {
                expect_token!(TokenKind::ParenL in self);
                let inner = self.parse_expr();
                expect_token!(TokenKind::ParenR in self);
                inner
            }

            _ => {
                return Err(CompileError::UnexpectedToken(self.next().unwrap()));
            }
        }
    }

    fn parse_ident_expr(&mut self) -> Result<Expr, CompileError> {
        expect_token!(TokenKind::Ident(ident) in self);

        match self.peek() {
            Some(TokenKind::ParenL) => {
                expect_token!(TokenKind::ParenL in self);
                let args = self.parse_args()?;
                expect_token!(TokenKind::ParenR in self);

                if BuiltinFunction::str_variants().contains(ident.as_str()) {
                    return Ok(Expr::BuiltinFunctionCall {
                        function: BuiltinFunction::from_str(&ident).unwrap(),
                        args,
                    });
                }

                Ok(Expr::FunctionCall(ident, args))
            }

            Some(TokenKind::Colon) => {
                expect_token!(TokenKind::Colon in self);
                expect_token!(TokenKind::Ident(func_name) in self);

                expect_token!(TokenKind::ParenL in self);
                let args = self.parse_args()?;
                expect_token!(TokenKind::ParenR in self);

                Ok(Expr::ForeignFunctionCall {
                    module_name: ident,
                    func_name,
                    args,
                })
            }

            _ => Ok(Expr::Var(ident)),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, CompileError> {
        let mut args = vec![];

        while let Some(token) = self.peek() {
            if token == &TokenKind::ParenR {
                break;
            }

            args.push(self.parse_expr()?);

            if self.peek() == Some(&TokenKind::Comma) {
                self.skip();
            } else {
                break;
            }
        }

        Ok(args)
    }
}

fn token_to_logical_op(token: &TokenKind) -> Option<BinaryOp> {
    match token {
        TokenKind::And => Some(BinaryOp::And),
        TokenKind::Or => Some(BinaryOp::Or),
        _ => None,
    }
}

fn token_to_comp_op(token: &TokenKind) -> Option<BinaryOp> {
    match token {
        TokenKind::Equal => Some(BinaryOp::Equal),
        TokenKind::NotEqual => Some(BinaryOp::NotEqual),
        TokenKind::Less => Some(BinaryOp::Less),
        TokenKind::LessEqual => Some(BinaryOp::LessEqual),
        TokenKind::Greater => Some(BinaryOp::Greater),
        TokenKind::GreaterEqual => Some(BinaryOp::GreaterEqual),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_expr() {
        let tokens = vec![
            TokenKind::Int(1),
            TokenKind::Plus,
            TokenKind::Int(2),
            TokenKind::Asterisk,
            TokenKind::Int(3),
            TokenKind::Equal,
            TokenKind::Ident("x".to_string()),
            TokenKind::ParenL,
            TokenKind::Int(7),
            TokenKind::Comma,
            TokenKind::Int(8),
            TokenKind::ParenR,
        ];

        let mut parser = Parser::new(&tokens);
        let expr = parser.parse_expr().unwrap();

        let expected_expr = Expr::new_binary_op(
            BinaryOp::Equal,
            Expr::new_binary_op(
                BinaryOp::Add,
                Expr::Literal(Value::Int(1)),
                Expr::new_binary_op(
                    BinaryOp::Mul,
                    Expr::Literal(Value::Int(2)),
                    Expr::Literal(Value::Int(3)),
                ),
            ),
            Expr::FunctionCall(
                "x".to_string(),
                vec![Expr::Literal(Value::Int(7)), Expr::Literal(Value::Int(8))],
            ),
        );

        assert_eq!(expr, expected_expr);
    }

    #[test]
    fn parse_args() {
        let tokens = vec![
            TokenKind::Int(1),
            TokenKind::Comma,
            TokenKind::Int(2),
            TokenKind::Comma,
            TokenKind::Int(3),
            TokenKind::ParenR,
        ];

        let mut parser = Parser::new(&tokens);
        let args = parser.parse_args().unwrap();
        let expected_args = vec![
            Expr::Literal(Value::Int(1)),
            Expr::Literal(Value::Int(2)),
            Expr::Literal(Value::Int(3)),
        ];

        assert_eq!(args, expected_args);
    }
}
