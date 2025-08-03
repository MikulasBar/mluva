mod interpreter;
mod value;
mod errors;
mod instruction;
mod compiler;
mod interpreter_source;
mod function;
mod ast;
mod bytecode;

use std::{collections::HashMap, io::{Read, Write}};
use compiler::compile_from_str;
use errors::InterpreterError;
use function::ExternalFunctionSource;
use interpreter::Interpreter;
use value::Value;

use crate::bytecode::BytecodeSerializable as _;


const PRINT_EFSOURCE: ExternalFunctionSource = ExternalFunctionSource {
    call: |args| {
        for a in args {
            print!("{} ", a);
        }
        println!();
        Ok(Value::Void)
    },
};

const ASSERT_EFSOURCE: ExternalFunctionSource = ExternalFunctionSource {
    call: |args| {
        if args[0].is_false()? {
            return Err(InterpreterError::Other("Assertion failed".to_string()));
        }

        Ok(Value::Void)
    }
};

fn main() {
    let v = Value::Float(54654.564654);
    let bytes = v.to_bytecode();

    let mut f = std::fs::File::create("test.mvb").unwrap();
    // file.write(&bytes).unwrap();



    




    let mut input = String::new();
    let mut file = std::fs::File::open("test.mv").unwrap();
    file.read_to_string(&mut input).unwrap();

    let mut externals = HashMap::new();
    externals.insert("print".to_string(), PRINT_EFSOURCE);
    externals.insert("assert".to_string(), ASSERT_EFSOURCE);

    let compile_result = compile_from_str(&input, externals);

    if let Err(e) = compile_result {
        eprintln!("Compilation error: {:?}", e);
        return;
    }

    let source = compile_result.unwrap();

    let function::FunctionSource::Internal(func) = source.functions[source.main_slot].clone() else {panic!()};
    let instructions = func.body;
    let mut bytes = vec![];

    for instruction in instructions {
        let instruction_bytes = instruction.to_bytecode();
        bytes.extend(instruction_bytes);
    }

    f.write(&bytes).unwrap();


    let mut instrs = vec![];
    let mut cursor = 0;
    while cursor < bytes.len() {
        match instruction::Instruction::from_bytecode(&bytes, &mut cursor) {
            Ok(instr) => instrs.push(instr),
            Err(e) => {
                eprintln!("Error reading instruction at position {}: {}", cursor, e);
                return;
            }
        }
    }

    // let interpret_result = Interpreter::new(source).interpret();

    // if let Err(e) = interpret_result {
    //     eprintln!("Interpretation error: {:?}", e);
    //     return;
    // }
}



