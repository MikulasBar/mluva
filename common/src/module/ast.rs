use std::collections::HashMap;

use crate::{
    Descriptor, ast::Statement, diagnostics::Span, function::FunctionSigniture,
    module::signiture::ModuleSigniture,
};

#[derive(Debug)]
pub struct ModuleAST {
    pub module_descriptor: Descriptor,
    imports: Vec<(Descriptor, Span)>,
    function_bodies: HashMap<String, Vec<Statement>>,
    signiture: ModuleSigniture,
}

impl ModuleAST {
    pub fn empty(module_descriptor: Descriptor) -> Self {
        Self {
            module_descriptor,
            imports: vec![],
            function_bodies: HashMap::new(),
            signiture: ModuleSigniture::empty(),
        }
    }

    pub fn add_import(&mut self, import: Descriptor, span: Span) -> u32 {
        let slot = self.imports.len() as u32;
        self.imports.push((import, span));
        slot
    }

    pub fn add_function(&mut self, name: String, sig: FunctionSigniture, body: Vec<Statement>) {
        self.signiture.add_function(name.clone(), sig);
        self.function_bodies.insert(name, body);
    }

    pub fn fn_count(&self) -> u32 {
        self.function_bodies.len() as u32
    }

    pub fn get_function_sig(&self, name: &str) -> Option<&FunctionSigniture> {
        self.signiture.get_function(name)
    }

    pub fn get_function_body(&self, name: &str) -> Option<&Vec<Statement>> {
        self.function_bodies.get(name)
    }

    pub fn get_function_body_mut(&mut self, name: &str) -> Option<&mut Vec<Statement>> {
        self.function_bodies.get_mut(name)
    }

    pub fn function_names(&self) -> Vec<String> {
        self.function_bodies.keys().cloned().collect()
    }

    pub fn imports(&self) -> &'_ [(Descriptor, Span)] {
        &self.imports
    }

    pub fn to_signiture(self) -> ModuleSigniture {
        self.signiture
    }
}
