use crate::bytecode::{serializable::BytecodeSerializable};

const MAGIC: &[u8] = &[0x00, 0x08, b'm', b'v', 0x00, b'b', 0x08];

#[derive(Debug, Clone, Copy)]
pub struct BytecodeHeader {
    pub version: u8,
    pub main_slot: Option<u32>,
    pub function_count: u32,
    pub text_block_offset: u32,
}

impl BytecodeHeader {
    pub const CURRENT_VERSION: u8 = 1;
    const ERROR_NOT_ENOUGH_BYTES: &'static str = "Not enough bytes for header";

    pub fn new(
        main_slot: Option<u32>,
        function_count: u32,
        block_buffer_size: u32,
    ) -> Self {
        let main_slot_size = main_slot.is_some() as u32 * 4 + 1;
        let header_size = MAGIC.len() as u32 + 1 + main_slot_size + 4 + 4;
        let text_block_offset = header_size + block_buffer_size;
        BytecodeHeader {
            version: Self::CURRENT_VERSION,
            main_slot,
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

        if version != Self::CURRENT_VERSION {
            return Err(format!("Unsupported bytecode version: {}", version));
        }

        *cursor += 1;

        let main_slot = Option::<u32>::from_bytecode(bytes, cursor)?;

        if bytes.len() < *cursor + 8 {
            return Err(Self::ERROR_NOT_ENOUGH_BYTES.to_string());
        }

        let function_count = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;

        let text_block_offset = u32::from_le_bytes(bytes[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;

        Ok(BytecodeHeader {
            version,
            main_slot,
            function_count,
            text_block_offset,
        })
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(MAGIC);
        buffer.push(self.version);
        self.main_slot.write_bytecode(buffer);
        buffer.extend_from_slice(&self.function_count.to_le_bytes());
        buffer.extend_from_slice(&self.text_block_offset.to_le_bytes());
    }
}
