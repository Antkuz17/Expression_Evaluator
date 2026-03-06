mod lexer;
mod parser; 
mod evaluator;
mod userinput;
mod token;
mod AstNode;

use crate::userinput::get_user_input;


fn main() {
    let tokens = lexer::tokenize("3");
    println!("{:?}", tokens);
}