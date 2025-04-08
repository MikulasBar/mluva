mod lexer;
mod interpreter;
mod token;
mod value;
mod macros;
mod parser;
mod token_tree;
mod data_type;
mod scope;
mod errors;
mod type_checker;
mod engine;
mod external;
mod instruction;
mod compiler;

use std::io::Read;
use engine::Engine;
use external::PRINT_FUNCTION;

fn main() {
    let mut input = String::new();
    let mut file = std::fs::File::open("test.mv").unwrap();
    file.read_to_string(&mut input).unwrap();

    let mut engine = Engine::new();
    engine.add_function(PRINT_FUNCTION);

    let parse_result = engine.parse(&input);
    if let Err(e) = parse_result {
        eprintln!("Parse error: {:?}", e);
        return;
    }

    let stmts = parse_result.unwrap();
    println!("{:?}", stmts);

    let type_check_result = engine.type_check(&stmts);
    if let Err(e) = type_check_result {
        eprintln!("Type check error: {:?}", e);
        return;
    }

    let interpret_result = engine.interpret(&stmts);
    if let Err(e) = interpret_result {
        eprintln!("Interpret error: {:?}", e);
        return;
    }
}



