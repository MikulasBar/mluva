use super::BytecodeSerializable;


struct BytecodeTypeId;

impl BytecodeTypeId {
    const EXECUTABLE: u8 = 0;
    const LIBRARY: u8 = 1;
}

#[derive(Debug, Clone, Copy)]
pub enum BytecodeType {
    Executable { main_slot: u32 },
    Library,
}

impl BytecodeSerializable for BytecodeType {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if bytes.len() < *cursor + 1 {
            return Err("Not enough bytes for BytecodeType".to_string());
        }
        let type_id = bytes[*cursor];
        *cursor += 1;

        match type_id {
            BytecodeTypeId::EXECUTABLE => {
                if bytes.len() < *cursor + 4 {
                    return Err("Not enough bytes for Executable main_slot".to_string());
                }
                let main_slot = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
                *cursor += 4;
                Ok(BytecodeType::Executable { main_slot })
            }
            BytecodeTypeId::LIBRARY => Ok(BytecodeType::Library),
            _ => Err(format!("Unknown BytecodeType ID: {}", type_id)),
        }
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        match self {
            BytecodeType::Executable { main_slot } => {
                buffer.push(BytecodeTypeId::EXECUTABLE);
                buffer.extend_from_slice(&main_slot.to_le_bytes());
            }
            BytecodeType::Library => {
                buffer.push(BytecodeTypeId::LIBRARY);
            }
        }
    }
}