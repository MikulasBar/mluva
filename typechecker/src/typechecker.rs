use std::collections::HashMap;

use super::data_type_scope::DataTypeScope;
use crate::bin_op_pat;
use common::ast::{BinaryOp, Expr, ExprKind, Statement, StatementKind, UnaryOp};
use common::compile_error::CompileError;
use common::data_type::DataType;
use common::diagnostics::Span;
use common::function_signiture_manager::{self, FunctionSignitureManager};
use common::type_manager::{TypeManager, TypeSpec};

pub struct TypeChecker<'a> {
    type_manager: &'a TypeManager,
    function_manager: &'a FunctionSignitureManager,
    dependencies: &'a HashMap<String, FunctionSignitureManager>,
    scope: DataTypeScope,
}

impl<'a> TypeChecker<'a> {
    pub fn new(
        type_manager: &'a TypeManager,
        function_manager: &'a FunctionSignitureManager,
        dependencies: &'a HashMap<String, FunctionSignitureManager>,
    ) -> Self {
        Self {
            type_manager,
            function_manager,
            dependencies,
            scope: DataTypeScope::new(),
        }
    }

    pub fn check(mut self) -> Result<(), CompileError> {
        self.check_functions()
    }

    fn check_functions(&mut self) -> Result<(), CompileError> {
        for slot in 0..self.function_manager.count() {
            self.scope.enter();

            self.function_manager
                .get_signiture(slot as u32)
                .unwrap()
                .params
                .iter()
                .try_for_each(|param| {
                    self.scope
                        .insert_new_var(param.name.clone(), param.ty.clone(), param.span)
                })?;

            let statements = self.function_manager.get_body(slot as u32).unwrap();
            let return_type = self
                .function_manager
                .get_signiture(slot as u32)
                .unwrap()
                .return_type
                .clone();

            self.check_statements(statements, &return_type)?;

            self.scope.exit();
        }

        Ok(())
    }

    fn check_statements(
        &mut self,
        statements: &[Statement],
        return_type: &TypeSpec,
    ) -> Result<(), CompileError> {
        for statement in statements {
            self.check_statement(statement, return_type)?;
        }

        Ok(())
    }

