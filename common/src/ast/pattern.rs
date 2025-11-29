use crate::{ast::Expr, diagnostics::Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
    pub type_id: Option<u32>,
}

impl Pattern {
    pub fn set_type(&mut self, type_id: u32) {
        self.type_id = Some(type_id);
    }

    pub fn is_declarable(&self) -> bool {
        matches!(self.kind, PatternKind::Variable(_))
    }

    pub fn var(name: String, span: Span) -> Self {
        Self {
            kind: PatternKind::Variable(name),
            span,
            type_id: None,
        }
    }

    pub fn index(callee: Self, index: Expr, span: Span) -> Self {
        Self {
            kind: PatternKind::Index {
                callee: Box::new(callee),
                index: index,
            },
            span,
            type_id: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    Variable(String),
    Index { callee: Box<Pattern>, index: Expr },
}
