use super::{UnaryOp, BinaryOp};
use crate::{Descriptor, diagnostics::Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub ty: Option<Descriptor>,
}

impl Expr {
    pub fn set_type(&mut self, ty: Descriptor) {
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

    pub fn var(name: Descriptor, span: Span) -> Self {
        Self::new(ExprKind::Var(name), span)
    }

    pub fn binary_op(op: BinaryOp, lhs: Self, rhs: Self, span: Span) -> Self {
        Self::new(ExprKind::BinaryOp(op, Box::new(lhs), Box::new(rhs)), span)
    }

    pub fn unary_op(op: UnaryOp, expr: Self, span: Span) -> Self {
        Self::new(ExprKind::UnaryOp(op, Box::new(expr)), span)
    }

    pub fn function_call(function: Descriptor, args: Vec<Self>, span: Span) -> Self {
        Self::new(ExprKind::FunctionCall { function, args }, span)
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
    Var(Descriptor),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    FunctionCall {
        function: Descriptor,
        args: Vec<Expr>,
    },
}
