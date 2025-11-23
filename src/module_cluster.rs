use std::collections::HashMap;

use crate::module::Module;

pub struct ModuleCluster {
    modules: Vec<Module>,
    slot_map: HashMap<String, usize>,
}

impl ModuleCluster {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            slot_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, module: Module) -> usize {
        let slot = self.modules.len();
        self.modules.push(module);
        self.slot_map.insert(name, slot);
        slot
    }

    pub fn get_by_slot(&self, slot: usize) -> Option<&Module> {
        self.modules.get(slot)
    }

    pub fn get_slot(&self, name: &str) -> Option<usize> {
        self.slot_map.get(name).copied()
    }
}
