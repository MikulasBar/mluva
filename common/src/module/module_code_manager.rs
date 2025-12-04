use std::collections::HashMap;

use super::module_code::ModuleCode;

pub struct ModuleCodeManager {
    main_slot: Option<u32>,
    codes: Vec<ModuleCode>,
    slot_map: HashMap<String, u32>,
}

impl ModuleCodeManager {
    pub fn new() -> Self {
        Self {
            main_slot: None,
            codes: Vec::new(),
            slot_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, source: ModuleCode) -> u32 {
        let slot = self.codes.len() as u32;
        self.codes.push(source);
        self.slot_map.insert(name, slot);
        slot
    }

    pub fn get_by_slot(&self, slot: u32) -> Option<&ModuleCode> {
        self.codes.get(slot as usize)
    }

    pub fn get_slot(&self, name: &str) -> Option<u32> {
        self.slot_map.get(name).copied()
    }

    pub fn set_main_slot(&mut self, slot: u32) {
        self.main_slot = Some(slot);
    }

    pub fn get_main_code(&self) -> Option<&ModuleCode> {
        if let Some(slot) = self.main_slot {
            self.codes.get(slot as usize)
        } else {
            None
        }
    }

    pub fn get_string_from_pool(&self, module_slot: u32, string_slot: u32) -> Option<&str> {
        self.codes
            .get(module_slot as usize)?
            .get_string_from_pool(string_slot)
    }

    pub fn get_main_slot(&self) -> Option<u32> {
        let main_source = self.get_main_code()?;
        main_source.get_main_slot()
    }

    pub fn contains_mod(&self, name: &str) -> bool {
        self.slot_map.contains_key(name)
    }
}
