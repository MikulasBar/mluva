use crate::errors::CompileError;
use crate::{compiler::DataType, instruction::Instruction};


/// Signiture of an in-language function without name
#[derive(Debug, Clone)]
pub struct InternalFunctionSigniture {
    pub return_type: DataType,
    pub params: Vec<(String, DataType)>,
}

impl InternalFunctionSigniture {
    pub fn new(return_type: DataType, params: Vec<(String, DataType)>) -> Self {
        Self {
            return_type,
            params,
        }
    }
    
    pub fn check_argument_types(&self, arg_types: &[DataType]) -> Result<(), CompileError> {
        if self.params.len() != arg_types.len() {
            return Err(CompileError::WrongNumberOfArguments {
                expected: self.params.len(),
                found: arg_types.len(),
            });
        }

        for (i, (_, param_type)) in self.params.iter().enumerate() {
            if &arg_types[i] != param_type {
                return Err(CompileError::WrongType {
                    expected: *param_type,
                    found: arg_types[i],
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct InternalFunctionSource {
    pub slot_count: usize,
    pub body: Vec<Instruction>,
}

impl InternalFunctionSource {
    pub fn new(slot_count: usize, body: Vec<Instruction>) -> Self {
        Self { slot_count, body }
    }
}
