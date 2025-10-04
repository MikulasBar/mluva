use std::collections::HashMap;

pub use header::BytecodeHeader;
pub use serializable::BytecodeSerializable;

mod header;
mod serializable;


pub fn write_fn_map_bytecode(fn_map: &HashMap<String, u32>, buffer: &mut Vec<u8>) {
    for (name, slot) in fn_map {
        name.write_bytecode(buffer);
        slot.write_bytecode(buffer);
    }
}

pub fn read_fn_map_bytecode(
    bytes: &[u8],
    cursor: &mut usize,
    count: usize,
) -> Result<HashMap<String, u32>, String> {
    let mut fn_map = HashMap::with_capacity(count);
    for _ in 0..count {
        let name = String::from_bytecode(bytes, cursor)?;
        let slot = u32::from_bytecode(bytes, cursor)?;
        fn_map.insert(name, slot);
    }
    Ok(fn_map)
}