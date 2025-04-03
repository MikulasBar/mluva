mod lexer;
mod interpreter;
mod token;
mod value;
mod macros;
mod parser;
mod type_checker;
mod token_tree;
mod data_type;
mod scope;
mod type_check_error;
mod parse_error;

use lexer::tokenize;
use parser::parse;
use type_checker::type_check;
use interpreter::interpret;

fn main() {
    let input = include_str!("./test.mv");
    let tokens = tokenize(input).unwrap_or_else(|e| {
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

    interpret(stmts);
}



