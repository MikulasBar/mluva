use std::sync::Arc;

use bincode::{Decode, Encode};

use crate::class::ClassEntity;
use crate::word::Word;
use crate::function::FunctionEntity;


/// Local constant pool
#[derive(Debug, Clone, Encode, Decode)]
pub struct LCP {
    pool: Vec<LCPEntry>
}

impl LCP {
    pub fn new() -> Self {
        Self { pool: vec![] }
    }

    // pub fn add_function(&mut self, function: Descriptor) -> LCPSlot {

    // }

    // pub fn add_module(&mut self, module: Descriptor) -> LCPSlot {
    //     let path = module.to_string();
    //     let path_slot = self.add_string(path);

    //     let mod_slot = self.pool.push(LCPEntry::UnresolvedModule(path_slot));
    //     let fn_name_slot = self.
    // }

    pub fn add_string(&mut self, s: String) {
        self.pool.push(LCPEntry::String(s));
    }

    pub fn get(&self, index: usize) -> Option<&LCPEntry> {
        self.pool.get(index)
    }
}

type LCPSlot = Word;

#[derive(Debug, Clone, Encode, Decode)]
pub enum LCPEntry {
    String(String),
    UnresolvedClass(LCPSlot),
    UnresolvedModule(LCPSlot),
    UnresolvedFunction {
        name_slot: LCPSlot,
        module_slot: LCPSlot,
    },
    UnresolvedInterface(LCPSlot),
    ResolvedClass(Arc<ClassEntity>),
    ResolvedFunction(Arc<FunctionEntity>),
}