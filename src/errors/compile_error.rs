use std::fmt;

use codespan_reporting::{
    diagnostic::{Diagnostic, Label, Severity},
    files::SimpleFiles,
};

use crate::{
    compiler::{data_type::DataType, token::Token},
    diagnostics::Span,
};

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

    // Convenience constructors
    pub fn unexpected_char(ch: char) -> Self {
        Self::new(
            CompileErrorKind::UnexpectedChar(ch),
            format!("unexpected character '{}'", ch),
        )
    }

    pub fn unexpected_char_at(ch: char, span: Span) -> Self {
        Self::unexpected_char(ch).with_span(span)
    }

    pub fn unterminated_string() -> Self {
        Self::new(
            CompileErrorKind::UnterminatedString,
            "unterminated string literal",
        )
    }

    pub fn unterminated_string_at(span: Span) -> Self {
        Self::unterminated_string().with_span(span)
    }

    pub fn unexpected_end_of_file() -> Self {
        Self::new(
            CompileErrorKind::UnexpectedEndOfFile,
            "unexpected end of file",
        )
    }

    pub fn unexpected_end_of_file_at(span: Span) -> Self {
        Self::unexpected_end_of_file().with_span(span)
    }

    pub fn invalid_number() -> Self {
        Self::new(CompileErrorKind::InvalidNumber, "invalid numeric literal")
    }

    pub fn invalid_number_at(span: Span) -> Self {
        Self::invalid_number().with_span(span)
    }

    pub fn other(msg: impl Into<String>) -> Self {
        Self::new(CompileErrorKind::Other, msg)
    }

    /// Convert into a codespan_reporting::diagnostic::Diagnostic using the supplied files map.
    /// Caller provides the `SimpleFiles<String, String>` instance that holds the source text(s).
    pub fn to_diagnostic(&self, files: &SimpleFiles<String, String>) -> Diagnostic<usize> {
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
    UnexpectedToken(Token),
    UnterminatedString,
    InvalidNumber,
    UnexpectedEndOfFile,
    WrongType { expected: DataType, found: DataType },
    WrongNumberOfArguments { expected: usize, found: usize },
    VariableNotFound(String),
    FunctionNotFound(String),
    FunctionAlreadyDefined(String),
    VarRedeclaration(String),
    ModuleNotFound(String),
    UnknownForeignFunction { module: String, name: String },
    ReservedFunctionName(String),
    Other,
}
