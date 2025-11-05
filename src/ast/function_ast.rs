use crate::{compiler::DataType, diagnostics::Span, errors::CompileError};

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedFunctionSigniture {
    pub return_type: DataType,
    pub params: Vec<SpannedParameter>,
    pub span: Span,
}

impl SpannedFunctionSigniture {
    pub fn new(return_type: DataType, params: Vec<SpannedParameter>, span: Span) -> Self {
        Self {
            return_type,
            params,
            span,
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
pub struct SpannedParameter {
    pub name: String,
    pub data_type: DataType,
    pub span: Span,
}

impl SpannedParameter {
    pub fn new(name: String, data_type: DataType, span: Span) -> Self {
        Self {
            name,
            data_type,
            span,
        }
    }
}
