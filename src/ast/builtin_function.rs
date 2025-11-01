use std::{collections::HashSet, str::FromStr};

use crate::{errors::RuntimeError, value::Value};

#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinFunction {
    Print,
    Assert,
    Format,
}

impl BuiltinFunction {
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

impl FromStr for BuiltinFunction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "print" => Ok(BuiltinFunction::Print),
            "assert" => Ok(BuiltinFunction::Assert),
            "format" => Ok(BuiltinFunction::Format),
            _ => Err("Not a builtin function".to_string()),
        }
    }
}
