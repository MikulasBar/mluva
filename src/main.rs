mod interpreter;
mod value;
mod errors;
mod engine;
mod external;
mod instruction;
mod compiler;
mod function_table;

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
    let (instructions, slot_used) = compile_result.unwrap();

    println!("{:?}", instructions);

    let interpret_result = engine.interpret(instructions, slot_used);

    if let Err(e) = interpret_result {
        eprintln!("Interpret error: {:?}", e);
        return;
    }
}



