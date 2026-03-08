mod AstNode;
mod evaluator;
mod lexer;
mod parser;
mod token;
mod userinput;

use crate::userinput::get_user_input;

fn main() {
    loop {
        let input = get_user_input();

        match lexer::tokenize(&input) {
            Err(e) => println!("Lexer error: {}", e),
            Ok(tokens) => match parser::parse(&tokens) {
                Err(e) => println!("Parser error: {}", e),
                Ok(tree) => match evaluator::evaluate(&tree) {
                    Err(e) => println!("Evaluator error: {}", e),
                    Ok(result) => println!("= {}", result),
                },
            },
        }

        println!("\n");
    }
}
