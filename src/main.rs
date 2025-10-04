mod ast;
mod bytecode;
mod compiler;
mod errors;
mod function;
mod instruction;
mod interpreter;
mod value;
mod module;

use std::{
    io::{Read, Write},
};

use crate::module::Module;

fn main() {
    let mut input = String::new();
    let mut file = std::fs::File::open("test.mv").unwrap();
    file.read_to_string(&mut input).unwrap();

    let module = Module::from_string(&input).expect("Failed to compile module");
    let result = module.execute().expect("Failed to execute module");
    println!("Result: {:?}", result);

    // let compile_result = compile_from_str_to_bc(&input);

    // if let Err(e) = compile_result {
    //     eprintln!("Compilation error: {:?}", e);
    //     return;
    // }

    // let (_, bytecode) = compile_result.unwrap();

    // let mut bytecode_file = std::fs::File::create("test.mvb").unwrap();

    // bytecode_file.write_all(&bytecode).unwrap();

    // let module = ExecutableModule::from_bytecode(&bytecode).unwrap();

    // let interpret_result = Interpreter::new(module).interpret();

    // if let Err(e) = interpret_result {
    //     eprintln!("Interpretation error: {:?}", e);
    //     return;
    // }
}
