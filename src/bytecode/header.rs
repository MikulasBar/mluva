use crate::bytecode::{bytecode_type::BytecodeType, serializable::BytecodeSerializable};

const MAGIC: &[u8] = &[0x00, 0x08, b'm', b'v', 0x00, b'b', 0x08];

pub struct BytecodeHeader {
    version: u8,
    bc_type: BytecodeType,
    function_count: u32,
    text_block_offset: u32,
}

impl BytecodeHeader {
    const ERROR_NOT_ENOUGH_BYTES: &'static str = "Not enough bytes for header";

    pub fn new(
        version: u8,
        bc_type: BytecodeType,
        function_count: u32,
        block_buffer_size: u32,
    ) -> Self {
        let text_block_offset = MAGIC.len() as u32 + 1 + bc_type.byte_count() + block_buffer_size;
        BytecodeHeader {
            version,
            bc_type,
            function_count,
            text_block_offset,
        }
    }
}

impl BytecodeSerializable for BytecodeHeader {
    fn from_bytecode(bytes: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if bytes.len() < *cursor + MAGIC.len() {
            return Err(Self::ERROR_NOT_ENOUGH_BYTES.to_string());
        }

        if &bytes[*cursor..*cursor + MAGIC.len()] != MAGIC {
            return Err("Invalid magic number".to_string());
        }

        *cursor += MAGIC.len();

        if bytes.len() < *cursor + 1 {
            return Err(Self::ERROR_NOT_ENOUGH_BYTES.to_string());
        }

        let version = bytes[*cursor];
        *cursor += 1;

        let bc_type = BytecodeType::from_bytecode(bytes, cursor)?;

        if bytes.len() < *cursor + 8 {
            return Err(Self::ERROR_NOT_ENOUGH_BYTES.to_string());
        }

        let function_count = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;

        let text_block_offset = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;

        Ok(BytecodeHeader {
            version,
            bc_type,
            function_count,
            text_block_offset,
        })
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(MAGIC);
        buffer.push(self.version);
        self.bc_type.write_bytecode(buffer);
        buffer.extend_from_slice(&self.function_count.to_le_bytes());
        buffer.extend_from_slice(&self.text_block_offset.to_le_bytes());
    }
}
