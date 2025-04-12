use crate::{function_source::FunctionSource, instruction::Instruction};



#[derive(Debug, Clone)]
pub struct InterpreterSource {
    pub functions: Vec<FunctionSource>,
    pub main_slot: Option<usize>,
}

impl InterpreterSource {
    pub fn new(functions: Vec<FunctionSource>, main_slot: Option<usize>) -> Self {
        Self {
            functions,
            main_slot,
        }
    }
}