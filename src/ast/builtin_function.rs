use std::collections::HashSet;

use crate::{errors::RuntimeError, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinFunction {
    Print,
    Assert,
    Format,
}

impl BuiltinFunction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "print" => Some(BuiltinFunction::Print),
            "assert" => Some(BuiltinFunction::Assert),
            "format" => Some(BuiltinFunction::Format),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BuiltinFunction::Print => "print",
            BuiltinFunction::Assert => "assert",
            BuiltinFunction::Format => "format",
        }
    }

    pub fn str_variants() -> HashSet<&'static str> {
        let mut set = HashSet::new();
        for variant in ["print", "assert", "format"] {
            set.insert(variant);
        }

        set
    }

    pub fn execute(&self, args: Vec<Value>) -> Result<Value, RuntimeError> {
        match self {
            BuiltinFunction::Print => {
                for arg in args {
                    print!("{}", arg);
                }
                println!();
                Ok(Value::Void)
            }
            BuiltinFunction::Assert => {
                for arg in args {
                    if arg.is_false()? {
                        return Err(RuntimeError::AssertionFailed);
                    }
                }

                Ok(Value::Void)
            }
            BuiltinFunction::Format => {
                let mut result = String::new();
                for arg in args {
                    result.push_str(&arg.to_string());
                }
                Ok(Value::String(result))
            }
        }
    }
}
