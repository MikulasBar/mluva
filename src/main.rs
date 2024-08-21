mod lexer;
mod parser;
mod interpreter;
mod token;
mod ast;
mod macros;


use lexer::tokenize;
use parser::parse;
use interpreter::interpret;


fn main() {
    let input = include_str!("./test.ph");
    let tokens = tokenize(input);
    let ast = parse(tokens);

    interpret(ast);
}