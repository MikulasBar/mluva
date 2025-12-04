use std::collections::HashMap;
use std::mem;

use super::type_scope::TypeScope;
use crate::bin_op_pat;
use common::ast::{BinaryOp, Expr, ExprKind, Statement, StatementKind, UnaryOp};
use common::compile_error::CompileError;
use common::diagnostics::Span;
use common::module::module_ast::ModuleAST;
use common::module::module_signiture::ModuleSigniture;
use common::type_manager::TypeSpec;

pub struct TypeChecker<'a> {
    ast: &'a mut ModuleAST,
    dependencies: &'a HashMap<String, ModuleSigniture>,
    scope: TypeScope,
}

impl<'a> TypeChecker<'a> {
    pub fn new(ast: &'a mut ModuleAST, dependencies: &'a HashMap<String, ModuleSigniture>) -> Self {
        Self {
            ast,
            dependencies,
            scope: TypeScope::new(),
        }
    }

    pub fn check(mut self) -> Result<(), CompileError> {
        self.check_functions()
    }

    fn check_functions(&mut self) -> Result<(), CompileError> {
        for slot in 0..self.ast.fn_count() {
            self.scope.enter();

            let signiture = self.ast.get_sign(slot as u32).unwrap();

            signiture.params.iter().try_for_each(|p| {
                self.scope
                    .insert_new_var(p.name.clone(), p.ty.clone(), p.span)
            })?;

            let return_type = signiture.return_type.clone();

            // Take the body out to avoid borrowing issues
            // This shouldn't break anything, because we this only once,
            // so no one else will use the body when we have it taken out
            let mut statements = mem::take(self.ast.get_body_mut(slot as u32).unwrap());

            self.check_statements(&mut statements, &return_type)?;

            let body = self.ast.get_body_mut(slot as u32).unwrap();
            *body = statements;

            self.scope.exit();
        }

        Ok(())
    }

    fn check_statements(
        &mut self,
        statements: &mut [Statement],
        return_type: &TypeSpec,
    ) -> Result<(), CompileError> {
        for statement in statements {
            self.check_statement(statement, return_type)?;
        }

        Ok(())
    }

    fn check_statement(
        &mut self,
        statement: &mut Statement,
        return_type: &TypeSpec,
    ) -> Result<(), CompileError> {
        match &mut statement.kind {
            StatementKind::If {
                condition,
                if_block,
                else_block,
            } => {
                let cond = self.check_expr(condition)?;
                if !cond.is_bool() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::bool(),
                        cond,
                        statement.span,
                        self.ast.tm(),
                    ));
                }

