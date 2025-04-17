mod interpreter;
mod value;
mod errors;
mod instruction;
mod compiler;
mod interpreter_source;
mod function;
mod ast;

use std::{collections::HashMap, io::Read};
use compiler::compile_from_str;
use function::ExternalFunctionSource;
use interpreter::Interpreter;
use value::Value;


pub const PRINT_EFSOURCE: ExternalFunctionSource = ExternalFunctionSource {
    call: |args| {
        for a in args {
            print!("{} ", a);
        }
        println!();
        Ok(Value::Void)
    },
};

fn main() {
    let mut input = String::new();
    let mut file = std::fs::File::open("test.mv").unwrap();
    file.read_to_string(&mut input).unwrap();

    let mut externals = HashMap::new();
    externals.insert("print".to_string(), PRINT_EFSOURCE.into());

    let compile_result = compile_from_str(&input, externals);

    if let Err(e) = compile_result {
        eprintln!("Compilation error: {:?}", e);
        return;
    }

    let source = compile_result.unwrap();

    let interpret_result = Interpreter::new(source).interpret();

    if let Err(e) = interpret_result {
        eprintln!("Interpretation error: {:?}", e);
        return;
    }
}



