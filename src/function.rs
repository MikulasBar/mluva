use crate::{
    compiler::DataType, diagnostics::Span, errors::CompileError, instruction::Instruction,
};

/// Signiture of an in-language function without name
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSigniture {
    pub return_type: DataType,
    pub params: Vec<Parameter>,
}

impl FunctionSigniture {
    pub fn new(return_type: DataType, params: Vec<Parameter>) -> Self {
        Self {
            return_type,
            params,
        }
    }

    pub fn check_argument_types(
        &self,
        args: &[(DataType, Span)],
        call_span: Span,
    ) -> Result<(), CompileError> {
        if self.params.len() != args.len() {
            return Err(CompileError::wrong_number_of_arguments_at(
                self.params.len(),
                args.len(),
                call_span,
            ));
        }

        for (i, param) in self.params.iter().enumerate() {
            let (arg_type, arg_span) = args[i];
            if arg_type != param.data_type {
                return Err(CompileError::wrong_type_at(
                    param.data_type,
                    arg_type,
                    arg_span,
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub data_type: DataType,
}

impl Parameter {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self { name, data_type }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionSource {
    pub slot_count: usize,
    pub body: Vec<Instruction>,
}

impl FunctionSource {
    pub fn new(slot_count: usize, body: Vec<Instruction>) -> Self {
        Self { slot_count, body }
    }
}
