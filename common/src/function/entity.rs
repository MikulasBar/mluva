use std::sync::Arc;

use bincode::{Decode, Encode};

use crate::function::FunctionCode;

#[derive(Debug, Clone, Encode, Decode)]
pub struct FunctionEntity {
    kind: FunctionEntityKind,
    parent_module: Arc<ModuleEntity>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum FunctionEntityKind {
    Code(FunctionCode),
}