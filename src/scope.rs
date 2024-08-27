use std::{collections::HashMap, hash::Hash};

use crate::value::Value;

use super::data_type::{self, DataType, DataTypeMap};

pub type DataTypeScope = Scope<DataTypeMap>;
pub type MemoryScope = Scope<HashMap<String, Value>>;

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


impl<K, V> Scope<HashMap<K, V>>
where
    K: Eq + Hash
{
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(key, value);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }    
}
