mod interpreter;
mod value;
mod errors;
mod instruction;
mod compiler;
mod interpreter_source;
mod function;
mod ast;

use std::io::Read;
use compiler::DataType;
use function::{ExternalFunction, ExternalFunctionDefinition, ExternalFunctionSource};
use value::Value;


pub const PRINT_FUNCTION: ExternalFunction = ExternalFunction {
    def: ExternalFunctionDefinition {
        name: "print",
        return_type: DataType::Void,
        check_types: |_types| Ok(()),
    },
    source: ExternalFunctionSource {
        call: |args| {
            for a in args {
                print!("{} ", a);
            }
            println!();
            Ok(Value::Void)
        },
    },
};

fn main() {
    let mut input = String::new();
    let mut file = std::fs::File::open("test.mv").unwrap();
    file.read_to_string(&mut input).unwrap();


    engine.add_function(PRINT_FUNCTION);

    let compile_result = engine.compile(&input);
    if let Err(e) = compile_result {
        eprintln!("Compile error: {:?}", e);
        return;
    }
    let interpreter_source = compile_result.unwrap();

    // println!("{:?}", interpreter_source);

    let interpret_result = engine.interpret(interpreter_source);

    if let Err(e) = interpret_result {
        eprintln!("Interpret error: {:?}", e);
        return;
    }
}



