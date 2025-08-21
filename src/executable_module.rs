use crate::function::InternalFunctionSource;

#[derive(Debug, Clone)]
pub struct ExecutableModule {
    pub functions: Vec<InternalFunctionSource>,
    pub main_slot: u32,
}

impl ExecutableModule {
    pub fn new(functions: Vec<InternalFunctionSource>, main_slot: u32) -> Self {
        Self {
            functions,
            main_slot,
        }
    }
}