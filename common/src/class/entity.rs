use std::{collections::HashMap, sync::Arc};

use bincode::{Decode, Encode};

use crate::{class::FieldInfo, function::FunctionEntity, word::Word};


#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ClassEntity {
    fields: Vec<FieldInfo>,
    itables: HashMap<String, ITable>,
}

impl ClassEntity {
    pub fn object_body_words(&self) -> usize {
        self.fields.len()
    }

    pub fn object_body_size(&self) -> usize {
        size_of::<Word>() * self.object_body_words()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct ITable {
    functions: Vec<Arc<FunctionEntity>>
}