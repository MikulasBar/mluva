use std::fmt::Display;

use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Descriptor {
    segments: Vec<String>,
}

impl Descriptor {
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    pub fn single(segment: String) -> Self {
        Self {
            segments: vec![segment],
        }
    }

    pub fn get_tail(&self) -> Option<&String> {
        self.segments.last()
    }
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = self.segments.join(".");
        write!(f, "{}", string)
    }
}
