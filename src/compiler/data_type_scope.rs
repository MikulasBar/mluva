use std::collections::HashMap;

use super::data_type::DataType;
use crate::errors::CompileError;

pub struct DataTypeScope {
    scopes: Vec<HashMap<String, DataType>>,
}

impl DataTypeScope {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn exit(&mut self) {
        self.scopes.pop();
    }

    pub fn enter(&mut self) {
        self.scopes.push(HashMap::new());
    }
}

impl DataTypeScope {
    pub fn contains(&self, name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
    }

    pub fn insert_new(&mut self, name: String, data_type: DataType) -> Result<(), CompileError> {
        if self.contains(&name) {
            return Err(CompileError::VarRedeclaration(name));
        }

        self.scopes
            .last_mut()
            .expect("There is no scope")
            .insert(name, data_type);

        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&DataType> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }
}
