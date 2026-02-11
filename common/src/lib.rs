pub use crate::descriptor::Descriptor;

pub mod ast;
pub mod compile_error;
pub mod diagnostics;
pub mod function;
pub mod instruction;
pub mod module;
pub mod serde_header;
pub mod token;
pub mod vm;
pub mod word;
pub mod class;

mod descriptor;