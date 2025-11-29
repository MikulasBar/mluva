use crate::expect_token;
use common::ast::*;
use common::compile_error::CompileError;
use common::diagnostics::{FileId, Span};
use common::function_signiture_manager::FunctionSignitureManager;
use common::token::{Token, TokenKind};
use common::type_manager::{TypeManager, TypeSpec};

pub struct Parser<'a> {
    file_id: FileId,
    tokens: &'a [Token],
    index: usize,
    imports: Vec<Path>,
    type_manager: TypeManager,
    function_manager: FunctionSignitureManager,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], file_id: FileId) -> Self {
        Self {
            file_id,
            tokens,
            index: 0,
            imports: vec![],
            type_manager: TypeManager::builtin(),
            function_manager: FunctionSignitureManager::empty(),
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

    pub fn parse(
        mut self,
    ) -> Result<(Vec<Path>, TypeManager, FunctionSignitureManager), CompileError> {
        self.parse_top_level()?;
        Ok((self.imports, self.type_manager, self.function_manager))
    }

    fn parse_top_level(&mut self) -> Result<(), CompileError> {
        while let Some(token) = self.peek() {
            let token_span = token.span;

            match &token.kind {
                TokenKind::EOL => {
                    self.skip();
                    continue;
                }

                TokenKind::Ident(_) => {
                    let (return_type, _) = self.parse_type()?;
                    expect_token!(TokenKind::Ident(name), name_span in self);
                    expect_token!(TokenKind::ParenL in self);

                    let params = self.parse_named_parameters()?;

                    expect_token!(TokenKind::ParenR, paren_r_span in self);
                    expect_token!(TokenKind::BraceL in self);

                    let body = self.parse_statements(TokenKind::BraceR)?;

                    expect_token!(TokenKind::BraceR in self);

                    let signiture =
                        FunctionSigniture::new(return_type, params, token_span.join(paren_r_span));

                    self.function_manager.add(name, signiture, body);
                }

                TokenKind::Import => {
                    expect_token!(TokenKind::Import in self);
                    expect_token!(TokenKind::Ident(module_name) in self);
                    expect_token!(TokenKind::EOL in self);

                    let import_path = Path::single(module_name);
                    self.imports.push(import_path);
                }

                _ => {
                    return Err(CompileError::unexpected_token_at(
                        token.kind.clone(),
                        token.span,
                    ));
                }
            }
        }

        Ok(())
    }

    fn parse_named_parameters(&mut self) -> Result<Vec<Parameter>, CompileError> {
        let mut params = vec![];
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::ParenR {
                break;
            }

            let (ty, ty_span) = self.parse_type()?;
            expect_token!(TokenKind::Ident(ident), ident_span in self);
            let param = Parameter::new(ident, ty, ty_span.join(ident_span));
            params.push(param);

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
                        Statement::return_statement(Expr::void_literal(token_span), token_span)
                    } else {
                        let expr = self.parse_expr()?;
                        expect_token!(TokenKind::EOL in self);
                        Statement::return_statement(expr, token_span)
                    }
                }

                TokenKind::Let => {
                    expect_token!(TokenKind::Let in self);
                    let pattern = self.parse_pattern()?;
                    expect_token!(TokenKind::Assign in self);
                    let expr = self.parse_expr()?;
                    expect_token!(TokenKind::EOL in self);

                    let expr_span = expr.span;
                    Statement::var_declare(None, pattern, expr, token_span.join(expr_span))
                }

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
        let saved_index = self.index;
        let mut var_type = None;
        let mut start_span = None;
        let mut pattern = None;

        // Try to parse as typed variable declaration first
        if let Ok((ty, ty_span)) = self.parse_type() {
            if let Ok(decl_pattern) = self.parse_pattern() {
                pattern = Some(decl_pattern);
                var_type = Some(ty);
                start_span = Some(ty_span);
            }
        }

        if pattern.is_none() {
            self.index = saved_index;
            pattern = Some(self.parse_pattern()?);
        }

        let pattern = pattern.unwrap();
        let start_span = start_span.unwrap_or(pattern.span);

        expect_token!(TokenKind::Assign in self);
        let expr = self.parse_expr()?;
        expect_token!(TokenKind::EOL, end_span in self);

        if var_type.is_some() {
            Ok(Statement::var_declare(
                var_type,
                pattern,
                expr,
                start_span.join(end_span),
            ))
        } else {
            Ok(Statement::var_assign(
                pattern,
                expr,
                start_span.join(end_span),
            ))
        }
    }

    fn parse_pattern(&mut self) -> Result<Pattern, CompileError> {
        self.parse_postfix_pattern()
    }

    fn parse_postfix_pattern(&mut self) -> Result<Pattern, CompileError> {
        let mut pattern = self.parse_atom_pattern()?;

        loop {
            match self.peek_kind() {
                Some(TokenKind::BracketL) => {
                    expect_token!(TokenKind::BracketL in self);
                    let index_expr = self.parse_expr()?;
                    expect_token!(TokenKind::BracketR, end_span in self);
                    let span = pattern.span.join(end_span);
                    pattern = Pattern::index(pattern, index_expr, span);
                }

                _ => break,
            }
        }

        Ok(pattern)
    }

    fn parse_atom_pattern(&mut self) -> Result<Pattern, CompileError> {
        let Some(token) = self.peek() else {
            return Err(CompileError::unexpected_end_of_file(self.file_id));
        };

        let token_span = token.span;

        match token.kind {
            TokenKind::Ident(_) => self.parse_pattern(),

            _ => {
                return Err(CompileError::unexpected_token_at(
                    self.next().unwrap().kind,
                    token_span,
                ));
            }
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
            return self.parse_postfix_expr();
        };

        let token_span = token.span;
        self.skip();
        let expr = self.parse_unary_op_expr()?;
        let expr_span = expr.span;
        return Ok(Expr::unary_op(op, expr, token_span.join(expr_span)));
    }

    fn parse_postfix_expr(&mut self) -> Result<Expr, CompileError> {
        let mut expr = self.parse_atom_expr()?;

        loop {
            match self.peek_kind() {
                Some(TokenKind::Dot) => {
                    self.skip();
                    expect_token!(TokenKind::Ident(method_name) in self);
                    expect_token!(TokenKind::ParenL in self);
                    let args = self.parse_args(TokenKind::ParenR)?;
                    expect_token!(TokenKind::ParenR, end_span in self);
                    let span = expr.span.join(end_span);
                    expr = Expr::method_call(expr, method_name, args, span);
                }

                Some(TokenKind::BracketL) => {
                    self.skip();
                    let index = self.parse_expr()?;
                    expect_token!(TokenKind::BracketR, end_span in self);
                    let span = expr.span.join(end_span);
                    expr = Expr::index_get(expr, index, span);
                }

                _ => break,
            }
        }

        Ok(expr)
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
                Ok(Expr::bool_literal(bool, token_span))
            }

            TokenKind::Int(_) => {
                expect_token!(TokenKind::Int(int) in self);
                Ok(Expr::i32_literal(int, token_span))
            }

            TokenKind::Float(_) => {
                expect_token!(TokenKind::Float(float) in self);
                Ok(Expr::f32_literal(float as f32, token_span))
            }

            TokenKind::StringLiteral(_) => {
                expect_token!(TokenKind::StringLiteral(string) in self);
                Ok(Expr::string_literal(string, token_span))
            }

            TokenKind::Ident(_) => self.parse_ident_expr(),

            TokenKind::ParenL => {
                expect_token!(TokenKind::ParenL in self);
                let inner = self.parse_expr();
                expect_token!(TokenKind::ParenR in self);
                inner
            }

            TokenKind::BracketL => {
                expect_token!(TokenKind::BracketL in self);
                let elements = self.parse_args(TokenKind::BracketR)?;
                expect_token!(TokenKind::BracketR, end_span in self);

                Ok(Expr::list_literal(elements, token_span.join(end_span)))
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
                let args = self.parse_args(TokenKind::ParenR)?;
                expect_token!(TokenKind::ParenR, end_span in self);

                Ok(Expr::function_call(ident, args, ident_span.join(end_span)))
            }

            Some(TokenKind::Colon) => {
                expect_token!(TokenKind::Colon in self);
                expect_token!(TokenKind::Ident(func_name) in self);

                expect_token!(TokenKind::ParenL in self);
                let args = self.parse_args(TokenKind::ParenR)?;
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

    fn parse_args(&mut self, critical_kind: TokenKind) -> Result<Vec<Expr>, CompileError> {
        let mut args = vec![];

        while let Some(token) = self.peek() {
            if token.kind == critical_kind {
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

    fn parse_type(&mut self) -> Result<(TypeSpec, Span), CompileError> {
        expect_token!(TokenKind::Ident(ident), ident_span in self);

        let type_id = if let Some(type_id) = self.type_manager.get_id(&ident) {
            type_id
        } else {
            return Err(CompileError::unknown_type_at(ident, ident_span));
        };

        match self.peek_kind() {
            Some(TokenKind::ArrowL) => {
                self.skip();
                let generics = self.parse_type_args()?;
                expect_token!(TokenKind::ArrowR, end_span in self);
                Ok((TypeSpec::new(type_id, generics), ident_span.join(end_span)))
            }

            _ => Ok((TypeSpec::new(type_id, vec![]), ident_span)),
        }
    }

    fn parse_type_args(&mut self) -> Result<Vec<TypeSpec>, CompileError> {
        let mut type_args = vec![];

        loop {
            let (ty, _) = self.parse_type()?;
            type_args.push(ty);

            if let Some(&TokenKind::Comma) = self.peek_kind() {
                self.skip();
            } else {
                break;
            }
        }

        Ok(type_args)
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
        TokenKind::ArrowL => Some(BinaryOp::Less),
        TokenKind::LessEqual => Some(BinaryOp::LessEqual),
        TokenKind::ArrowR => Some(BinaryOp::Greater),
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
        TokenKind::Percent => Some(BinaryOp::Modulo),
        _ => None,
    }
}

fn token_to_unary_op(token: &Token) -> Option<UnaryOp> {
    match &token.kind {
        TokenKind::Bang => Some(UnaryOp::Not),
        TokenKind::Minus => Some(UnaryOp::Negate),
        _ => None,
    }
}
