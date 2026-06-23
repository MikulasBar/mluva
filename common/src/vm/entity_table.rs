use std::{collections::HashMap, sync::Arc};

use crate::{function::FunctionEntity, module::LCP};



pub struct EntityTable {
    functions: HashMap<String, Arc<FunctionEntity>>,
    pools: Vec<Arc<LCP>>
}

impl EntityTable {
    pub fn empty() -> Self {
        Self {
            functions: HashMap::new(),
            pools: vec![]
        }
    }

    pub fn add_function(&mut self, key: String, entity: FunctionEntity) {
        self.functions.insert(key, Arc::new(entity));
    }

    pub fn get_function(&self, key: &str) -> Option<&FunctionEntity> {
        self.functions.get(key).map(|e| &**e)
    }
}