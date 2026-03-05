use std::io;

// This files job is just to get the user input

pub fn get_user_input() -> String {

    // Creating empty string to store the user input
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    println!("{}!", input.trim());

    input

}