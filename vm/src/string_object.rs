pub struct StringObject {
    bytes: Box<[u8]>,
}

impl StringObject {
    pub fn new(s: &str) -> Self {
        Self {
            bytes: s.as_bytes().into(),
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.bytes) }
    }
}
