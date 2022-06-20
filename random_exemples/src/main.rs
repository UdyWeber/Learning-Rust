use core::fmt;
use std::{io::{stdin}, process::{exit}};

// Custom Type that shows the FizzBuzz + the number used in the FizzBuzz
#[derive(Debug)]
pub enum FizzBuzzType{
    Fizz(i32),
    Buzz(i32),
    FizzBuzz(i32),
}

// New Struct that allows me to make a impl of Display into my Vectors
struct CustomVec(Vec<i32>);

impl fmt::Display for CustomVec {
    // Function that makes the Vector display on the terminal 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vector = &self.0;
        // Check if vec is empty to print it with None instead of nothing
        if vector.is_empty(){
            write!(f, "None").unwrap();
        }
        for (index, v) in vector.into_iter().enumerate() {
            // Some changes that i did in it to be well formated
            // When the vec is near to its end it doesnt put the coma
            // Otherwaise it format normaly
            if index == vector.len() - 1{
                write!(f, "{}", v).unwrap();
            } else {
                write!(f, "{}, ", v)?;
            }
        }
        Ok(())
    }
}

fn main() {

    println!("Type a number and see magic happends");
    println!("========================================================");

    // Creates the new string to me mutate it into my input
    let mut num_input = String::new();
    stdin().read_line(&mut num_input).expect("Nothing was inputed");

    // The Result Type of the parsing operation to be matched behind
    let num_string = num_input.trim().parse::<i32>();

    // If here the parsed inputed string is compost of characters instead of numbers
    // The program should exit 
    match num_string{
        Ok(_) => (),
        Err(_) => {
            println!("This string contains charactes, must be only integer");
            exit(1)
        },
    }
    
    // Variable used to avoid borrowing errors :D
    let parsed_num_string = num_string.unwrap();

    let fizz_buzz_result = fizz_buzz(parsed_num_string);

    // Here is where we match the result of the fizzbuzz case the number inst divisible by any
    // Is going to return Result::Err and show the number that caused the "Error"
    // Result::Ok is going to show the FizzBuzz value and o numero que causou isso e o path dos divisiveis
    match fizz_buzz_result{
        Ok(_) => {
            println!("========================================================");
            println!("And your result is.... {:?}", fizz_buzz_result.unwrap());
            println!("========================================================");
            print_fizz_buzz_divisible_paths(parsed_num_string);
        },
        Err(error_num) => println!("{} is not divisible per 3 nor 5", error_num),
    };
    println!("========================================================");
    
}
// Calculate the FizzBuzz Type and return a Result 
fn fizz_buzz(number: i32) -> Result<FizzBuzzType, i32> {
    if number % 3 == 0 && number % 5 == 0{
        Ok(FizzBuzzType::FizzBuzz(number))
    } else if number % 3 == 0 {
        Ok(FizzBuzzType::Fizz(number))
    } else if number % 5 == 0 {
        Ok(FizzBuzzType::Buzz(number))
    } else {
        Err(number)
    }
}

// This Funcion iterates over the range of 1 to the number that we had input
// Where we use a Vec cause Vecs can by dinamicly changed
// But the vecs must be Muttable 
fn print_fizz_buzz_divisible_paths(parsed_num: i32){
    let mut divisibles_by_tree: Vec<i32> = vec![];
    let mut divisibles_by_five: Vec<i32> = vec![];
    let mut divisibles_by_both: Vec<i32> = vec![];

    // Calculates the FizzBuzz over the range 1..parsed num + 1 to get its value with it :D
    for number in 1..parsed_num + 1{
        // Implementing For loop To Store Values divisibles by Values
        if number % 3 == 0 && number % 5 == 0{
            divisibles_by_both.push(number)
        } else if number % 3 == 0 {
            divisibles_by_tree.push(number)
        } else if number % 5 == 0 {
            divisibles_by_five.push(number)
        }
    }

    // Print the values inside the vecs with the CustomVec struct
    // More information about it in the top of the code
    println!("Numbers that are divisible by 3: {}", CustomVec(divisibles_by_tree));
    println!("Numbers that are divisible by 5: {}", CustomVec(divisibles_by_five));
    println!("Numbers that are divisible by both: {}", CustomVec(divisibles_by_both));
}
