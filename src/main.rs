mod interpreter;
mod value;
mod errors;
mod engine;
mod external;
mod instruction;
mod compiler;
mod function_table;
mod interpreter_source;
mod function_source;

use std::io::Read;
use engine::Engine;
use external::PRINT_FUNCTION;

fn main() {
    let mut input = String::new();
    let mut file = std::fs::File::open("test.mv").unwrap();
    file.read_to_string(&mut input).unwrap();

    let mut engine = Engine::new();
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



