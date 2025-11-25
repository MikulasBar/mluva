use crate::{arena::Arena, value_stack::ValueStack, word::Word};

pub const PRIMITIVES_VTABLE_COUNT: u32 = 3;
pub const I32_TYPE_ID: u32 = 0;
pub const F32_TYPE_ID: u32 = 1;
pub const BOOL_TYPE_ID: u32 = 2;
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
        func: fn(Word, &mut ValueStack, &mut Arena, &[VTable]),
    },
}

impl Method {
    pub fn execute(
        &self,
        callee: Word,
        value_stack: &mut ValueStack,
        arena: &mut Arena,
        vtables: &[VTable],
    ) {
        match self {
            Self::Native { func } => {
                func(callee, value_stack, arena, vtables);
            }
        }
    }
}
