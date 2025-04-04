mod interpreter_error;
mod parse_error;
mod type_check_error;

pub use interpreter_error::InterpreterError;
pub use parse_error::ParseError;
pub use type_check_error::TypeCheckError;