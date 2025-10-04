use crate::bytecode::{serializable::BytecodeSerializable};

const MAGIC: &[u8] = &[0x00, 0x08, b'm', b'v', 0x00, b'b', 0x08];

#[derive(Debug, Clone, Copy)]
pub struct BytecodeHeader {
    pub version: u8,
    pub main_slot: Option<u32>,
    pub function_count: u32,
}

impl BytecodeHeader {
    pub const CURRENT_VERSION: u8 = 1;
    const ERROR_NOT_ENOUGH_BYTES: &'static str = "Not enough bytes for header";

    pub fn new(
        main_slot: Option<u32>,
        function_count: u32,
    ) -> Self {
        BytecodeHeader {
            version: Self::CURRENT_VERSION,
            main_slot,
            function_count,
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

        let version = u8::from_bytecode(bytes, cursor)?;

        if version != Self::CURRENT_VERSION {
            return Err(format!("Unsupported bytecode version: {}", version));
        }

        let main_slot = Option::<u32>::from_bytecode(bytes, cursor)?;
        let function_count = u32::from_bytecode(bytes, cursor)?;

        Ok(BytecodeHeader {
            version,
            main_slot,
            function_count,
        })
    }

    fn write_bytecode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(MAGIC);
        self.version.write_bytecode(buffer);
        self.main_slot.write_bytecode(buffer);
        self.function_count.write_bytecode(buffer);
    }
}
