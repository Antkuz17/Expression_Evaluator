use crate::token::Token;

// This file is used for taking the raw string from the user and converting it into a vector of tokens that can be used by the parser
// Goal is to return a vector of tokens

// Tokenize function will return a vector of token enums (does not check if expression is valid)
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    // buffer used for building multi digit numbers and decimals
    let mut num_buffer = String::new();

    // vector to store the tokens (the thing we will return at the end)
    let mut tokens = Vec::new();

    let mut flush = |buffer: &mut String, tokens: &mut Vec<Token>| {
        if !buffer.is_empty() {
            tokens.push(Token::Number(buffer.parse().unwrap()));
            buffer.clear();
        }
    };

    // Iterating through every character and creating enums for each
    for c in input.chars() {
        match c {
            '0'..='9' | '.' => {
                num_buffer.push(c);
            }
            '+' => {
                flush(&mut num_buffer, &mut tokens);
                tokens.push(Token::Plus);
            }
            '-' => {
                flush(&mut num_buffer, &mut tokens);
                tokens.push(Token::Minus);
            }
            '*' => {
                flush(&mut num_buffer, &mut tokens);
                tokens.push(Token::Multiply);
            }
            '/' => {
                flush(&mut num_buffer, &mut tokens);
                tokens.push(Token::Divide);
            }
            '(' => tokens.push(Token::LeftParen),
            ')' => {
                if !num_buffer.is_empty() {
                    tokens.push(Token::Number(num_buffer.parse().unwrap()));
                    num_buffer.clear();
                }
                tokens.push(Token::RightParen);
            }
            '^' => {
                flush(&mut num_buffer, &mut tokens);
                tokens.push(Token::Exponent);
            }
            _ => panic!("unknown character: {}", c),
        }
    }
    if !num_buffer.is_empty() {
        tokens.push(Token::Number(num_buffer.parse().unwrap()));
    }
    Ok(tokens)
}
