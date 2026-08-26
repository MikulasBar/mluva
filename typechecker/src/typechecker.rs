use std::collections::HashMap;
use std::mem;

use super::type_scope::TypeScope;
use crate::bin_op_pat;
use common::CompileError;
use common::ast::{BinaryOp, Expr, ExprKind, Statement, StatementKind, UnaryOp};
use common::diagnostics::Span;
use common::module::ModuleAST;
use common::module::ModuleSigniture;
use common::{Descriptor, descriptor};

pub struct TypeChecker<'a> {
    ast: &'a mut ModuleAST,
    dependencies: &'a HashMap<Descriptor, ModuleSigniture>,
    scope: TypeScope,
    import_map: HashMap<String, Descriptor>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(
        ast: &'a mut ModuleAST,
        dependencies: &'a HashMap<Descriptor, ModuleSigniture>,
    ) -> Self {
        let import_map = ast
            .imports()
            .iter()
            .map(|(desc, _)| (desc.last().unwrap().to_string(), desc.clone()))
            .collect();

        Self {
            ast,
            dependencies,
            import_map,
            scope: TypeScope::new(),
        }
    }

    pub fn check(mut self) -> Result<(), CompileError> {
        self.check_imports()?;
        self.check_functions()?;

        Ok(())
    }

    fn check_imports(&mut self) -> Result<(), CompileError> {
        for (path, span) in self.ast.imports() {
            if !self.dependencies.contains_key(path) {
                return Err(CompileError::module_not_found_at(path.clone(), *span));
            }
        }

        Ok(())
    }

    fn check_functions(&mut self) -> Result<(), CompileError> {
        for name in self.ast.function_names() {
            self.scope.enter();

            let signiture = self.ast.get_function_sig(&name).unwrap();

            signiture.params.iter().try_for_each(|p| {
                self.scope
                    .insert_new_var(p.name.clone(), p.ty.clone(), p.span)
            })?;

            let return_type = signiture.return_type.clone();

            // Take the body out to avoid borrowing issues
            // This shouldn't break anything, because we do this only once,
            // so no one else will use the body when we have it taken out
            let mut statements = mem::take(self.ast.get_function_body_mut(&name).unwrap());

            self.check_statements(&mut statements, &return_type)?;

            let body = self.ast.get_function_body_mut(&name).unwrap();
            *body = statements;

            self.scope.exit();
        }

        Ok(())
    }

    fn check_statements(
        &mut self,
        statements: &mut [Statement],
        return_type: &Descriptor,
    ) -> Result<(), CompileError> {
        for statement in statements {
            self.check_statement(statement, return_type)?;
        }

        Ok(())
    }

    fn check_statement(
        &mut self,
        statement: &mut Statement,
        return_type: &Descriptor,
    ) -> Result<(), CompileError> {
        match &mut statement.kind {
            StatementKind::If {
                condition,
                if_block,
                else_block,
            } => {
                let cond = self.check_expr(condition)?;
                if !cond.is_bool_type() {
                    return Err(CompileError::wrong_type_at(
                        Descriptor::bool_type(),
                        cond,
                        statement.span,
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
                        ));
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
                    ));
                }
            }

            StatementKind::While { condition, block } => {
                let cond = self.check_expr(condition)?;
                if !cond.is_bool_type() {
                    return Err(CompileError::wrong_type_at(
                        Descriptor::bool_type(),
                        cond,
                        statement.span,
                    ));
                }

                return self.check_statements(block, return_type);
            }

            StatementKind::Expr(expr) => {
                self.check_expr(expr)?;
            }

            StatementKind::Return(expr) => match (expr, return_type) {
                (Some(e), r) => {
                    let e = self.check_expr(e)?;

                    if !e.matches(r) {
                        return Err(CompileError::wrong_type_at(e, r.clone(), statement.span));
                    }
                }
                (None, r) if !r.is_void_type() => {
                    return Err(CompileError::wrong_type_at(
                        Descriptor::void_type(),
                        r.clone(),
                        statement.span,
                    ));
                }
                _ => (),
            },
        }

        Ok(())
    }

    fn check_expr(&self, expr: &mut Expr) -> Result<Descriptor, CompileError> {
        let expr_ty = match &mut expr.kind {
            ExprKind::Path(ident) => {
                // TODO: solve other paths that are not variables
                let ident = ident.last().unwrap();
                let Some(ty) = self.scope.get(&ident) else {
                    return Err(CompileError::variable_not_found_at(ident, expr.span));
                };

                ty.clone()
            }
            ExprKind::I32Literal(_) => Descriptor::i32_type(),
            ExprKind::F32Literal(_) => Descriptor::f32_type(),
            ExprKind::BoolLiteral(_) => Descriptor::bool_type(),
            ExprKind::StringLiteral(_) => todo!(),
            ExprKind::ArrayLiteral(_) => todo!(),

            ExprKind::FunctionCall { function, args } => {
                self.check_call_expr(expr.span, function, args)?
            }

            ExprKind::BinaryOp(op, lhs, rhs) => {
                self.check_binary_op_expr(expr.span, op, lhs, rhs)?
            }
            ExprKind::UnaryOp(op, expr) => self.check_unary_op_expr(expr, op)?,
        };

        expr.ty = Some(expr_ty.clone());

        Ok(expr_ty)
    }

    // TODO: optimize this
    fn check_call_expr(
        &self,
        span: Span,
        function: &mut Descriptor,
        args: &mut [Expr],
    ) -> Result<Descriptor, CompileError> {
        let sig = if function.len() == 1 {
            let Some(sig) = self.ast.get_function_sig(function.first().unwrap()) else {
                return Err(CompileError::function_not_found_at(function.clone(), span));
            };

            sig
        } else {
            let Some(import) = self.import_map.get(function.first().unwrap()) else {
                return Err(CompileError::module_not_found_at(function.clone(), span));
            };

            function.sub_first(import.segments.clone());

            let mut path = function.clone();
            let tail = path.pop_last_unchecked();

            let Some(module) = self.dependencies.get(&path) else {
                return Err(CompileError::module_not_found_at(path, span));
            };

            let Some(sig) = module.get_function(&tail) else {
                return Err(CompileError::function_not_found_at(function.clone(), span));
            };

            sig
        };

        let arg_types: Vec<(Descriptor, Span)> = args
            .iter_mut()
            .map(|arg| self.check_expr(arg).map(|t| (t, arg.span)))
            .collect::<Result<Vec<(Descriptor, Span)>, CompileError>>()?;

        sig.check_argument_types(&arg_types, span)?;

        Ok(sig.return_type.clone())
    }

    fn check_binary_op_expr(
        &self,
        span: Span,
        op: &BinaryOp,
        lhs: &mut Expr,
        rhs: &mut Expr,
    ) -> Result<Descriptor, CompileError> {
        let lhs_type = self.check_expr(lhs)?;
        let rhs_type = self.check_expr(rhs)?;
        match op {
            bin_op_pat!(NUMERIC) => {
                if !lhs_type.is_i32_type() && !lhs_type.is_f32_type() {
                    return Err(CompileError::wrong_type_at(
                        descriptor!("numeric"),
                        lhs_type,
                        span,
                    ));
                }

                if rhs_type != lhs_type {
                    return Err(CompileError::wrong_type_at(lhs_type, rhs_type, span));
                }

                Ok(lhs_type)
            }

            bin_op_pat!(NUMERIC_COMPARISON) => {
                if !lhs_type.is_i32_type() && !lhs_type.is_f32_type() {
                    return Err(CompileError::wrong_type_at(
                        descriptor!("numeric"),
                        lhs_type,
                        span,
                    ));
                }

                if rhs_type != lhs_type {
                    return Err(CompileError::wrong_type_at(lhs_type, rhs_type, span));
                }

                Ok(Descriptor::bool_type())
            }

            bin_op_pat!(COMPARISON) => {
                if lhs_type != rhs_type {
                    return Err(CompileError::wrong_type_at(lhs_type, rhs_type, span));
                }

                Ok(Descriptor::bool_type())
            }

            bin_op_pat!(LOGICAL) => {
                if !lhs_type.is_bool_type() {
                    return Err(CompileError::wrong_type_at(
                        Descriptor::bool_type(),
                        lhs_type,
                        span,
                    ));
                }

                if !rhs_type.is_bool_type() {
                    return Err(CompileError::wrong_type_at(
                        Descriptor::bool_type(),
                        rhs_type,
                        span,
                    ));
                }

                Ok(Descriptor::bool_type())
            }
        }
    }

    fn check_unary_op_expr(
        &self,
        expr: &mut Expr,
        op: &UnaryOp,
    ) -> Result<Descriptor, CompileError> {
        let expr_type = self.check_expr(expr)?;
        match op {
            UnaryOp::Not => {
                if !expr_type.is_bool_type() {
                    return Err(CompileError::wrong_type_at(
                        Descriptor::bool_type(),
                        expr_type,
                        expr.span,
                    ));
                }

                Ok(Descriptor::bool_type())
            }

            UnaryOp::Negate => match expr_type {
                t if t.is_i32_type() => Ok(Descriptor::i32_type()),
                t if t.is_f32_type() => Ok(Descriptor::f32_type()),
                _ => {
                    return Err(CompileError::wrong_type_at(
                        descriptor!("numeric"),
                        expr_type,
                        expr.span,
                    ));
                }
            },
        }
    }
}
