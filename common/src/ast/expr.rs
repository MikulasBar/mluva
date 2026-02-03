use super::{UnaryOp, binary_op::BinaryOp};
use crate::{Type, diagnostics::Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub ty: Option<Type>,
}

impl Expr {
    pub fn set_type(&mut self, ty: Type) {
        self.ty = Some(ty);
    }

    pub fn new(kind: ExprKind, span: Span) -> Self {
        Self {
            kind,
            span,
            ty: None,
        }
    }

    pub fn void_literal(span: Span) -> Self {
        Self::new(ExprKind::VoidLiteral, span)
    }

    pub fn i32_literal(value: i32, span: Span) -> Self {
        Self::new(ExprKind::I32Literal(value), span)
    }

    pub fn f32_literal(value: f32, span: Span) -> Self {
        Self::new(ExprKind::F32Literal(value), span)
    }

    pub fn bool_literal(value: bool, span: Span) -> Self {
        Self::new(ExprKind::BoolLiteral(value), span)
    }

    pub fn string_literal(value: String, span: Span) -> Self {
        Self::new(ExprKind::StringLiteral(value), span)
    }

    pub fn array_literal(list: Vec<Self>, span: Span) -> Self {
        Self::new(ExprKind::ArrayLiteral(list), span)
    }

    pub fn var(name: String, span: Span) -> Self {
        Self::new(ExprKind::Var(name), span)
    }

    pub fn binary_op(op: BinaryOp, lhs: Self, rhs: Self, span: Span) -> Self {
        Self::new(ExprKind::BinaryOp(op, Box::new(lhs), Box::new(rhs)), span)
    }

    pub fn unary_op(op: UnaryOp, expr: Self, span: Span) -> Self {
        Self::new(ExprKind::UnaryOp(op, Box::new(expr)), span)
    }

    pub fn function_call(func_name: String, args: Vec<Self>, span: Span) -> Self {
        Self::new(ExprKind::FunctionCall { func_name, args }, span)
    }

    pub fn foreign_function_call(
        module_name: String,
        func_name: String,
        args: Vec<Self>,
        span: Span,
    ) -> Self {
        Self::new(
            ExprKind::ForeignFunctionCall {
                module_name,
                func_name,
                args,
            },
            span,
        )
    }

    pub fn method_call(callee: Self, method_name: String, args: Vec<Self>, span: Span) -> Self {
        Self::new(
            ExprKind::MethodCall {
                callee: Box::new(callee),
                method_name,
                args,
            },
            span,
        )
    }

    pub fn index_get(callee: Self, index: Self, span: Span) -> Self {
        Self::new(
            ExprKind::IndexGet {
                callee: Box::new(callee),
                index: Box::new(index),
            },
            span,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    VoidLiteral,
    I32Literal(i32),
    F32Literal(f32),
    BoolLiteral(bool),
    StringLiteral(String),
    ArrayLiteral(Vec<Expr>),
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
