use crate::function::{InternalFunctionDefinition};


#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    FunctionDef(InternalFunctionDefinition),
    Import(String), // Now only support module name as string
}