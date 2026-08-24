use std::fmt;

use codespan_reporting::diagnostic::{Diagnostic, Label, Severity};

use crate::Descriptor;
use crate::diagnostics::{FileId, Span};
use crate::token::TokenKind;

/// CLI / top-level runner can convert this into a codespan_reporting Diagnostic
/// using `to_diagnostic` and then render it.
#[derive(Debug, Clone)]
pub struct CompileError {
    pub kind: CompileErrorKind,
    pub message: String,
    pub span: Option<Span>,
    pub notes: Vec<String>,
}

impl CompileError {
    pub fn new(kind: CompileErrorKind, message: impl Into<String>) -> Self {
        CompileError {
            kind,
            message: message.into(),
            span: None,
            notes: Vec::new(),
        }
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn add_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    pub fn unexpected_char_at(ch: char, span: Span) -> Self {
        Self::new(
            CompileErrorKind::UnexpectedChar(ch),
            format!("unexpected character '{}'", ch),
        )
        .with_span(span)
    }

    pub fn unterminated_string_at(span: Span) -> Self {
        Self::new(
            CompileErrorKind::UnterminatedString,
            "unterminated string literal",
        )
        .with_span(span)
    }

    pub fn unexpected_end_of_file(file: FileId) -> Self {
        Self::new(
            CompileErrorKind::UnexpectedEndOfFile,
            "unexpected end of file",
        )
        .with_span(Span::new(file, 0, 0))
    }

    pub fn reserved_function_at(function: Descriptor, span: Span) -> Self {
        Self::new(
            CompileErrorKind::ReservedFunction(function.clone()),
            format!("use of reserved function name {}", function),
        )
        .with_span(span)
    }

    pub fn unexpected_token_at(token_kind: TokenKind, span: Span) -> Self {
        Self::new(
            CompileErrorKind::UnexpectedToken(token_kind.clone()),
            format!("unexpected token {}", token_kind),
        )
        .with_span(span)
    }

    pub fn wrong_type_at(
        expected: Descriptor,
        found: Descriptor,
        span: Span,
    ) -> Self {
        let message = format!(
            "wrong type: expected {}, found {}",
            expected,
            found
        );
        Self::new(CompileErrorKind::WrongType { expected, found }, message).with_span(span)
    }

    pub fn variable_not_found_at(name: impl Into<String> + Clone, span: Span) -> Self {
        Self::new(
            CompileErrorKind::VariableNotFound(name.clone().into()),
            format!("variable not found: {}", name.into()),
        )
        .with_span(span)
    }

    pub fn function_not_found_at(function: Descriptor, span: Span) -> Self {
        Self::new(
            CompileErrorKind::FunctionNotFound(function.clone()),
            format!("function not found: {}", function),
        )
        .with_span(span)
    }

    pub fn module_not_found_at(module: Descriptor, span: Span) -> Self {
        Self::new(
            CompileErrorKind::ModuleNotFound(module.clone()),
            format!("module not found: {}", module),
        )
        .with_span(span)
    }

    pub fn variable_redeclaration_at(name: impl Into<String> + Clone, span: Span) -> Self {
        Self::new(
            CompileErrorKind::VarRedeclaration(name.clone().into()),
            format!("variable redeclaration: {}", name.into()),
        )
        .with_span(span)
    }

    pub fn wrong_number_of_arguments_at(expected: usize, found: usize, span: Span) -> Self {
        Self::new(
            CompileErrorKind::WrongNumberOfArguments { expected, found },
            format!(
                "wrong number of arguments: expected {}, found {}",
                expected, found
            ),
        )
        .with_span(span)
    }

    pub fn unknown_type_at(ty: Descriptor, span: Span) -> Self {
        Self::new(
            CompileErrorKind::UnknownType(ty.clone()),
            format!("unknown type: {}", ty),
        )
        .with_span(span)
    }

    pub fn cannot_infer_type_at(span: Span) -> Self {
        Self::new(
            CompileErrorKind::CannotInferType,
            format!("cannot infer type"),
        )
        .with_span(span)
    }

    pub fn invalid_pattern_at(span: Span) -> Self {
        Self::new(
            CompileErrorKind::InvalidPattern,
            "invalid pattern".to_string(),
        )
        .with_span(span)
    }

    pub fn invalid_indexing_at(span: Span) -> Self {
        Self::new(
            CompileErrorKind::InvalidIndexing,
            "invalid indexing operation".to_string(),
        )
        .with_span(span)
    }

    pub fn invalid_literal_at(span: Span) -> Self {
        Self::new(
            CompileErrorKind::InvalidLiteral,
            "invalid literal".to_string(),
        )
        .with_span(span)
    }

    pub fn internal_compiler_error(message: impl Into<String>) -> Self {
        Self::new(CompileErrorKind::InternalCompilerError, message.into())
    }

    pub fn other_at(msg: impl Into<String>, span: Span) -> Self {
        Self::new(CompileErrorKind::Other, msg).with_span(span)
    }

    pub fn other(msg: impl Into<String>) -> Self {
        Self::new(CompileErrorKind::Other, msg)
    }

    /// Convert into a codespan_reporting::diagnostic::Diagnostic using the supplied files map.
    /// Caller provides the `SimpleFiles<String, String>` instance that holds the source text(s).
    pub fn to_diagnostic(&self) -> Diagnostic<usize> {
        let mut diag = Diagnostic::new(Severity::Error).with_message(self.message.clone());

        if let Some(span) = &self.span {
            let label = Label::primary(span.file, span.lo..span.hi);
            diag = diag.with_labels(vec![label]);
        }

        if !self.notes.is_empty() {
            diag = diag.with_notes(self.notes.clone());
        }

        diag
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(span) = &self.span {
            write!(
                f,
                "At file {}:{}-{}\n{}",
                span.file, span.lo, span.hi, self.message
            )
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for CompileError {}

#[derive(Debug, Clone)]
pub enum CompileErrorKind {
    UnexpectedChar(char),
    UnexpectedToken(TokenKind),
    UnterminatedString,
    UnexpectedEndOfFile,
    WrongType {
        expected: Descriptor,
        found: Descriptor,
    },
    WrongNumberOfArguments {
        expected: usize,
        found: usize,
    },
    UnknownType(Descriptor),
    VariableNotFound(String),
    FunctionNotFound(Descriptor),
    FunctionAlreadyDefined(Descriptor),
    VarRedeclaration(String),
    ModuleNotFound(Descriptor),
    ReservedFunction(Descriptor),
    CannotInferType,
    InvalidPattern,
    InvalidIndexing,
    InvalidLiteral,
    InternalCompilerError,
    Other,
}
