#[derive(Debug, Clone)]
pub enum InterpreterError {
    ValueStackUnderflow,
    DivisionByZero,
    Unknown,
    
    /// this is for any type error
    /// these errors should be handled in the type checker
    /// but I want to have a fallback for any type error
    TypeError,
}