use std::collections::HashMap;

use crate::{
    executable_module::ExecutableModule,
    function::{InternalFunctionDefinition, InternalFunctionSource},
};
pub use header::BytecodeHeader;
pub use serializable::BytecodeSerializable;

mod header;
mod serializable;


pub fn write_fn_map_bytecode(fn_map: &HashMap<String, u32>, buffer: &mut Vec<u8>) {
    fn_map.len().write_bytecode(buffer);
    for (name, slot) in fn_map {
        name.write_bytecode(buffer);
        slot.write_bytecode(buffer);
    }
}