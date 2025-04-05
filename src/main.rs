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

use std::io::Read;

use lexer::tokenize;
use parser::parse;
use type_checker::type_check;
use interpreter::interpret;

fn main() {
    // let input = include_str!("./test.mv");
    let mut buf = String::new();
    let _ = std::fs::File::open("test.mv").unwrap_or_else(|e| {
        eprintln!("Error opening file: {:?}", e);
        std::process::exit(1);
    }).read_to_string(&mut buf);

    let tokens = tokenize(&buf).unwrap_or_else(|e| {
        eprintln!("Tokenize error: {:?}", e);
        std::process::exit(1);
    });
    
    let stmts = parse(tokens).unwrap_or_else(|e| {
        eprintln!("Parse error: {:?}", e);
        std::process::exit(1);
    });

    if let Err(e) = type_check(&stmts) {
        eprintln!("Type check error: {:?}", e);
        return;
    }

    let result = interpret(stmts);

    if let Err(e) = result {
        eprintln!("Interpret error: {:?}", e);
        return;
    }
}



