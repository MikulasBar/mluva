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

use crate::bytecode::BytecodeSerializable as _;


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

    let compile_result = compile_from_str(&input);

    if let Err(e) = compile_result {
        eprintln!("Compilation error: {:?}", e);
        return;
    }

    let source = compile_result.unwrap();

    let func = source.functions[source.main_slot].clone();
    let instructions = func.body;
    let mut bytes = vec![];

    for instruction in instructions {
        let instruction_bytes = instruction.to_bytecode();
        bytes.extend(instruction_bytes);
    }

    // f.write(&bytes).unwrap();


    // let mut instrs = vec![];
    // let mut cursor = 0;
    // while cursor < bytes.len() {
    //     match instruction::Instruction::from_bytecode(&bytes, &mut cursor) {
    //         Ok(instr) => instrs.push(instr),
    //         Err(e) => {
    //             eprintln!("Error reading instruction at position {}: {}", cursor, e);
    //             return;
    //         }
    //     }
    // }

    let interpret_result = Interpreter::new(source).interpret();

    if let Err(e) = interpret_result {
        eprintln!("Interpretation error: {:?}", e);
        return;
    }
}



