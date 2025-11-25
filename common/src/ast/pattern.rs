use crate::{ast::Expr, diagnostics::Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
}

impl Pattern {
    pub fn is_declarable(&self) -> bool {
        matches!(self.kind, PatternKind::Variable(_))
    }

    pub fn var(name: String, span: Span) -> Self {
        Self {
            kind: PatternKind::Variable(name),
            span,
        }
    }

    pub fn index(callee: Self, index: Expr, span: Span) -> Self {
        Self {
            kind: PatternKind::Index {
                callee: Box::new(callee),
                index: Box::new(index),
            },
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    Variable(String),
    Index {
        callee: Box<Pattern>,
        index: Box<Expr>,
    },
}