    fn check_statement(
        &mut self,
        statement: &Statement,
        return_type: &TypeSpec,
    ) -> Result<(), CompileError> {
        match &statement.kind {
            StatementKind::If {
                condition,
                if_block,
                else_block,
            } => {
                let cond = self.check_expr(&condition)?;
                if !cond.is_bool() {
                    return Err(CompileError::wrong_type_at(
                        TypeSpec::bool(),
                        cond,
                        statement.span,
                        self.type_manager,
                    ));
                }

                self.check_statements(&if_block, return_type)?;
                if let Some(else_stmts) = else_block {
                    self.check_statements(&else_stmts, return_type)?;
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

                let expr_type = self.check_expr(&value)?;
                let expr_span = value.span;

                let data_type = match (var_type, expr_type) {
                    (Some(var_type), expr_type) if !expr_type.matches_type(var_type) => {
                        return Err(CompileError::wrong_type_at(
                            var_type.clone(),
                            expr_type,
                            expr_span,
                            self.type_manager,
                        ));
                    }
                    (None, DataType::List { item_type: None }) => {
                        return Err(CompileError::cannot_infer_type_at(expr_span));
                    }
                    (Some(var_type), _) => var_type.clone(),
                    (None, expr_type) => expr_type,
                };

                self.scope
                    .insert_new_pattern(assignee.clone(), data_type, statement.span)?;
            }

            StatementKind::VarAssign { assignee, value } => {
                let expr_type = self.check_expr(&value)?;
                let assignee_type = self.scope.get_pattern(assignee)?;

                if !expr_type.matches_type(&assignee_type) {
                    return Err(CompileError::wrong_type_at(
                        assignee_type,
                        expr_type,
                        statement.span,
                    ));
                }
            }

            StatementKind::While { condition, block } => {
                let cond = self.check_expr(&condition)?;
                if !cond.is_bool() {
                    return Err(CompileError::wrong_type_at(
                        DataType::Bool,
                        cond,
                        statement.span,
                    ));
                }

                return self.check_statements(&block, return_type);
            }

            StatementKind::Expr(expr) => {
                self.check_expr(&expr)?;
            }

            StatementKind::Return(expr) => {
                let expr_type = self.check_expr(&expr)?;
                if expr_type != *return_type {
                    return Err(CompileError::wrong_type_at(
                        return_type.clone(),
                        expr_type,
                        statement.span,
                    ));
                }
            }
        }

        Ok(())
    }

    fn check_expr(&self, expr: &Expr) -> Result<TypeSpec, CompileError> {
        match &expr.kind {
            ExprKind::Var(ident) => {
                let Some(data_type) = self.scope.get(&ident) else {
                    return Err(CompileError::variable_not_found_at(
                        ident.clone(),
                        expr.span,
                    ));
                };

                Ok(data_type.clone())
            }
            ExprKind::VoidLiteral => Ok(TypeSpec::void()),
            ExprKind::I32Literal(_) => Ok(TypeSpec::i32()),
            ExprKind::F32Literal(_) => Ok(TypeSpec::f32()),
            ExprKind::BoolLiteral(_) => Ok(TypeSpec::bool()),
            ExprKind::StringLiteral(_) => Ok(TypeSpec::string()),
            ExprKind::ListLiteral(list) => {
                if list.is_empty() {
                    Ok(TypeSpec::unknown_list())
                } else {
                    let first_type = self.check_expr(&list[0])?;
                    for element in list.iter().skip(1) {
                        let element_type = self.check_expr(element)?;
                        if element_type != first_type {
                            return Err(CompileError::wrong_type_at(
                                first_type.clone(),
                                element_type,
                                expr.span,
                                self.type_manager,
                            ));
                        }
                    }

                    Ok(TypeSpec::list_of(first_type))
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
                        self.type_manager,
                    ));
                }

                if let Some(item_type) = callee_type.get_index_type() {
                    Ok(item_type)
                } else {
                    return Err(CompileError::invalid_indexing_at(expr.span));
                }
            }

            ExprKind::FunctionCall { func_name, args } => {
                self.check_call_expr(expr, func_name, args)
            }

            ExprKind::ForeignFunctionCall {
                module_name,
                func_name,
                args,
            } => self.check_foreign_call_expr(expr, module_name, func_name, args),

            ExprKind::MethodCall {
                callee,
                method_name,
                args,
            } => self.check_method_call_expr(expr, callee, method_name, args),

            ExprKind::BinaryOp(op, lhs, rhs) => self.check_binary_op_expr(expr, op, lhs, rhs),
            ExprKind::UnaryOp(op, expr) => self.check_unary_op_expr(expr, op),
        }
    }

    fn check_call_expr(
        &self,
        expr: &Expr,
        func_name: &str,
        args: &[Expr],
    ) -> Result<DataType, CompileError> {
        let Some(signiture) = self.ast.get_function_signiture(&func_name) else {
            return Err(CompileError::function_not_found_at(func_name, expr.span));
        };

        let arg_types: Vec<(DataType, Span)> = args
            .iter()
            .map(|arg| self.check_expr(arg).map(|dt| (dt, arg.span)))
            .collect::<Result<Vec<(DataType, Span)>, CompileError>>()?;

        signiture.check_argument_types(&arg_types, expr.span)?;

        Ok(signiture.return_type.clone())
    }

    fn check_foreign_call_expr(
        &self,
        expr: &Expr,
        module_name: &str,
        func_name: &str,
        args: &[Expr],
    ) -> Result<DataType, CompileError> {
        let signiture = self
            .dependencies
            .get(module_name)
            .ok_or_else(|| CompileError::module_not_found_at(module_name.clone(), expr.span))?
            .get_function_signiture(&func_name)
            .ok_or_else(|| CompileError::function_not_found_at(func_name.clone(), expr.span))?;

        let arg_types: Vec<(DataType, Span)> = args
            .iter()
            .map(|arg| self.check_expr(arg).map(|dt| (dt, arg.span)))
            .collect::<Result<Vec<(DataType, Span)>, CompileError>>()?;

        signiture.check_argument_types(&arg_types, expr.span)?;

        Ok(signiture.return_type.clone())
    }

    fn check_method_call_expr(
        &self,
        expr: &Expr,
        callee: &Expr,
        method_name: &str,
        args: &[Expr],
    ) -> Result<DataType, CompileError> {
        let callee_type = self.check_expr(callee)?;

        let arg_types: Vec<DataType> = args
            .iter()
            .map(|arg| self.check_expr(arg))
            .collect::<Result<Vec<DataType>, CompileError>>()?;

        callee_type.check_method_call(method_name, expr.span, &arg_types)
    }

    fn check_binary_op_expr(
        &self,
        expr: &Expr,
        op: &BinaryOp,
        lhs: &Expr,
        rhs: &Expr,
    ) -> Result<DataType, CompileError> {
        let lhs_type = self.check_expr(&lhs)?;
        let rhs_type = self.check_expr(&rhs)?;
        match op {
            bin_op_pat!(NUMERIC) => match (&lhs_type, &rhs_type) {
                (DataType::Int, DataType::Int) => Ok(DataType::Int),
                (DataType::Float, DataType::Float) => Ok(DataType::Float),
                (DataType::Int | DataType::Float, _) => {
                    return Err(CompileError::wrong_type_at(lhs_type, rhs_type, expr.span));
                }
                (_, DataType::Int | DataType::Float) => {
                    return Err(CompileError::wrong_type_at(rhs_type, lhs_type, expr.span));
                }
                _ => {
                    return Err(CompileError::wrong_type_at(
                        DataType::Int,
                        lhs_type,
                        expr.span,
                    ));
                }
            },

            bin_op_pat!(NUMERIC_COMPARISON) => match (&lhs_type, &rhs_type) {
                (DataType::Int, DataType::Int) => Ok(DataType::Bool),
                (DataType::Float, DataType::Float) => Ok(DataType::Bool),
                (DataType::Int | DataType::Float, _) => {
                    return Err(CompileError::wrong_type_at(lhs_type, rhs_type, expr.span));
                }
                (_, DataType::Int | DataType::Float) => {
                    return Err(CompileError::wrong_type_at(rhs_type, lhs_type, expr.span));
                }
                _ => {
                    return Err(CompileError::wrong_type_at(
                        DataType::Int,
                        lhs_type,
                        expr.span,
                    ));
                }
            },

            bin_op_pat!(COMPARISON) => Ok(DataType::Bool),

            bin_op_pat!(LOGICAL) => {
                if lhs_type != DataType::Bool {
                    return Err(CompileError::wrong_type_at(
                        DataType::Bool,
                        lhs_type,
                        expr.span,
                    ));
                }

                if rhs_type != DataType::Bool {
                    return Err(CompileError::wrong_type_at(
                        DataType::Bool,
                        rhs_type,
                        expr.span,
                    ));
                }

                Ok(DataType::Bool)
            }
        }
    }

    fn check_unary_op_expr(&self, expr: &Expr, op: &UnaryOp) -> Result<DataType, CompileError> {
        let expr_type = self.check_expr(&expr)?;
        match op {
            UnaryOp::Not => {
                if expr_type != DataType::Bool {
                    return Err(CompileError::wrong_type_at(
                        DataType::Bool,
                        expr_type,
                        expr.span,
                    ));
                }

                Ok(DataType::Bool)
            }

            UnaryOp::Negate => match expr_type {
                DataType::Int => Ok(DataType::Int),
                DataType::Float => Ok(DataType::Float),
                _ => {
                    return Err(CompileError::wrong_type_at(
                        DataType::Int,
                        expr_type,
                        expr.span,
                    ));
                }
            },
        }
    }
}
