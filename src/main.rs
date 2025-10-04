mod ast;
mod bytecode;
mod compiler;
mod errors;
mod function;
mod instruction;
mod runtime;
mod value;
mod module;

use std::collections::HashMap;

use crate::{compiler::{tokenize, Compiler, Parser, TypeChecker}, module::Module, runtime::Runtime};

fn main() {
    // let input = std::fs::read_to_string("test.mv").unwrap();
    // let module = Module::from_string(&input).expect("Failed to compile module");
    // let result = module.execute_without_dependencies().expect("Failed to execute module");
    // println!("Result: {:?}", result);

    // let bytecode = module.to_bytecode();
    // std::fs::write("test.mvb", &bytecode).unwrap();

    // let buffer = std::fs::read("test.mvb").unwrap();
    // let module_from_bc = Module::from_bytecode_bytes(&buffer).expect("Failed to load module from bytecode");
    // let result_from_bc = module_from_bc.execute_without_dependencies().expect("Failed to execute module from bytecode");
    // println!("Result from bytecode: {:?}", result_from_bc);


    let math_input = std::fs::read_to_string("math.mv").unwrap();
    let math_module = Module::from_string(&math_input).expect("Failed to compile module");
    let mut dependencies = HashMap::new();
    dependencies.insert("math".to_string(), math_module);

    let input = std::fs::read_to_string("test.mv").unwrap();
    let tokens = tokenize(&input).unwrap();
    let ast = Parser::new(&tokens).parse().unwrap();

    TypeChecker::new(&ast, &dependencies).check().unwrap();
    let module = Compiler::new(ast, &dependencies).compile().unwrap();

    let result = Runtime::new(&module, &dependencies).execute().expect("Failed to execute module");
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
