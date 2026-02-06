use bincode::{Decode, Encode};

use crate::{
    Descriptor, compile_error::CompileError, diagnostics::Span
};

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct FunctionSigniture {
    pub return_type: Descriptor,
    pub params: Vec<Parameter>,
    pub span: Span,
}

impl FunctionSigniture {
    pub fn new(return_type: Descriptor, params: Vec<Parameter>, span: Span) -> Self {
        Self {
            return_type,
            params,
            span,
        }
    }

    pub fn check_argument_types(
        &self,
        args: &[(Descriptor, Span)],
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
            let (arg_type, arg_span) = &args[i];
            if *arg_type != param.ty {
                return Err(CompileError::wrong_type_at(
                    param.ty.clone(),
                    arg_type.clone(),
                    *arg_span,
                ));
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct Parameter {
    pub name: String,
    pub ty: Descriptor,
    pub span: Span,
}

impl Parameter {
    pub fn new(name: String, ty: Descriptor, span: Span) -> Self {
        Self { name, ty, span }
    }
}
