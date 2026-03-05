use crate::token::Token;


// This file is used for taking the raw string from the user and converting it into a vector of tokens that can be used by the parser
// Goal is to return a vector of tokens

// Tokenize function will return a vector of token enums (does not check if expression is valid)
pub fn tokenize(input: &str) -> Result<Vec<Token>, String>{

    // buffer used for building multi digit numbers and decimals
    let mut num_buffer = String::new();

    // vector to store the tokens (the thing we will return at the end)
    let mut tokens = Vec::new();  

    // Detemines when num buffer ended
    let mut prev_char_is_num: bool = false;
    
    // Iterating through every character and creating enums for each
    for c in input.chars(){
        match c{
            '0'..='9' | '.' => {
                num_buffer.push(c);
                prev_char_is_num = true;
            }
            '+' => {
                if prev_char_is_num {
                    tokens.push(Token::Number(num_buffer.parse().unwrap())) ;
                    num_buffer.clear(); // Emptying the buffer
                    tokens.push(Token::Plus);
                }
            }
            '-' => {
                if prev_char_is_num {
                    tokens.push(Token::Number(num_buffer.parse().unwrap()));
                    num_buffer.clear(); // Emptying the buffer
                    tokens.push(Token::Minus);
                }
            }
            '*' => {
                if prev_char_is_num {
                    tokens.push(Token::Number(num_buffer.parse().unwrap()));
                    num_buffer.clear(); // Emptying the buffer
                    tokens.push(Token::Multiply);
                }
            }
            '/' => {
                if prev_char_is_num {
                    tokens.push(Token::Number(num_buffer.parse().unwrap()));
                    num_buffer.clear(); // Emptying the buffer
                    tokens.push(Token::Divide);
                }
            }
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '^' => {
                if prev_char_is_num {
                    tokens.push(Token::Number(num_buffer.parse().unwrap()));
                    num_buffer.clear(); // Emptying the buffer
                    tokens.push(Token::Exponent);
                }
            }
            _ => panic!("unknown character: {}", c),
        }
    
    }
    if !num_buffer.is_empty() {
        tokens.push(Token::Number(num_buffer.parse().unwrap()));
    }
    Ok(tokens)
}