use crate::function::FunctionSource;

#[derive(Debug, Clone)]
pub struct InterpreterSource {
    pub functions: Vec<FunctionSource>,
    pub main_slot: usize,
}

impl InterpreterSource {
    pub fn new(functions: Vec<FunctionSource>, main_slot: usize) -> Self {
        Self {
            functions,
            main_slot,
        }
    }
}