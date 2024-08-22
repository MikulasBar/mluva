
use std::{collections::HashMap, hash::Hash};

use crate::data_type::{self, DataType, DataTypeMap};


pub type DataTypeScope = Scope<DataTypeMap>;


pub struct Scope<T> {
    scopes: Vec<T>,
}

impl<T> Scope<T> {
    pub fn new() -> Self {
        Self {
            scopes: vec![],
        }
    }
    
    pub fn exit(&mut self) {
        self.scopes.pop();
    }
}

impl<T: Default> Scope<T> {
    pub fn enter(&mut self) {
        self.scopes.push(T::default());
    }
}


impl Scope<DataTypeMap> {
    pub fn insert(&mut self, ident: String, data_type: DataType) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(ident, data_type);
        }
    }

    pub fn get(&self, ident: &str) -> Option<&DataType> {
        for scope in self.scopes.iter().rev() {
            if let Some(data_type) = scope.get(ident) {
                return Some(data_type);
            }
        }
        None
    }    
}
