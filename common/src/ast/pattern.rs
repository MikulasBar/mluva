use crate::{Descriptor, ast::Expr, diagnostics::Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
    pub ty: Option<Descriptor>,
}

impl Pattern {
    pub fn set_type(&mut self, ty: Descriptor) {
        self.ty = Some(ty);
    }

    pub fn is_declarable(&self) -> bool {
        matches!(self.kind, PatternKind::Variable(_))
    }

    pub fn new(kind: PatternKind, span: Span) -> Self {
        Self {
            kind,
            span,
            ty: None,
        }
    }

    pub fn var(name: String, span: Span) -> Self {
        Self::new(PatternKind::Variable(name), span)
    }

    pub fn index(callee: Self, index: Expr, span: Span) -> Self {
        Self::new(
            PatternKind::Index {
                callee: Box::new(callee),
                index,
            },
            span,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    Variable(String),
    Index { callee: Box<Pattern>, index: Expr },
}
