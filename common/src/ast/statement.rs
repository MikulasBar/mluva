use super::expr::Expr;
use crate::{Descriptor, ast::pattern::Pattern, diagnostics::Span};

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub span: Span,
}

impl Statement {
    pub fn new(kind: StatementKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn var_assign(assignee: Pattern, value: Expr, span: Span) -> Self {
        Self {
            kind: StatementKind::VarAssign { assignee, value },
            span,
        }
    }

    pub fn var_declare(ty: Option<Descriptor>, assignee: Pattern, value: Expr, span: Span) -> Self {
        Self {
            kind: StatementKind::VarDeclare {
                assignee_ty: ty,
                assignee,
                value,
            },
            span,
        }
    }

    pub fn if_statement(
        condition: Expr,
        if_block: Vec<Self>,
        else_block: Option<Vec<Self>>,
        span: Span,
    ) -> Self {
        Self {
            kind: StatementKind::If {
                condition,
                if_block,
                else_block,
            },
            span,
        }
    }

    pub fn while_statement(condition: Expr, block: Vec<Self>, span: Span) -> Self {
        Self {
            kind: StatementKind::While { condition, block },
            span,
        }
    }

    pub fn expr_statement(expr: Expr, span: Span) -> Self {
        Self {
            kind: StatementKind::Expr(expr),
            span,
        }
    }

    pub fn return_statement(expr: Option<Expr>, span: Span) -> Self {
        Self {
            kind: StatementKind::Return(expr),
            span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    VarAssign {
        assignee: Pattern,
        value: Expr,
    },
    VarDeclare {
        assignee_ty: Option<Descriptor>,
        assignee: Pattern,
        value: Expr,
    },
    If {
        condition: Expr,
        if_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
    While {
        condition: Expr,
        block: Vec<Statement>,
    },
    Expr(Expr),
    Return(Option<Expr>),
}
