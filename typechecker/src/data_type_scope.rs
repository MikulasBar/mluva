use std::collections::HashMap;

use common::ast::{Pattern, PatternKind};
use common::data_type::DataType;
use common::{compile_error::CompileError, diagnostics::Span};

pub struct DataTypeScope {
    scopes: Vec<HashMap<String, DataType>>,
}

impl DataTypeScope {
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

impl DataTypeScope {
    pub fn contains(&self, name: &str) -> bool {
        self.scopes
            .iter()
            .rev()
            .any(|scope| scope.contains_key(name))
    }

    pub fn insert_new_pattern(
        &mut self,
        assignee: Pattern,
        data_type: DataType,
        span: Span,
    ) -> Result<(), CompileError> {
        match assignee.kind {
            PatternKind::Index { .. } => {
                return Err(CompileError::invalid_pattern_at(assignee.span));
            }
            PatternKind::Variable(var) => self.insert_new_var(var, data_type, span),
        }
    }

    pub fn insert_new_var(
        &mut self,
        name: String,
        data_type: DataType,
        span: Span,
    ) -> Result<(), CompileError> {
        if self.contains(&name) {
            return Err(CompileError::variable_redeclaration_at(name, span));
        }

        self.scopes
            .last_mut()
            .expect("There is no scope")
            .insert(name, data_type);

        Ok(())
    }

    pub fn get_pattern(&self, pattern: &Pattern) -> Result<DataType, CompileError> {
        match &pattern.kind {
            PatternKind::Variable(name) => self
                .get(name)
                .cloned()
                .ok_or_else(|| CompileError::variable_not_found_at(name.clone(), pattern.span)),
            PatternKind::Index { callee, index } => {
                let callee_type = self.get_pattern(callee)?;
                callee_type
                    .get_index_type()
                    .ok_or_else(|| CompileError::invalid_indexing_at(index.span))
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&DataType> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(value);
            }
        }
        None
    }
}
