use crate::module::{module_code::ModuleCode};

pub mod module_ast;
pub mod module_code;
pub mod module_manager;
pub mod module_signiture;
mod lcp;

pub enum Module {
    Code(ModuleCode),
}
