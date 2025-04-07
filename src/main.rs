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

use std::io::Read;
use engine::Engine;
use external::PRINT_FUNCTION;

fn main() {
    let mut buf = String::new();
    let _ = std::fs::File::open("test.mv").unwrap_or_else(|e| {
        eprintln!("Error opening file: {:?}", e);
        std::process::exit(1);
    }).read_to_string(&mut buf);

    let mut engine = Engine::new();
    engine.add_function(PRINT_FUNCTION);

    let stmts = engine.parse(&buf).unwrap_or_else(|e| {
        eprintln!("Paring error: {:?}", e);
        std::process::exit(1);
    });

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



