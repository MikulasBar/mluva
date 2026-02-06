mod ast;
mod code;
mod signiture;
mod lcp;

pub use ast::ModuleAST;
pub use code::ModuleCode;
pub use signiture::ModuleSigniture;

pub enum Module {
    Code(ModuleCode),
}
