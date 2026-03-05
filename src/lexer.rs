mod token;

// This file is used for taking the raw string from the user and converting it into a vector of tokens that can be used by the parser
// Goal is to return a vector of tokens

// Tokenize function will return a vector of token enums (does not check if expression is valid)
pub fn tokenize(input: &str) -> Result<Vec<Token>, String>{

    // stack used for validating parentheses
    let mut stack = Vec::new();

    // buffer used for building multi digit numbers and decimals
    let mut num_buffer = String::new();

    // vector to store the tokens (the thing we will return at the end)
    let mut tokens = Vec::new();  

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
                    tokens.push(TOKEN::NUMBER(num_buffer.parse().unwrap())) 
                    num_buffer.clear(); // Emptying the buffer
                    tokens.push(TOKEN::MULTIPLY)
                }
            }
            '-'
            '*'
            '/'
            '('
            ')'
            '^'
            _ => Err(String::from("unknown char"))
        }
    }

}