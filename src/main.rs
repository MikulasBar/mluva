mod lexer;
mod parser;
mod interpreter;
mod token;
mod value;
mod token_tree;
mod data_type;
mod scope;
mod macros;


use lexer::tokenize;
use parser::parse;
use interpreter::interpret;

fn main() {
    let input = include_str!("./test.ph");
    let tokens = tokenize(input);
    let stmts = parse(tokens);

    interpret(stmts);
}