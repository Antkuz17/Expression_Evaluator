mod lexer;
mod parser; 
mod evaluator;
mod userinput;

use crate::userinput::get_user_input;


fn main() {
    get_user_input();
}
