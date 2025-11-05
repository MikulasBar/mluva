use std::collections::HashMap;

use super::data_type::DataType;
use super::data_type_scope::DataTypeScope;
use crate::ast::{
    Ast, BinaryOp, BuiltinFunction, Expr, ExprKind, Statement, StatementKind, UnaryOp,
};
use crate::bin_op_pat;
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
                .return_type;

            self.check_statements(statements, return_type)?;

            self.scope.exit();
        }

        Ok(())
    }

    fn check_statements(
        &mut self,
        stmts: &[Statement],
        return_type: DataType,
    ) -> Result<(), CompileError> {
        for s in stmts {
            self.check_statement(s, return_type)?;
        }

        Ok(())
    }

    fn check_statement(
        &mut self,
        statement: &Statement,
        return_type: DataType,
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
                data_type,
                variable,
                value,
            } => {
                // if the declaration has explicit type or not
                // check the type if yes
                // if no then do essentialy nothing
                let expr_type = self.check_expr(&value)?;
                let data_type = if let Some(data_type) = data_type {
                    if expr_type != *data_type {
                        return Err(CompileError::wrong_type_at(
                            *data_type,
                            expr_type,
                            statement.span,
                        ));
                    }

                    *data_type
                } else {
                    expr_type
                };

                self.scope
                    .insert_new(variable.clone(), data_type, statement.span)?;
            }

            StatementKind::VarAssign { variable, value } => {
                let Some(&data_type) = self.scope.get(&variable) else {
                    return Err(CompileError::variable_not_found_at(
                        variable.clone(),
                        statement.span,
                    ));
                };

                let value_type = self.check_expr(&value)?;

                if value_type != data_type {
                    return Err(CompileError::wrong_type_at(
                        data_type,
                        value_type,
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
                if expr_type != return_type {
                    return Err(CompileError::wrong_type_at(
                        return_type,
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
            ExprKind::FunctionCall { func_name, args } => {
                let Some(signiture) = self.ast.get_function_signiture(&func_name) else {
                    return Err(CompileError::function_not_found_at(
                        func_name.clone(),
                        expr.span,
                    ));
                };

                let arg_types: Vec<(DataType, Span)> = args
                    .iter()
                    .map(|arg| self.check_expr(arg).map(|dt| (dt, arg.span)))
                    .collect::<Result<Vec<(DataType, Span)>, CompileError>>()?;

                signiture.check_argument_types(&arg_types, expr.span)?;

                Ok(signiture.return_type)
            }

            ExprKind::ForeignFunctionCall {
                module_name,
                func_name,
                args,
            } => {
                let signiture = self
                    .dependencies
                    .get(module_name)
                    .ok_or_else(|| {
                        CompileError::module_not_found_at(module_name.clone(), expr.span)
                    })?
                    .get_function_signiture(&func_name)
                    .ok_or_else(|| {
                        CompileError::function_not_found_at(func_name.clone(), expr.span)
                    })?;

                let arg_types: Vec<(DataType, Span)> = args
                    .iter()
                    .map(|arg| self.check_expr(arg).map(|dt| (dt, arg.span)))
                    .collect::<Result<Vec<(DataType, Span)>, CompileError>>()?;

                signiture.check_argument_types(&arg_types, expr.span)?;

                Ok(signiture.return_type)
            }

            ExprKind::BuiltinFunctionCall { function, args } => {
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

            ExprKind::BinaryOp(op, lhs, rhs) => {
                let lhs_type = self.check_expr(&lhs)?;
                let rhs_type = self.check_expr(&rhs)?;
                match op {
                    bin_op_pat!(NUMERIC) => match (lhs_type, rhs_type) {
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

                    bin_op_pat!(NUMERIC_COMPARISON) => match (lhs_type, rhs_type) {
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

            ExprKind::UnaryOp(op, expr) => {
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
    }
}
