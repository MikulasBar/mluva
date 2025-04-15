use crate::function::InternalFunctionDefinition;


#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    FnDef(InternalFunctionDefinition),
}