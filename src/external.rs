use crate::{data_type::DataType, errors::{InterpreterError, TypeCheckError}, value::Value};



pub struct ExternalFunction {
    pub name: &'static str,
    pub return_type: DataType,
    check_types: fn(&[DataType]) -> Result<(), TypeCheckError>,
    call: fn(Vec<Value>) -> Result<Value, InterpreterError>,
}

impl ExternalFunction {
    pub fn check_types(&self, args: &[DataType]) -> Result<(), TypeCheckError> {
        (self.check_types)(args)
    }

    pub fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        (self.call)(args)
    }
}

pub const PRINT_FUNCTION: ExternalFunction = ExternalFunction {
    name: "print",
    return_type: DataType::Void,
    check_types: |_args| Ok(()),
    call: |args| {
        for a in args {
            println!("{}", a);
        }
        Ok(Value::Void)
    },
};




// pub trait ExternalFunction {
//     fn get_name(&self) -> String;
//     fn get_return_type(&self) -> DataType;
//     fn check_types(&self, args: &[DataType]) -> Result<(), TypeCheckError>;
//     fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError>;
// }


// pub struct PrintFunction;

// impl ExternalFunction for PrintFunction {
//     fn get_name(&self) -> String {
//         "print".to_string()
//     }

//     fn get_return_type(&self) -> DataType {
//         DataType::Void
//     }

//     fn check_types(&self, _args: &[DataType]) -> Result<(), TypeCheckError> {
//         Ok(())
//     }

//     fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
//         for arg in args {
//             println!("{}", arg);
//         }
//         Ok(Value::Void)
//     }
// }
