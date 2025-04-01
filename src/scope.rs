use core::panic;
use std::{collections::HashMap, hash::Hash, ops::Index};

use crate::value::Value;

use super::data_type::DataType;

pub type DataTypeScope = Scope<HashMap<String, DataType>>;
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
    K: Eq + Hash + Clone
{
    pub fn insert_new(&mut self, key: K, value: V) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(key, value);
        }
    }

    pub fn change(&mut self, key: &K, value: V) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(key) {
                scope.insert(key.clone(), value);
                return;
            }
        }

        panic!()
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

impl<K, V> Index<&K> for Scope<HashMap<K, V>>
where
    K: Eq + Hash + Clone
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).unwrap()
    }
} 
