use std::collections::HashMap;

use crate::{
    ast::{FunctionSigniture, Path, Statement},
    module::module_signiture::ModuleSigniture,
    type_manager::{Type, TypeManager},
};

pub struct ModuleAST {
    imports: Vec<Path>,
    function_bodies: Vec<Vec<Statement>>,
    signiture: ModuleSigniture,
}

impl ModuleAST {
    pub fn empty() -> Self {
        Self {
            imports: vec![],
            function_bodies: vec![],
            signiture: ModuleSigniture::empty(),
        }
    }

    pub fn add_import(&mut self, import: Path) -> u32 {
        let slot = self.imports.len() as u32;
        self.imports.push(import);
        slot
    }

    pub fn add_fn(&mut self, name: String, sign: FunctionSigniture, body: Vec<Statement>) -> u32 {
        let body_slot = self.function_bodies.len() as u32;
        self.function_bodies.push(body);
        let sign_slot = self.signiture.add_fn(name, sign);

        if sign_slot != body_slot {
            panic!("Function signiture slot and body slot do not match");
        }

        sign_slot
    }

    pub fn get_type_id(&self, name: &str) -> Option<u32> {
        self.signiture.tm.get_id(name)
    }

    pub fn fn_count(&self) -> u32 {
        self.function_bodies.len() as u32
    }

    pub fn get_fn_slot(&self, name: &str) -> Option<u32> {
        self.signiture.get_fn_slot(name)
    }

    pub fn get_sign(&self, slot: u32) -> Option<&FunctionSigniture> {
        self.signiture.get_sign(slot)
    }

    pub fn get_sign_by_name(&self, name: &str) -> Option<&FunctionSigniture> {
        self.signiture.get_sign_by_name(name)
    }

    pub fn get_body(&self, slot: u32) -> Option<&Vec<Statement>> {
        self.function_bodies.get(slot as usize)
    }

    pub fn get_body_mut(&mut self, slot: u32) -> Option<&mut Vec<Statement>> {
        self.function_bodies.get_mut(slot as usize)
    }

    pub fn get_type(&self, slot: u32) -> Option<&Type> {
        self.signiture.tm.get_type(slot)
    }

    pub fn tm(&self) -> &'_ TypeManager {
        &self.signiture.tm
    }

    pub fn imports(&self) -> &'_ [Path] {
        &self.imports
    }

    pub fn get_fn_map(&self) -> &'_ HashMap<String, u32> {
        &self.signiture.get_fn_map()
    }

    pub fn to_signiture(self) -> ModuleSigniture {
        self.signiture
    }
}
