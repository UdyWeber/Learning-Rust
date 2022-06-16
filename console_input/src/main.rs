// In order to getting user input you'll have to import the std for it
// Crate = package/library
use std::io::{stdin};

fn main() {
    let mut input = String::new(); // String::new() We are creating a new empty string
    println!("Console: Hello user how should i call you: ");
    stdin().read_line(&mut input).expect("Failed to read line");
    // So we are calling here the standard input
    // It will going to read the line that we have inputed in
    // Gonna return tho a Result Object that can be a OK or Err so what expect is going to do is
    // It will check if the Result is Ok or Err if its err it will rais "Failed to read line"
    // If we know that is never going to throw and error we can just use .unwrap
    greetings(&input);
}

fn greetings(name: &str){
    for letter in name.chars(){
        if letter.is_numeric(){
            println!("This string contains numeric characters, NOT ALLOWED");
            break;
        }
    }
    // We have to use trim to match the str cause it could have some invisible chars with it
    match name.trim(){
        "Jaw" => println!("Console: Get out, you're ugly!"),
        "Murilo" => println!("Console: You are tchola Murilo!"),
        _ => (),
    };
}