                self.check_statements(if_block, return_type)?;
                if let Some(else_stmts) = else_block {
                    self.check_statements(else_stmts, return_type)?;
                }
            }

            StatementKind::VarDeclare {
                assignee_ty: var_type,
                assignee,
                value,
            } => {
                if !assignee.is_declarable() {
                    return Err(CompileError::invalid_pattern_at(assignee.span));
                }

                let value_type = self.check_expr(value)?;
                let expr_span = value.span;

                let data_type = match (var_type, value_type) {
                    (Some(var_type), val_type) if !val_type.matches(var_type) => {
                        return Err(CompileError::wrong_type_at(
                            var_type.clone(),
                            val_type,
                            expr_span,
                            self.ast.tm(),
                        ));
                    }
                    (None, ty) if ty == TypeSpec::unknown_list() => {
                        return Err(CompileError::cannot_infer_type_at(expr_span));
                    }
                    (Some(var_type), _) => var_type.clone(),
                    (None, val_type) => val_type,
                };

                self.scope
                    .insert_new_pattern(assignee.clone(), data_type, statement.span)?;
            }

            StatementKind::VarAssign { assignee, value } => {
                let expr_type = self.check_expr(value)?;
                let assignee_type = self.scope.get_pattern(assignee)?;

                if !expr_type.matches(&assignee_type) {
                    return Err(CompileError::wrong_type_at(
                        assignee_type,
                        expr_type,
                        statement.span,
                        self.ast.tm(),
                    ));
                }
            }

            StatementKind::While { condition, block } => {
                let cond = self.check_expr(condition)?;
                if !cond.is_bool() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::bool(),
                        cond,
                        statement.span,
                        self.ast.tm(),
                    ));
                }

                return self.check_statements(block, return_type);
            }

            StatementKind::Expr(expr) => {
                self.check_expr(expr)?;
            }

            StatementKind::Return(expr) => {
                let expr_type = self.check_expr(expr)?;
                if expr_type != *return_type {
                    return Err(CompileError::wrong_type_at(
                        return_type.clone(),
                        expr_type,
                        statement.span,
                        self.ast.tm(),
                    ));
                }
            }
        }

        Ok(())
    }

    fn check_expr(&self, expr: &mut Expr) -> Result<TypeSpec, CompileError> {
        let expr_ty = match &mut expr.kind {
            ExprKind::Var(ident) => {
                let Some(ty) = self.scope.get(&ident) else {
                    return Err(CompileError::variable_not_found_at(
                        ident.clone(),
                        expr.span,
                    ));
                };

                ty.clone()
            }
            ExprKind::VoidLiteral => TypeSpec::void(),
            ExprKind::I32Literal(_) => TypeSpec::i32(),
            ExprKind::F32Literal(_) => TypeSpec::f32(),
            ExprKind::BoolLiteral(_) => TypeSpec::bool(),
            ExprKind::StringLiteral(_) => TypeSpec::string(),
            ExprKind::ListLiteral(list) => {
                if list.is_empty() {
                    TypeSpec::unknown_list()
                } else {
                    let first_type = self.check_expr(&mut list[0])?;
                    for element in list.iter_mut().skip(1) {
                        let element_type = self.check_expr(element)?;
                        if element_type != first_type {
                            return Err(CompileError::wrong_type_at(
                                first_type.clone(),
                                element_type,
                                expr.span,
                                self.ast.tm(),
                            ));
                        }
                    }

                    TypeSpec::list_of(first_type)
                }
            }

            ExprKind::IndexGet { callee, index } => {
                let callee_type = self.check_expr(callee)?;
                let index_type = self.check_expr(index)?;

                if index_type != TypeSpec::i32() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::i32(),
                        index_type,
                        expr.span,
                        self.ast.tm(),
                    ));
                }

                if let Some(item_type) = callee_type.get_index_type() {
                    item_type
                } else {
                    return Err(CompileError::invalid_indexing_at(expr.span));
                }
            }

            ExprKind::FunctionCall { func_name, args } => {
                self.check_call_expr(expr.span, func_name, args)?
            }

            ExprKind::ForeignFunctionCall {
                module_name,
                func_name,
                args,
            } => self.check_foreign_call_expr(expr.span, module_name, func_name, args)?,

            ExprKind::MethodCall {
                callee,
                method_name,
                args,
            } => self.check_method_call_expr(expr.span, callee, method_name, args)?,

            ExprKind::BinaryOp(op, lhs, rhs) => {
                self.check_binary_op_expr(expr.span, op, lhs, rhs)?
            }
            ExprKind::UnaryOp(op, expr) => self.check_unary_op_expr(expr, op)?,
        };

        expr.ty = Some(expr_ty.clone());

        Ok(expr_ty)
    }

    fn check_call_expr(
        &self,
        span: Span,
        func_name: &str,
        args: &mut [Expr],
    ) -> Result<TypeSpec, CompileError> {
        let Some(sign) = self.ast.get_sign_by_name(&func_name) else {
            return Err(CompileError::function_not_found_at(func_name, span));
        };

        let arg_types: Vec<(TypeSpec, Span)> = args
            .iter_mut()
            .map(|arg| self.check_expr(arg).map(|dt| (dt, arg.span)))
            .collect::<Result<Vec<(TypeSpec, Span)>, CompileError>>()?;

        sign.check_argument_types(&arg_types, span, self.ast.tm())?;

        Ok(sign.return_type.clone())
    }

    fn check_foreign_call_expr(
        &self,
        span: Span,
        module_name: &str,
        func_name: &str,
        args: &mut [Expr],
    ) -> Result<TypeSpec, CompileError> {
        let signiture = self
            .dependencies
            .get(module_name)
            .ok_or_else(|| CompileError::module_not_found_at(module_name, span))?
            .get_sign_by_name(&func_name)
            .ok_or_else(|| CompileError::function_not_found_at(func_name, span))?;

        let arg_types: Vec<(TypeSpec, Span)> = args
            .iter_mut()
            .map(|arg| self.check_expr(arg).map(|dt| (dt, arg.span)))
            .collect::<Result<Vec<(TypeSpec, Span)>, CompileError>>()?;

        signiture.check_argument_types(&arg_types, span, self.ast.tm())?;

        Ok(signiture.return_type.clone())
    }

    fn check_method_call_expr(
        &self,
        span: Span,
        callee: &mut Expr,
        method_name: &str,
        args: &mut [Expr],
    ) -> Result<TypeSpec, CompileError> {
        let callee_type = self.check_expr(callee)?;

        let arg_types: Vec<(TypeSpec, Span)> = args
            .iter_mut()
            .map(|arg| self.check_expr(arg).map(|t| (t, arg.span)))
            .collect::<Result<Vec<(TypeSpec, Span)>, CompileError>>()?;

        let ty = self
            .ast
            .get_type(callee_type.id)
            .expect("Internal Typechecker error, not recovering");

        let method_slot =
            ty.get_method_slot(method_name)
                .ok_or(CompileError::method_not_found_at(
                    callee_type,
                    method_name,
                    span,
                    self.ast.tm(),
                ))?;

        let method = self.ast.get_sign(method_slot).unwrap();
        method.check_argument_types(&arg_types, span, self.ast.tm())?;

        Ok(method.return_type.clone())
    }

    fn check_binary_op_expr(
        &self,
        span: Span,
        op: &BinaryOp,
        lhs: &mut Expr,
        rhs: &mut Expr,
    ) -> Result<TypeSpec, CompileError> {
        let lhs_type = self.check_expr(lhs)?;
        let rhs_type = self.check_expr(rhs)?;
        match op {
            bin_op_pat!(NUMERIC) => {
                if !lhs_type.is_i32() && !lhs_type.is_f32() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::i32(),
                        lhs_type,
                        span,
                        self.ast.tm(),
                    ));
                }

                if rhs_type != lhs_type {
                    return Err(CompileError::wrong_type_at(
                        lhs_type,
                        rhs_type,
                        span,
                        self.ast.tm(),
                    ));
                }

                Ok(lhs_type)
            }

            bin_op_pat!(NUMERIC_COMPARISON) => {
                if !lhs_type.is_i32() && !lhs_type.is_f32() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::i32(),
                        lhs_type,
                        span,
                        self.ast.tm(),
                    ));
                }

                if rhs_type != lhs_type {
                    return Err(CompileError::wrong_type_at(
                        lhs_type,
                        rhs_type,
                        span,
                        self.ast.tm(),
                    ));
                }

                Ok(TypeSpec::bool())
            }

            bin_op_pat!(COMPARISON) => {
                if lhs_type != rhs_type {
                    return Err(CompileError::wrong_type_at(
                        lhs_type,
                        rhs_type,
                        span,
                        self.ast.tm(),
                    ));
                }

                Ok(TypeSpec::bool())
            }

            bin_op_pat!(LOGICAL) => {
                if !lhs_type.is_bool() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::bool(),
                        lhs_type,
                        span,
                        self.ast.tm(),
                    ));
                }

                if !rhs_type.is_bool() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::bool(),
                        rhs_type,
                        span,
                        self.ast.tm(),
                    ));
                }

                Ok(TypeSpec::bool())
            }
        }
    }

    fn check_unary_op_expr(&self, expr: &mut Expr, op: &UnaryOp) -> Result<TypeSpec, CompileError> {
        let expr_type = self.check_expr(expr)?;
        match op {
            UnaryOp::Not => {
                if !expr_type.is_bool() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::bool(),
                        expr_type,
                        expr.span,
                        self.ast.tm(),
                    ));
                }

                Ok(TypeSpec::bool())
            }

            UnaryOp::Negate => match expr_type {
                t if t == TypeSpec::i32() => Ok(TypeSpec::i32()),
                t if t == TypeSpec::f32() => Ok(TypeSpec::f32()),
                _ => {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::i32(),
                        expr_type,
                        expr.span,
                        self.ast.tm(),
                    ));
                }
            },
        }
    }
}
