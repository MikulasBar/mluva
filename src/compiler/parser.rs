use std::str::FromStr as _;

use super::token::{Token, TokenKind};
use super::DataType;
use crate::ast::*;
use crate::diagnostics::FileId;
use crate::errors::CompileError;
use crate::expect_token;
use crate::function::InternalFunctionSigniture;
use crate::value::Value;

pub struct Parser<'a> {
    file_id: FileId,
    tokens: &'a [Token],
    index: usize,
    ast: Ast,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], file_id: FileId) -> Self {
        Self {
            file_id,
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

    /// Returns the current token kind as ref without advancing the index.
    fn peek_kind(&self) -> Option<&TokenKind> {
        self.peek().map(|t| &t.kind)
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

                    let body = self.parse_statements(TokenKind::BraceR)?;
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

            if let Some(&TokenKind::Comma) = self.peek_kind() {
                self.skip();
            } else {
                break;
            }
        }

        Ok(params)
    }

    /// Parses a list of statements until the critical token is found.
    /// The critical token is not consumed.
    fn parse_statements(
        &mut self,
        critical_kind: TokenKind,
    ) -> Result<Vec<Statement>, CompileError> {
        let mut statements = vec![];

        while let Some(token) = self.peek() {
            if token.kind == critical_kind {
                break;
            }

            let token_span = token.span;
            let statement = match token.kind {
                // lonely EOL -> skip
                TokenKind::EOL => {
                    self.skip();
                    continue;
                }

                TokenKind::Return => {
                    expect_token!(TokenKind::Return in self);
                    if let Some(TokenKind::EOL) = self.peek_kind() {
                        self.skip();
                        Statement::return_statement(
                            Expr::literal(Value::Void, token_span),
                            token_span,
                        )
                    } else {
                        let expr = self.parse_expr()?;
                        expect_token!(TokenKind::EOL in self);
                        Statement::return_statement(expr, token_span)
                    }
                }

                // var declaration with explicit type
                TokenKind::DataType(_) => {
                    expect_token!(TokenKind::DataType(data_type) in self);
                    expect_token!(TokenKind::Ident(ident) in self);
                    expect_token!(TokenKind::Assign in self);

                    let expr = self.parse_expr()?;

                    expect_token!(TokenKind::EOL in self);

                    let expr_span = expr.span;
                    Statement::var_declare(Some(data_type), ident, expr, token_span.join(expr_span))
                }

                TokenKind::Let => {
                    expect_token!(TokenKind::Let in self);
                    expect_token!(TokenKind::Ident(ident) in self);
                    expect_token!(TokenKind::Assign in self);

                    let expr = self.parse_expr()?;

                    expect_token!(TokenKind::EOL in self);

                    let expr_span = expr.span;
                    Statement::var_declare(None, ident, expr, token_span.join(expr_span))
                }

                // var assign / function call in expr stmt
                TokenKind::Ident(_) => self.parse_ident_statement()?,

                TokenKind::If => self.parse_if_statement()?,

                TokenKind::While => {
                    expect_token!(TokenKind::While in self);

                    let cond = self.parse_expr()?;

                    expect_token!(TokenKind::BraceL in self);

                    let stmts = self.parse_statements(TokenKind::BraceR)?;

                    expect_token!(TokenKind::BraceR, brace_r_span in self);

                    Statement::while_statement(cond, stmts, token_span.join(brace_r_span))
                }

                _ => {
                    let expr = self.parse_expr()?;
                    expect_token!(TokenKind::EOL in self);
                    let span = expr.span;
                    Statement::expr_statement(expr, span)
                }
            };

            statements.push(statement);
        }

        Ok(statements)
    }

    fn parse_ident_statement(&mut self) -> Result<Statement, CompileError> {
        expect_token!(TokenKind::Ident(ident) in self);

        if let Some(TokenKind::Assign) = self.peek_kind() {
            expect_token!(TokenKind::Assign, assign_span in self);

            let expr = self.parse_expr()?;

            expect_token!(TokenKind::EOL in self);

            let expr_span = expr.span;
            Ok(Statement::var_assign(
                ident,
                expr,
                assign_span.join(expr_span),
            ))
        } else {
            // if the next token is not an assign, it must be a function call
            // so we need to backtrack the ident token
            // and parse it as function call
            self.back();
            let expr = self.parse_expr()?;
            expect_token!(TokenKind::EOL in self);

            let expr_span = expr.span;

            Ok(Statement::expr_statement(expr, expr_span))
        }
    }

    fn parse_if_statement(&mut self) -> Result<Statement, CompileError> {
        expect_token!(TokenKind::If, if_span in self);

        let cond = self.parse_expr()?;

        expect_token!(TokenKind::BraceL in self);

        let if_block = self.parse_statements(TokenKind::BraceR)?;
        expect_token!(TokenKind::BraceR, brace_r_span in self);

        let (else_block, end_span) = if let Some(TokenKind::Else) = self.peek_kind() {
            expect_token!(TokenKind::Else in self);

            if let Some(TokenKind::If) = self.peek_kind() {
                let else_if_stmt = self.parse_if_statement()?;
                let end_span = else_if_stmt.span;
                (Some(vec![else_if_stmt]), end_span)
            } else {
                expect_token!(TokenKind::BraceL in self);
                let if_block = self.parse_statements(TokenKind::BraceR)?;
                expect_token!(TokenKind::BraceR, brace_r_span in self);
                (Some(if_block), brace_r_span)
            }
        } else {
            (None, brace_r_span)
        };

        Ok(Statement::if_statement(
            cond,
            if_block,
            else_block,
            if_span.join(end_span),
        ))
    }

    fn parse_expr(&mut self) -> Result<Expr, CompileError> {
        self.parse_logical_expr()
    }

    /// Parse logical `BinaryOp` such as and, or
    fn parse_logical_expr(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_comp_expr()?;

        while let Some(token) = self.peek() {
            let Some(op) = token_to_logical_op(&token.kind) else {
                return Ok(lhs);
            };

            self.skip();
            let rhs = self.parse_comp_expr()?;
            let lhs_span = lhs.span;
            let rhs_span = rhs.span;
            lhs = Expr::binary_op(op, lhs, rhs, lhs_span.join(rhs_span));
        }

        Ok(lhs)
    }

    /// Parse eq and neq `BinaryOp`
    fn parse_comp_expr(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_add_expr()?;

        if let Some(token) = self.peek() {
            let Some(op) = token_to_comp_op(&token.kind) else {
                return Ok(lhs);
            };

            self.skip();
            let rhs = self.parse_add_expr()?;
            let lhs_span = lhs.span;
            let rhs_span = rhs.span;
            lhs = Expr::binary_op(op, lhs, rhs, lhs_span.join(rhs_span));
        }

        Ok(lhs)
    }

    /// Parse add and subtract `BinaryOp`
    fn parse_add_expr(&mut self) -> Result<Expr, CompileError> {
        let mut lhs = self.parse_mul_expr()?;

        while let Some(token) = self.peek() {
            let Some(op) = token_to_add_op(&token.kind) else {
                return Ok(lhs);
            };

            self.skip();
            let rhs = self.parse_mul_expr()?;
            let lhs_span = lhs.span;
            let rhs_span = rhs.span;
            lhs = Expr::binary_op(op, lhs, rhs, lhs_span.join(rhs_span));
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
            let lhs_span = lhs.span;
            let rhs_span = rhs.span;
            lhs = Expr::binary_op(op, lhs, rhs, lhs_span.join(rhs_span));
        }

        Ok(lhs)
    }

    /// Parse unary `UnaryOp` such as not
    fn parse_unary_op_expr(&mut self) -> Result<Expr, CompileError> {
        let token = self
            .peek()
            .ok_or(CompileError::unexpected_end_of_file(self.file_id))?;

        let Some(op) = token_to_unary_op(token) else {
            return self.parse_atom_expr();
        };

        let token_span = token.span;
        self.skip();
        let expr = self.parse_unary_op_expr()?;
        let expr_span = expr.span;
        return Ok(Expr::unary_op(op, expr, token_span.join(expr_span)));
    }

    /// Parse atom expr such as Ident, Num, Bool, not ops.
    fn parse_atom_expr(&mut self) -> Result<Expr, CompileError> {
        let Some(token) = self.peek() else {
            return Err(CompileError::unexpected_end_of_file(self.file_id));
        };

        let token_span = token.span;

        match token.kind {
            TokenKind::Bool(_) => {
                expect_token!(TokenKind::Bool(bool) in self);
                Ok(Expr::literal(Value::Bool(bool), token_span))
            }

            TokenKind::Int(_) => {
                expect_token!(TokenKind::Int(int) in self);
                Ok(Expr::literal(Value::Int(int), token_span))
            }

            TokenKind::Float(_) => {
                expect_token!(TokenKind::Float(float) in self);
                Ok(Expr::literal(Value::Float(float), token_span))
            }

            TokenKind::StringLiteral(_) => {
                expect_token!(TokenKind::StringLiteral(string) in self);
                Ok(Expr::literal(Value::String(string), token_span))
            }

            TokenKind::Ident(_) => self.parse_ident_expr(),

            TokenKind::ParenL => {
                expect_token!(TokenKind::ParenL in self);
                let inner = self.parse_expr();
                expect_token!(TokenKind::ParenR in self);
                inner
            }

            _ => {
                return Err(CompileError::unexpected_token_at(
                    self.next().unwrap().kind,
                    token_span,
                ));
            }
        }
    }

    fn parse_ident_expr(&mut self) -> Result<Expr, CompileError> {
        expect_token!(TokenKind::Ident(ident), ident_span in self);

        match self.peek_kind() {
            Some(TokenKind::ParenL) => {
                expect_token!(TokenKind::ParenL in self);
                let args = self.parse_args()?;
                expect_token!(TokenKind::ParenR, end_span in self);

                if let Ok(builtin_function) = BuiltinFunction::from_str(ident.as_str()) {
                    return Ok(Expr::builtin_function_call(
                        builtin_function,
                        args,
                        ident_span.join(end_span),
                    ));
                }

                Ok(Expr::function_call(ident, args, ident_span.join(end_span)))
            }

            Some(TokenKind::Colon) => {
                expect_token!(TokenKind::Colon in self);
                expect_token!(TokenKind::Ident(func_name) in self);

                expect_token!(TokenKind::ParenL in self);
                let args = self.parse_args()?;
                expect_token!(TokenKind::ParenR, end_span in self);

                Ok(Expr::foreign_function_call(
                    ident,
                    func_name,
                    args,
                    ident_span.join(end_span),
                ))
            }

            _ => Ok(Expr::var(ident, ident_span)),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, CompileError> {
        let mut args = vec![];

        while let Some(token) = self.peek() {
            if token.kind == TokenKind::ParenR {
                break;
            }

            args.push(self.parse_expr()?);

            if let Some(&TokenKind::Comma) = self.peek_kind() {
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

fn token_to_add_op(token: &TokenKind) -> Option<BinaryOp> {
    match token {
        TokenKind::Plus => Some(BinaryOp::Add),
        TokenKind::Minus => Some(BinaryOp::Sub),
        _ => None,
    }
}

fn token_to_mul_op(token: &Token) -> Option<BinaryOp> {
    match &token.kind {
        TokenKind::Asterisk => Some(BinaryOp::Mul),
        TokenKind::Slash => Some(BinaryOp::Div),
        TokenKind::Modulo => Some(BinaryOp::Modulo),
        _ => None,
    }
}

fn token_to_unary_op(token: &Token) -> Option<UnaryOp> {
    match &token.kind {
        TokenKind::Not => Some(UnaryOp::Not),
        TokenKind::Minus => Some(UnaryOp::Negate),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ast::ExprKind;
    use crate::diagnostics::{FileId, Span};
    const TEST_FILE_ID: FileId = 0;

    fn create_token(kind: TokenKind, start: usize, end: usize) -> Token {
        Token {
            kind,
            span: Span::new(TEST_FILE_ID, start, end),
        }
    }

    fn create_parser<'a>(tokens: &'a [Token]) -> Parser<'a> {
        Parser::new(&tokens, TEST_FILE_ID)
    }

    #[test]
    fn parse_simple_arithmetic() {
        let tokens = vec![
            create_token(TokenKind::Int(1), 0, 1),
            create_token(TokenKind::Plus, 2, 3),
            create_token(TokenKind::Int(2), 4, 5),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        // Check that we get a binary operation
        match expr.kind {
            ExprKind::BinaryOp(BinaryOp::Add, ..) => (),
            _ => panic!("Expected binary add operation"),
        }
    }

    #[test]
    fn parse_operator_precedence() {
        let tokens = vec![
            create_token(TokenKind::Int(1), 0, 1),
            create_token(TokenKind::Plus, 2, 3),
            create_token(TokenKind::Int(2), 4, 5),
            create_token(TokenKind::Asterisk, 6, 7),
            create_token(TokenKind::Int(3), 8, 9),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        // Should parse as 1 + (2 * 3)
        if let ExprKind::BinaryOp(_, lhs, rhs) = &expr.kind {
            if let ExprKind::Literal(Value::Int(1)) = &lhs.kind {
                if let ExprKind::BinaryOp(_, ..) = &rhs.kind {
                    return; // Correct precedence
                }
            }
        }
        panic!("Operator precedence not handled correctly");
    }

    #[test]
    fn parse_comparison() {
        let tokens = vec![
            create_token(TokenKind::Int(5), 0, 1),
            create_token(TokenKind::Equal, 2, 4),
            create_token(TokenKind::Int(5), 5, 6),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        match expr.kind {
            ExprKind::BinaryOp(BinaryOp::Equal, ..) => (),
            _ => panic!("Expected equality comparison"),
        }
    }

    #[test]
    fn parse_logical_operations() {
        let tokens = vec![
            create_token(TokenKind::Bool(true), 0, 4),
            create_token(TokenKind::And, 5, 8),
            create_token(TokenKind::Bool(false), 9, 14),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        match expr.kind {
            ExprKind::BinaryOp(BinaryOp::And, ..) => (),
            _ => panic!("Expected logical AND operation"),
        }
    }

    #[test]
    fn parse_unary_operations() {
        let tokens = vec![
            create_token(TokenKind::Not, 0, 3),
            create_token(TokenKind::Bool(true), 4, 8),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        match expr.kind {
            ExprKind::UnaryOp(UnaryOp::Not, ..) => (),
            _ => panic!("Expected unary NOT operation"),
        }
    }

    #[test]
    fn parse_parentheses() {
        let tokens = vec![
            create_token(TokenKind::ParenL, 0, 1),
            create_token(TokenKind::Int(42), 1, 3),
            create_token(TokenKind::ParenR, 3, 4),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        match expr.kind {
            ExprKind::Literal(Value::Int(42)) => (),
            _ => panic!("Expected literal value 42"),
        }
    }

    #[test]
    fn parse_function_call() {
        let tokens = vec![
            create_token(TokenKind::Ident("foo".to_string()), 0, 3),
            create_token(TokenKind::ParenL, 3, 4),
            create_token(TokenKind::Int(1), 4, 5),
            create_token(TokenKind::Comma, 5, 6),
            create_token(TokenKind::Int(2), 7, 8),
            create_token(TokenKind::ParenR, 8, 9),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        if let ExprKind::FunctionCall { func_name, args } = &expr.kind {
            assert_eq!(func_name, "foo");
            assert_eq!(args.len(), 2);
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn parse_variable_reference() {
        let tokens = vec![create_token(TokenKind::Ident("variable".to_string()), 0, 8)];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        if let ExprKind::Var(name) = &expr.kind {
            assert_eq!(name, "variable");
        } else {
            panic!("Expected variable reference");
        }
    }

    #[test]
    fn parse_foreign_function_call() {
        let tokens = vec![
            create_token(TokenKind::Ident("module".to_string()), 0, 6),
            create_token(TokenKind::Colon, 6, 7),
            create_token(TokenKind::Ident("func".to_string()), 7, 11),
            create_token(TokenKind::ParenL, 11, 12),
            create_token(TokenKind::ParenR, 12, 13),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        if let ExprKind::ForeignFunctionCall {
            module_name,
            func_name,
            ..
        } = &expr.kind
        {
            assert_eq!(module_name, "module");
            assert_eq!(func_name, "func");
        } else {
            panic!("Expected foreign function call");
        }
    }

    #[test]
    fn parse_empty_args() {
        let tokens = vec![create_token(TokenKind::ParenR, 0, 1)];

        let mut parser = create_parser(&tokens);
        let args = parser.parse_args().unwrap();
        assert!(args.is_empty());
    }

    #[test]
    fn parse_multiple_args() {
        let tokens = vec![
            create_token(TokenKind::Int(1), 0, 1),
            create_token(TokenKind::Comma, 1, 2),
            create_token(TokenKind::StringLiteral("hello".to_string()), 3, 10),
            create_token(TokenKind::Comma, 10, 11),
            create_token(TokenKind::Bool(true), 12, 16),
            create_token(TokenKind::ParenR, 16, 17),
        ];

        let mut parser = create_parser(&tokens);
        let args = parser.parse_args().unwrap();
        assert_eq!(args.len(), 3);
    }

    #[test]
    fn parse_string_literal() {
        let tokens = vec![create_token(
            TokenKind::StringLiteral("hello world".to_string()),
            0,
            13,
        )];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        match expr.kind {
            ExprKind::Literal(Value::String(s)) => assert_eq!(s, "hello world"),
            _ => panic!("Expected string literal"),
        }
    }

    #[test]
    fn parse_float_literal() {
        let tokens = vec![create_token(TokenKind::Float(3.14), 0, 4)];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        match expr.kind {
            ExprKind::Literal(Value::Float(f)) => assert!((f - 3.14).abs() < f64::EPSILON),
            _ => panic!("Expected float literal"),
        }
    }

    #[test]
    fn parse_complex_expression() {
        // (1 + 2) * 3 == 9 and true
        let tokens = vec![
            create_token(TokenKind::ParenL, 0, 1),
            create_token(TokenKind::Int(1), 1, 2),
            create_token(TokenKind::Plus, 3, 4),
            create_token(TokenKind::Int(2), 5, 6),
            create_token(TokenKind::ParenR, 6, 7),
            create_token(TokenKind::Asterisk, 8, 9),
            create_token(TokenKind::Int(3), 10, 11),
            create_token(TokenKind::Equal, 12, 14),
            create_token(TokenKind::Int(9), 15, 16),
            create_token(TokenKind::And, 17, 20),
            create_token(TokenKind::Bool(true), 21, 25),
        ];

        let mut parser = create_parser(&tokens);
        let expr = parser.parse_expr().unwrap();

        // Should parse as ((1 + 2) * 3 == 9) and true
        if let ExprKind::BinaryOp(BinaryOp::And, ..) = &expr.kind {
            // Success - parsed as logical AND at top level
        } else {
            panic!("Expected logical AND at top level");
        }
    }
}
