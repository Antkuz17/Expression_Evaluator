use std::io;

// This files job is just to get the user input and validate it, it does not do any parsing or evaluating

pub fn get_user_input() -> String {

    // Creating empty string to store the user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    while !validate_input(&input) {
        println!("Invalid input, please try again");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
    }

    input = remove_white_space(&input);

    input
}

// Based on the input return true or false if it contains the right variables
// Does not check if its a valid expression e.g. 3{{ would give true
pub fn validate_input(input: &str) -> bool {
    let valid_chars = ['+', '-', '*', '/', '(', ')', '.', ' ', '^', 
                       '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    input.chars().all(|c| valid_chars.contains(&c))
}

// Allows early exit from the program if the user types "exit"
pub fn is_exit_command(input: &str) -> bool {
    input.to_lowercase() == "exit"
}

// Remove all white space since the lexer does not care about it
pub fn remove_white_space(input: &str) -> String {
    input.chars().filter(|c| !c.is_whitespace()).collect()
}




