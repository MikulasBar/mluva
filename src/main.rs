mod interpreter;
mod value;
mod errors;
mod instruction;
mod compiler;
mod executable_module;
mod function;
mod ast;
mod bytecode;

use std::{collections::HashMap, io::{Read, Write}};
use compiler::compile_from_str;
use errors::InterpreterError;
use interpreter::Interpreter;
use value::Value;

use crate::compiler::compile_from_str_to_bc;

fn main() {
    // let v = Value::String("Hello, world!".to_string());
    // let bytes = v.to_bytecode();

    // let mut f = std::fs::File::create("test.mvb").unwrap();
    // f.write(&bytes).unwrap();

    // let mut c = 0;
    // let x = Value::from_bytecode(&bytes, &mut c).unwrap();
    // println!("Deserialized value: {:?}", x);
    // println!("Cursor position after deserialization: {} / {}", c, bytes.len());



    




    let mut input = String::new();
    let mut file = std::fs::File::open("test.mv").unwrap();
    file.read_to_string(&mut input).unwrap();

    let compile_result = compile_from_str_to_bc(&input);

    if let Err(e) = compile_result {
        eprintln!("Compilation error: {:?}", e);
        return;
    }

    let (module, bytecode) = compile_result.unwrap();

    let interpret_result = Interpreter::new(module).interpret();

    let mut bytecode_file = std::fs::File::create("test.mvb").unwrap();

    bytecode_file.write_all(&bytecode).unwrap();

    if let Err(e) = interpret_result {
        eprintln!("Interpretation error: {:?}", e);
        return;
    }
}



