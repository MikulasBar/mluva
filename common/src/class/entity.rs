use std::{collections::HashMap, sync::Arc};

use bincode::{Decode, Encode};

use crate::{class::FieldInfo, function::FunctionEntity};


#[derive(Debug, Clone, Encode, Decode)]
pub struct ClassEntity {
    fields: Vec<FieldInfo>,
    itables: HashMap<String, ITable>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ITable {
    functions: Vec<Arc<FunctionEntity>>
}