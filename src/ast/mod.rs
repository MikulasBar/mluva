mod binary_op;
mod expr;
mod path;
mod statement;
mod unary_op;

use std::collections::HashMap;

pub use binary_op::BinaryOp;
pub use expr::Expr;
pub use path::Path;
pub use statement::Stmt;
pub use unary_op::UnaryOp;

use crate::function::InternalFunctionSigniture;

pub struct Ast {
    function_map: HashMap<String, u32>,
    function_signitures: Vec<InternalFunctionSigniture>,
    function_bodies: Vec<Vec<Stmt>>,
    imports: Vec<Path>,
}

impl Ast {
    pub fn new(
        function_map: HashMap<String, u32>,
        function_signitures: Vec<InternalFunctionSigniture>,
        function_bodies: Vec<Vec<Stmt>>,
        imports: Vec<Path>,
    ) -> Self {
        Self {
            function_map,
            function_signitures,
            function_bodies,
            imports,
        }
    }

    pub fn empty() -> Self {
        Self {
            function_map: HashMap::new(),
            function_signitures: vec![],
            function_bodies: vec![],
            imports: vec![],
        }
    }

    pub fn add_function(
        &mut self,
        name: String,
        signiture: InternalFunctionSigniture,
        body: Vec<Stmt>,
    ) {
        let slot = self.function_signitures.len() as u32;
        self.function_map.insert(name, slot);
        self.function_bodies.push(body);
        self.function_signitures.push(signiture);
    }

    pub fn add_import(&mut self, path: Path) {
        self.imports.push(path);
    }

    pub fn function_count(&self) -> u32 {
        self.function_map.len() as u32
    }

    pub fn get_function_slot(&self, name: &str) -> Option<u32> {
        self.function_map.get(name).copied()
    }

    pub fn get_function_signiture(&self, name: &str) -> Option<&InternalFunctionSigniture> {
        let slot = *self.function_map.get(name)?;
        self.function_signitures.get(slot as usize)
    }

    pub fn get_function_signiture_by_slot(&self, slot: u32) -> Option<&InternalFunctionSigniture> {
        self.function_signitures.get(slot as usize)
    }

    pub fn get_function_body_by_slot(&self, slot: u32) -> Option<&[Stmt]> {
        self.function_bodies.get(slot as usize).map(|v| v.as_slice())
    }

    pub fn get_function_map(&self) -> &HashMap<String, u32> {
        &self.function_map
    }

    pub fn deconstruct(self) -> (
        HashMap<String, u32>,
        Vec<InternalFunctionSigniture>,
        Vec<Vec<Stmt>>,
        Vec<Path>,
    ) {
        (
            self.function_map,
            self.function_signitures,
            self.function_bodies,
            self.imports,
        )
    }
}
