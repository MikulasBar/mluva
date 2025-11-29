use std::collections::HashMap;

use super::module_source::ModuleSource;

pub struct ModuleManager {
    main_slot: Option<usize>,
    sources: Vec<ModuleSource>,
    slot_map: HashMap<String, usize>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            main_slot: None,
            sources: Vec::new(),
            slot_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, source: ModuleSource) -> usize {
        let slot = self.sources.len();
        self.sources.push(source);
        self.slot_map.insert(name, slot);
        slot
    }

    pub fn get_by_slot(&self, slot: usize) -> Option<&ModuleSource> {
        self.sources.get(slot)
    }

    pub fn get_slot(&self, name: &str) -> Option<usize> {
        self.slot_map.get(name).copied()
    }

    pub fn set_main_slot(&mut self, slot: usize) {
        self.main_slot = Some(slot);
    }

    pub fn get_main_source_slot(&self) -> Option<usize> {
        self.main_slot
    }

    pub fn get_main_source(&self) -> Option<&ModuleSource> {
        if let Some(slot) = self.main_slot {
            self.sources.get(slot)
        } else {
            None
        }
    }

    pub fn get_string_from_pool(&self, module_slot: usize, string_slot: usize) -> Option<&str> {
        self.sources
            .get(module_slot)?
            .get_string_from_pool(string_slot)
    }
}
