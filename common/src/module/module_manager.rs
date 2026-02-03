use std::collections::HashMap;

use crate::module::{Module};

use super::module_code::ModuleCode;

pub struct ModuleManager {
    main_slot: Option<u32>,
    codes: Vec<Module>,
    slot_map: HashMap<String, u32>,
}

impl ModuleManager {
    pub fn new() -> Self {
        Self {
            main_slot: None,
            codes: Vec::new(),
            slot_map: HashMap::new(),
        }
    }

    pub fn add_compiled(&mut self, name: String, code: ModuleCode) -> u32 {
        let slot = self.codes.len() as u32;
        self.codes.push(Module::Code(code));
        self.slot_map.insert(name, slot);
        slot
    }

    pub fn get_by_slot(&self, slot: u32) -> Option<&Module> {
        self.codes.get(slot as usize)
    }

    pub fn get_slot(&self, name: &str) -> Option<u32> {
        self.slot_map.get(name).copied()
    }

    pub fn set_main_slot(&mut self, slot: u32) {
        self.main_slot = Some(slot);
    }

    pub fn get_main_code(&self) -> Option<&Module> {
        if let Some(slot) = self.main_slot {
            self.codes.get(slot as usize)
        } else {
            None
        }
    }

    pub fn get_string_from_pool(&self, module_slot: u32, string_slot: u32) -> Option<&str> {
        if let Module::Code(m) = self.codes.get(module_slot as usize)? {
            m.get_string_from_pool(string_slot)
        } else {
            None
        }
    }

    pub fn get_main_slot(&self) -> Option<u32> {
        let main = self.get_main_code()?;

        if let Module::Code(main) = main {
            main.get_main_slot()
        } else {
            None
        }
    }

    pub fn contains_mod(&self, name: &str) -> bool {
        self.slot_map.contains_key(name)
    }
}
