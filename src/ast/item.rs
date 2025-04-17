use crate::function::{ExternalFunctionDefinition, InternalFunctionDefinition};


#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    FunctionDef(InternalFunctionDefinition),
    ExternalFunctionDef(ExternalFunctionDefinition),
}