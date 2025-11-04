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
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Literal(Value),
    Var(String),
    BinaryOp(BinaryOp, Box<Self>, Box<Self>),
    UnaryOp(UnaryOp, Box<Self>),
    FunctionCall(String, Vec<ExprKind>),
    ForeignFunctionCall {
        module_name: String,
        func_name: String,
        args: Vec<ExprKind>,
    },
    BuiltinFunctionCall {
        function: BuiltinFunction,
        args: Vec<ExprKind>,
    },
}

impl ExprKind {
    pub fn new_binary_op(op: BinaryOp, lhs: Self, rhs: Self) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_unary_op(op: UnaryOp, expr: Self) -> Self {
        Self::UnaryOp(op, Box::new(expr))
    }
}
