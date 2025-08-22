mod ast;
mod bytecode;
mod compiler;
mod errors;
mod executable_module;
mod function;
mod instruction;
mod interpreter;
mod value;

use interpreter::Interpreter;
use std::{
    io::{Read, Write},
};

use crate::{compiler::compile_from_str_to_bc, executable_module::ExecutableModule};

fn main() {
    let mut input = String::new();
    let mut file = std::fs::File::open("test.mv").unwrap();
    file.read_to_string(&mut input).unwrap();

    let compile_result = compile_from_str_to_bc(&input);

    if let Err(e) = compile_result {
        eprintln!("Compilation error: {:?}", e);
        return;
    }

    let (_, bytecode) = compile_result.unwrap();

    let mut bytecode_file = std::fs::File::create("test.mvb").unwrap();

    bytecode_file.write_all(&bytecode).unwrap();

    let module = ExecutableModule::from_bytecode(&bytecode).unwrap();

    let interpret_result = Interpreter::new(module).interpret();

    if let Err(e) = interpret_result {
        eprintln!("Interpretation error: {:?}", e);
        return;
    }
}
