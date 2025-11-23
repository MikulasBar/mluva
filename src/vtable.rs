use std::ptr::NonNull;

use crate::{arena::Arena, value_stack::ValueStack, vm::Vm};

pub const PRIMITIVES_VTABLE_COUNT: u32 = 3;
pub const I32_TYPE_ID: u32 = 0;
pub const F32_TYPE_ID: u32 = 1;
pub const BOOL_TYPE_ID: u32 = 2;
pub const HHANDLE_TYPE_ID: u32 = 3;
pub const STRING_TYPE_ID: u32 = 4;
pub const LIST_TYPE_ID: u32 = 5;

pub struct VTable {
    pub methods: Vec<Method>,
}

impl VTable {
    pub const DESTRUCTOR_SLOT: usize = 0;
    pub const INDEX_SLOT: usize = 1;
    pub const TO_STRING_SLOT: usize = 2;
}

#[derive(Debug, Clone)]
pub enum Method {
    Native {
        func: fn(NonNull<u8>, &Vec<VTable>, &mut Arena),
    },
}

impl Method {
    pub fn execute(&self, vtables: &Vec<VTable>, arena: &mut Arena) {
        panic!("Methods are not implemented");
    }
}
