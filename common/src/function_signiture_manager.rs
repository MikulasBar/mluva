use std::collections::HashMap;

use crate::ast::{FunctionSigniture, Statement};

pub struct FunctionSignitureManager {
    pub slots: HashMap<String, u32>,
    signitures: Vec<FunctionSigniture>,
    bodies: Vec<Vec<Statement>>,
}

impl FunctionSignitureManager {
    pub fn empty() -> Self {
        Self {
            slots: HashMap::new(),
            signitures: vec![],
            bodies: vec![],
        }
    }

    pub fn add(&mut self, name: String, signiture: FunctionSigniture, body: Vec<Statement>) -> u32 {
        let slot = self.signitures.len() as u32;
        self.slots.insert(name, slot);
        self.signitures.push(signiture);
        self.bodies.push(body);

        slot
    }

    pub fn count(&self) -> usize {
        self.signitures.len()
    }

    pub fn slots(&self) -> &HashMap<String, u32> {
        &self.slots
    }

    pub fn get_slot(&self, name: &str) -> Option<u32> {
        self.slots.get(name).copied()
    }

    pub fn get_signiture(&self, slot: u32) -> Option<&FunctionSigniture> {
        self.signitures.get(slot as usize)
    }

    pub fn get_signiture_by_name(&self, name: &str) -> Option<&FunctionSigniture> {
        let slot = self.slots.get(name)?;
        self.signitures.get(*slot as usize)
    }

    pub fn get_body(&self, slot: u32) -> Option<&Vec<Statement>> {
        self.bodies.get(slot as usize)
    }

    pub fn get_body_mut(&mut self, slot: u32) -> Option<&mut Vec<Statement>> {
        self.bodies.get_mut(slot as usize)
    }

    pub fn set_body(&mut self, slot: u32, body: Vec<Statement>) {
        if let Some(dest) = self.bodies.get_mut(slot as usize) {
            *dest = body;
        }
    }
}
