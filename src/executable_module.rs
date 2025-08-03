use crate::function::InternalFunctionSource;

#[derive(Debug, Clone)]
pub struct ExecutableModule {
    pub functions: Vec<InternalFunctionSource>,
    pub main_slot: usize,
}

impl ExecutableModule {
    pub fn new(functions: Vec<InternalFunctionSource>, main_slot: usize) -> Self {
        Self {
            functions,
            main_slot,
        }
    }
}