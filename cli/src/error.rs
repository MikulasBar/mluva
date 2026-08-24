use bincode::error::{DecodeError, EncodeError};
use common::CompileError;

pub enum CliError {
    Compile(CompileError),
    Encode(EncodeError),
    Decode(DecodeError),
    Other(String),
    Handled,
}

impl From<CompileError> for CliError {
    fn from(err: CompileError) -> Self {
        CliError::Compile(err)
    }
}

impl From<EncodeError> for CliError {
    fn from(err: EncodeError) -> Self {
        CliError::Encode(err)
    }
}

impl From<DecodeError> for CliError {
    fn from(err: DecodeError) -> Self {
        CliError::Decode(err)
    }
}

impl From<String> for CliError {
    fn from(err: String) -> Self {
        CliError::Other(err)
    }
}
