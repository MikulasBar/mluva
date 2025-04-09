use std::collections::HashMap;

use crate::external::ExternalFunction;




pub struct FunctionTable {
    map: HashMap<String, usize>,
    functions: Vec<ExternalFunction>
}

impl FunctionTable {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            functions: Vec::new(),
        }
    }

    pub fn insert(&mut self, function: ExternalFunction) {
        let name = function.name.to_string();
        if self.map.contains_key(&name) {
            panic!("Function {} already exists", name);
        }

        let index = self.functions.len();
        self.functions.push(function);
        self.map.insert(name, index);
    }

    pub fn get_slot(&self, name: &str) -> Option<usize> {
        self.map.get(name).copied()
    }

    pub fn get_fn(&self, name: &str) -> Option<&ExternalFunction> {
        self.map.get(name).and_then(|&index| self.functions.get(index))
    }

    pub fn get_fn_by_index(&self, index: usize) -> Option<&ExternalFunction> {
        if index >= self.functions.len() {
            return None;
        }

        Some(&self.functions[index])
    }
}