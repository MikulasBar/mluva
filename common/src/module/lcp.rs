use std::sync::Arc;

use bincode::{Decode, Encode};

use crate::class::ClassEntity;
use crate::function::FunctionEntity;


/// Local constant pool
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct LCP {
    pool: Vec<LCPEntry>
}

impl LCP {
    pub fn new() -> Self {
        Self { pool: vec![] }
    }

    // pub fn add_function(&mut self, function: Descriptor) -> LCPSlot {

    // }

    pub fn add_string(&mut self, s: String) {
        self.pool.push(LCPEntry::String(s));
    }

    pub fn add(&mut self, entry: LCPEntry) {
        self.pool.push(entry);
    }

    pub fn get(&self, index: usize) -> Option<&LCPEntry> {
        self.pool.get(index)
    }
}

type LCPSlot = u32;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub enum LCPEntry {
    String(String),
    UnresolvedClass {
        path: String,
    },
    UnresolvedFunction {
        path: String,
    },
    UnresolvedInterface {
        path: String,
    },
    ResolvedClass(Arc<ClassEntity>),
    ResolvedFunction(Arc<FunctionEntity>),
}