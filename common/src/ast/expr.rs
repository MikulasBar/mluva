use super::{UnaryOp, binary_op::BinaryOp};
use crate::diagnostics::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn void_literal(span: Span) -> Self {
        Self {
            kind: ExprKind::VoidLiteral,
            span,
        }
    }

    pub fn int_literal(value: i32, span: Span) -> Self {
        Self {
            kind: ExprKind::IntLiteral(value),
            span,
        }
    }

    pub fn float_literal(value: f32, span: Span) -> Self {
        Self {
            kind: ExprKind::FloatLiteral(value),
            span,
        }
    }

    pub fn bool_literal(value: bool, span: Span) -> Self {
        Self {
            kind: ExprKind::BoolLiteral(value),
            span,
        }
    }

    pub fn string_literal(value: String, span: Span) -> Self {
        Self {
            kind: ExprKind::StringLiteral(value),
            span,
        }
    }

    pub fn list_literal(elements: Vec<Self>, span: Span) -> Self {
        Self {
            kind: ExprKind::ListLiteral(elements),
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
            kind: ExprKind::BinaryOp(op, Box::new(lhs), Box::new(rhs)),
            span,
        }
    }

    pub fn unary_op(op: UnaryOp, expr: Self, span: Span) -> Self {
        Self {
            kind: ExprKind::UnaryOp(op, Box::new(expr)),
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

    pub fn index_get(callee: Self, index: Self, span: Span) -> Self {
        Self {
            kind: ExprKind::IndexGet {
                callee: Box::new(callee),
                index: Box::new(index),
            },
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    VoidLiteral,
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    StringLiteral(String),
    ListLiteral(Vec<Expr>),
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
    MethodCall {
        callee: Box<Expr>,
        method_name: String,
        args: Vec<Expr>,
    },
    IndexGet {
        callee: Box<Expr>,
        index: Box<Expr>,
    },
}
