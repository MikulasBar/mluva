use std::sync::Arc;

use bincode::{Decode, Encode};

use crate::{function::FunctionCode, module::LCP};

#[derive(Debug, Clone, Encode, Decode)]
pub struct FunctionEntity {
    kind: FunctionEntityKind,
    pool: Arc<LCP>,
}



#[derive(Debug, Clone, Encode, Decode)]
pub enum FunctionEntityKind {
    Code(FunctionCode),
}