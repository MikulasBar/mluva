use std::collections::HashMap;

use common::Descriptor;
use common::ast::{Pattern, PatternKind};
use common::{CompileError, diagnostics::Span};

pub struct TypeScope {
    scopes: Vec<HashMap<String, Descriptor>>,
}

impl TypeScope {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn exit(&mut self) {
        self.scopes.pop();
    }

    pub fn enter(&mut self) {
        self.scopes.push(HashMap::new());
    }
}

impl TypeScope {
    pub fn contains(&self, name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
    }

    pub fn insert_new_pattern(
        &mut self,
        assignee: Pattern,
        ty: Descriptor,
        span: Span,
    ) -> Result<(), CompileError> {
        match assignee.kind {
            PatternKind::Variable(var) => self.insert_new_var(var, ty, span),
        }
    }

    pub fn insert_new_var(
        &mut self,
        name: String,
        ty: Descriptor,
        span: Span,
    ) -> Result<(), CompileError> {
        if self.contains(&name) {
            return Err(CompileError::variable_redeclaration_at(name, span));
        }

        self.scopes
            .last_mut()
            .expect("There is no scope")
            .insert(name, ty);

        Ok(())
    }

    pub fn get_pattern(&self, pattern: &Pattern) -> Result<Descriptor, CompileError> {
        match &pattern.kind {
            PatternKind::Variable(name) => self
                .get(name)
                .cloned()
                .ok_or_else(|| CompileError::variable_not_found_at(name.clone(), pattern.span))
        }
    }

    pub fn get(&self, key: &str) -> Option<&Descriptor> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }
}
