use std::collections::HashMap;

use super::FunctionDefinition;


pub struct FunctionDefinitionTable {
    map: HashMap<String, usize>,
    functions: Vec<FunctionDefinition>
}

impl FunctionDefinitionTable {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            functions: Vec::new(),
        }
    }

    pub fn insert(&mut self, def: FunctionDefinition) {
        // We need to use the to_string here
        // because it is pushed so the reference would be invalid
        let name = def.name().to_string();
        if self.map.contains_key(&name) {
            panic!("Function {} already exists", name);
        }

        let index = self.functions.len();
        self.functions.push(def);
        self.map.insert(name.to_string(), index);
    }

    pub fn get_slot(&self, name: &str) -> Option<usize> {
        self.map.get(name).copied()
    }

    pub fn get_fn_by_name(&self, name: &str) -> Option<&FunctionDefinition> {
        self.map.get(name).and_then(|&index| self.functions.get(index))
    }

    pub fn get_fn_by_index(&self, index: usize) -> Option<&FunctionDefinition> {
        if index >= self.functions.len() {
            return None;
        }

        Some(&self.functions[index])
    }
}