mod AstNode;
mod evaluator;
mod lexer;
mod parser;
mod token;
mod userinput;

use crate::userinput::get_user_input;

fn main() {
    let tokens = lexer::tokenize("3");
    println!("{:?}", tokens);
}
