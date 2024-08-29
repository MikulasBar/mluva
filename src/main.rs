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

use lexer::tokenize;
use parser::parse;
use type_checker::type_check;
use interpreter::interpret;

fn main() {
    let input = include_str!("./test.mv");
    let tokens = tokenize(input);
    let stmts = parse(tokens);
    let stmts = type_check(stmts);

    interpret(stmts);
}



