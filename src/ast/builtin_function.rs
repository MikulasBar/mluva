use std::str::FromStr;

use crate::{
    data_type::DataType,
    diagnostics::Span,
    errors::{CompileError, RuntimeError},
    value::Value,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinFunction {
    Print,
    Assert,
    Format,
}

impl BuiltinFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Print => "print",
            Self::Assert => "assert",
            Self::Format => "format",
        }
    }

    pub fn check_types(&self, span: Span, args: &[DataType]) -> Result<DataType, CompileError> {
        match self {
            Self::Print => Ok(DataType::Void),
            Self::Assert => {
                if args.len() != 1 {
                    return Err(CompileError::wrong_number_of_arguments_at(
                        1,
                        args.len(),
                        span,
                    ));
                }

                if !args[0].is_bool() {
                    return Err(CompileError::wrong_type_at(
                        DataType::Bool,
                        args[0].clone(),
                        span,
                    ));
                }

                Ok(DataType::Void)
            }
            Self::Format => Ok(DataType::String),
        }
    }

    pub fn execute(&self, args: Vec<Value>) -> Result<Value, RuntimeError> {
        match self {
            Self::Print => {
                for arg in args {
                    print!("{}", arg);
                }
                println!();
                Ok(Value::Void)
            }
            Self::Assert => {
                let arg = args.get(0).expect("Malformed arguments for assert");

                if arg.is_false()? {
                    return Err(RuntimeError::AssertionFailed);
                }

                Ok(Value::Void)
            }
            Self::Format => {
                let mut result = String::new();
                for arg in args {
                    result.push_str(&arg.to_string());
                }
                Ok(Value::String(result))
            }
        }
    }
}

impl FromStr for BuiltinFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "print" => Ok(Self::Print),
            "assert" => Ok(Self::Assert),
            "format" => Ok(Self::Format),
            _ => Err("Not a builtin function".to_string()),
        }
    }
}
