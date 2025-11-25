use std::collections::HashMap;

use crate::function::FunctionSigniture;

pub struct ModuleSigniture {
    function_map: HashMap<String, u32>,
    function_signitures: Vec<FunctionSigniture>,
}

impl ModuleSigniture {
    pub fn new(
        function_map: HashMap<String, u32>,
        function_signitures: Vec<FunctionSigniture>,
    ) -> Self {
        Self {
            function_map,
            function_signitures,
        }
    }

    pub fn get_function_signiture(&self, name: &str) -> Option<&FunctionSigniture> {
        let slot = self.function_map.get(name)?;
        self.function_signitures.get(*slot as usize)
    }

    pub fn get_slot(&self, name: &str) -> Option<u32> {
        self.function_map.get(name).copied()
    }
}
