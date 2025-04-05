

#[derive(Debug, Clone)]
pub enum InterpreterError {
    UndefinedVariable(String),
    ValueError,
    TypeError,
}