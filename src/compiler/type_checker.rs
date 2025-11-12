use std::collections::HashMap;

use super::data_type_scope::DataTypeScope;
use crate::ast::{
    Ast, BinaryOp, BuiltinFunction, Expr, ExprKind, Statement, StatementKind, UnaryOp,
};
use crate::bin_op_pat;
use crate::data_type::DataType;
use crate::diagnostics::Span;
use crate::errors::CompileError;
use crate::module::Module;

pub struct TypeChecker<'a> {
    ast: &'a Ast,
    dependencies: &'a HashMap<String, Module>,
    scope: DataTypeScope,
}

impl<'a> TypeChecker<'a> {
    pub fn new(ast: &'a Ast, dependencies: &'a HashMap<String, Module>) -> Self {
        Self {
            ast,
            dependencies,
            scope: DataTypeScope::new(),
        }
    }

    pub fn check(mut self) -> Result<(), CompileError> {
        self.check_functions()
    }

    fn check_functions(&mut self) -> Result<(), CompileError> {
        for slot in 0..self.ast.function_count() {
            self.scope.enter();

            self.ast
                .get_function_signiture_by_slot(slot)
                .unwrap()
                .params
                .iter()
                .try_for_each(|param| {
                    self.scope
                        .insert_new(param.name.clone(), param.data_type.clone(), param.span)
                })?;

            let statements = self.ast.get_function_body_by_slot(slot).unwrap();
            let return_type = self
                .ast
                .get_function_signiture_by_slot(slot)
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
        return_type: &DataType,
    ) -> Result<(), CompileError> {
        for statement in statements {
            self.check_statement(statement, return_type)?;
        }

        Ok(())
    }

    fn check_statement(
        &mut self,
        statement: &Statement,
        return_type: &DataType,
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
                        DataType::Bool,
                        cond,
                        statement.span,
                    ));
                }

                self.check_statements(&if_block, return_type)?;
                if let Some(else_stmts) = else_block {
                    self.check_statements(&else_stmts, return_type)?;
                }
            }

            StatementKind::VarDeclare {
                data_type: var_type,
                variable,
                value,
            } => {
                // if the declaration has explicit type or not
                // check the type if yes
                // if no then do essentialy nothing
                let expr_type = self.check_expr(&value)?;
                let expr_span = value.span;

                let data_type = match (var_type, expr_type) {
                    (Some(var_type), expr_type) if !expr_type.matches_type(var_type) => {
                        return Err(CompileError::wrong_type_at(
                            var_type.clone(),
                            expr_type,
                            expr_span,
                        ));
                    }
                    (None, DataType::List { item_type: None }) => {
                        return Err(CompileError::cannot_infer_type_at(
                            variable.clone(),
                            expr_span,
                        ));
                    }
                    (Some(var_type), _) => var_type.clone(),
                    (None, expr_type) => expr_type,
                };

                self.scope
                    .insert_new(variable.clone(), data_type, statement.span)?;
            }

            StatementKind::VarAssign { variable, value } => {
                let Some(var_type) = self.scope.get(&variable) else {
                    return Err(CompileError::variable_not_found_at(
                        variable.clone(),
                        statement.span,
                    ));
                };

                let expr_type = self.check_expr(&value)?;

                if !expr_type.matches_type(var_type) {
                    return Err(CompileError::wrong_type_at(
                        var_type.clone(),
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

    fn check_expr(&self, expr: &Expr) -> Result<DataType, CompileError> {
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
            ExprKind::Literal(lit) => Ok(lit.get_type()),
            ExprKind::ListLiteral(list) => {
                if list.is_empty() {
                    Ok(DataType::unknow_list())
                } else {
                    let first_type = self.check_expr(&list[0])?;
                    for element in list.iter().skip(1) {
                        let element_type = self.check_expr(element)?;
                        if element_type != first_type {
                            return Err(CompileError::wrong_type_at(
                                first_type.clone(),
                                element_type,
                                expr.span,
                            ));
                        }
                    }

                    Ok(DataType::list_of(first_type))
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

            ExprKind::BuiltinFunctionCall { function, args } => {
                self.check_builtin_call_expr(expr, function, args)
            }

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

    fn check_builtin_call_expr(
        &self,
        expr: &Expr,
        function: &BuiltinFunction,
        args: &[Expr],
    ) -> Result<DataType, CompileError> {
        let arg_types: Vec<DataType> = args
            .iter()
            .map(|arg| self.check_expr(arg))
            .collect::<Result<Vec<DataType>, CompileError>>()?;

        match function {
            BuiltinFunction::Print => {
                // Print can take any type of arguments
                Ok(DataType::Void)
            }
            BuiltinFunction::Assert => {
                // Assert arguments must be bool
                for arg_type in arg_types {
                    if arg_type != DataType::Bool {
                        return Err(CompileError::wrong_type_at(
                            DataType::Bool,
                            arg_type,
                            expr.span,
                        ));
                    }
                }

                Ok(DataType::Void)
            }
            BuiltinFunction::Format => {
                // Format can take any type of arguments
                Ok(DataType::String)
            }
        }
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
                    return Err(CompileError::wrong_type_at(lhs_type, rhs_type, expr.span))
                }
                (_, DataType::Int | DataType::Float) => {
                    return Err(CompileError::wrong_type_at(rhs_type, lhs_type, expr.span))
                }
                _ => {
                    return Err(CompileError::wrong_type_at(
                        DataType::Int,
                        lhs_type,
                        expr.span,
                    ))
                }
            },

            bin_op_pat!(NUMERIC_COMPARISON) => match (&lhs_type, &rhs_type) {
                (DataType::Int, DataType::Int) => Ok(DataType::Bool),
                (DataType::Float, DataType::Float) => Ok(DataType::Bool),
                (DataType::Int | DataType::Float, _) => {
                    return Err(CompileError::wrong_type_at(lhs_type, rhs_type, expr.span))
                }
                (_, DataType::Int | DataType::Float) => {
                    return Err(CompileError::wrong_type_at(rhs_type, lhs_type, expr.span))
                }
                _ => {
                    return Err(CompileError::wrong_type_at(
                        DataType::Int,
                        lhs_type,
                        expr.span,
                    ))
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
                    ))
                }
            },
        }
    }
}
