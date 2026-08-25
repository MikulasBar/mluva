use std::fmt::Display;

use bincode::{Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, Hash)]
pub struct Descriptor {
    pub segments: Vec<String>,
}

impl Descriptor {
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn first(&self) -> Option<&str> {
        self.segments.first().map(|s| s.as_str())
    }

    pub fn last(&self) -> Option<&str> {
        self.segments.last().map(|s| s.as_str())
    }

    pub fn pop_last_unchecked(&mut self) -> String {
        self.segments.pop().unwrap()
    }

    /// Substitute first segment with more segments
    ///
    /// Used for import syntax in compiler
    pub fn sub_first(&mut self, segments: Vec<String>) {
        self.segments.splice(0..1, segments);
    }

    pub fn matches(&self, other: &Descriptor) -> bool {
        self.len() == other.len()
            && self
                .segments
                .iter()
                .zip(other.segments.iter())
                .all(|(a, b)| a == b)
    }

    pub fn is_void_type(&self) -> bool {
        self.matches(&Self::void_type())
    }

    pub fn is_bool_type(&self) -> bool {
        self.matches(&Self::bool_type())
    }

    pub fn is_i32_type(&self) -> bool {
        self.matches(&Self::i32_type())
    }

    pub fn is_f32_type(&self) -> bool {
        self.matches(&Self::f32_type())
    }

    pub fn void_type() -> Self {
        descriptor!("Void")
    }

    pub fn bool_type() -> Self {
        descriptor!("Bool")
    }

    pub fn i32_type() -> Self {
        descriptor!("I32")
    }

    pub fn f32_type() -> Self {
        descriptor!("F32")
    }
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self.into();
        write!(f, "{}", string)
    }
}

#[macro_export]
macro_rules! descriptor {
    ($($rest:expr),*) => {
        Descriptor::new(vec![$($rest.to_string(),)*])
    };
}

use descriptor;

impl From<&Descriptor> for String {
    fn from(value: &Descriptor) -> Self {
        value.segments.join(".")
    }
}
