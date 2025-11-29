use common::word::Word;

use crate::{arena::Arena, value_stack::ValueStack};

pub struct VTable {
    pub methods: Vec<Method>,
}

impl VTable {
    pub const DESTRUCTOR_SLOT: usize = 0;
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
