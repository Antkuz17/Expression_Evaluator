mod lexer;
mod parser; 
mod evaluator;
mod userinput;
mod token;

use crate::userinput::get_user_input;


fn main() {
    let tokens = lexer::tokenize("abc");
    println!("{:?}", tokens);
}