use super::{binary_op::BinaryOp, UnaryOp};
use crate::{ast::BuiltinFunction, diagnostics::Span, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn literal(value: Value, span: Span) -> Self {
        Self {
            kind: ExprKind::Literal(value),
            span,
        }
    }

    pub fn var(name: String, span: Span) -> Self {
        Self {
            kind: ExprKind::Var(name),
            span,
        }
    }

    pub fn binary_op(op: BinaryOp, lhs: Self, rhs: Self, span: Span) -> Self {
        Self {
            kind: ExprKind::new_binary_op(op, lhs, rhs),
            span,
        }
    }

    pub fn unary_op(op: UnaryOp, expr: Self, span: Span) -> Self {
        Self {
            kind: ExprKind::new_unary_op(op, expr),
            span,
        }
    }

    pub fn function_call(func_name: String, args: Vec<Self>, span: Span) -> Self {
        Self {
            kind: ExprKind::FunctionCall { func_name, args },
            span,
        }
    }

    pub fn foreign_function_call(
        module_name: String,
        func_name: String,
        args: Vec<Self>,
        span: Span,
    ) -> Self {
        Self {
            kind: ExprKind::ForeignFunctionCall {
                module_name,
                func_name,
                args,
            },
            span,
        }
    }

    pub fn builtin_function_call(function: BuiltinFunction, args: Vec<Self>, span: Span) -> Self {
        Self {
            kind: ExprKind::BuiltinFunctionCall { function, args },
            span,
        }
    }

    pub fn method_call(callee: Self, method_name: String, args: Vec<Self>, span: Span) -> Self {
        Self {
            kind: ExprKind::MethodCall {
                callee: Box::new(callee),
                method_name,
                args,
            },
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Literal(Value),
    Var(String),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    FunctionCall {
        func_name: String,
        args: Vec<Expr>,
    },
    ForeignFunctionCall {
        module_name: String,
        func_name: String,
        args: Vec<Expr>,
    },
    BuiltinFunctionCall {
        function: BuiltinFunction,
        args: Vec<Expr>,
    },
    MethodCall {
        callee: Box<Expr>,
        method_name: String,
        args: Vec<Expr>,
    },
}

impl ExprKind {
    pub fn new_binary_op(op: BinaryOp, lhs: Expr, rhs: Expr) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_unary_op(op: UnaryOp, expr: Expr) -> Self {
        Self::UnaryOp(op, Box::new(expr))
    }
}
