use std::collections::HashMap;

use crate::function::FunctionSource;

pub struct ModuleSource {
    string_pool: Vec<String>,
    main_slot: Option<u32>,
    function_map: HashMap<String, u32>,
    function_sources: Vec<FunctionSource>,
}

impl ModuleSource {
    pub fn new(
        string_pool: Vec<String>,
        main_slot: Option<u32>,
        function_map: HashMap<String, u32>,
        function_sources: Vec<FunctionSource>,
    ) -> Self {
        Self {
            string_pool,
            main_slot,
            function_map,
            function_sources,
        }
    }

    pub fn is_executable(&self) -> bool {
        self.main_slot.is_some()
    }

    pub fn get_main_slot(&self) -> Option<u32> {
        self.main_slot
    }

    pub fn get_main_source(&self) -> Option<&FunctionSource> {
        let slot = self.main_slot?;
        self.function_sources.get(slot as usize)
    }

    pub fn get_sources(&self) -> &[FunctionSource] {
        &self.function_sources
    }

    pub fn get_function_source_by_slot(&self, slot: u32) -> Option<&FunctionSource> {
        self.function_sources.get(slot as usize)
    }

    pub fn get_slot(&self, name: &str) -> Option<u32> {
        self.function_map.get(name).copied()
    }

    pub fn get_string_from_pool(&self, slot: usize) -> Option<&str> {
        self.string_pool.get(slot).map(|s| s.as_str())
    }
}
