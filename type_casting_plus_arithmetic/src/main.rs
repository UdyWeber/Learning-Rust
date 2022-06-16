fn main() {
    let x: u8 = 255; // 0..255 range for u8
    // If we pass the limit of the data type its going to overflow
    let y: i8 = 1; // -128..127
    // if wee try let z = x + y
    // its going to raise an error cause you cannot sum values from different data type
    let z = x as u16 + y as u16;
    println!("{}", z);
    // Make this add problem work we have to convert the data types
    // The the type of the result of the operation will always be the same type of the values
    // Lets make another operation with division
    let a: u8 = 255;
    let b: u8 = 10;
    let result = a / b;
    println!("{}", result);
    // So as we printed out the value we see that it gives us 25 insted of 25.5
    // Thats because 25.5 cannot be represented as a u8 so we have to convert the types
    // In order to get the correct answer
    let right_result = a as f32 / b as f32 ;
    println!("{}", right_result);
    // Lets go with module operation now
    // We can perform mod operation using the % sing
    let mod_op = a % b;
    println!("{}", mod_op);
    // When you'll have to convert certain numbers to another type
    // You always will prefer to convert the number who has the least amount of bits to a grather
    // For Exemple
    let min = 25 as i8;
    let max = 100 as i16;
    let min_max = max + min as i16;
    println!("{}", min_max);
    // To know the limit of each type we have some methods like
    let min = i8::MIN;
    let max = i16::MAX;
    println!("{}", min);
    println!("{}", max);

    // We can also convert string to Numbers
    use std::io::{stdin};

    println!("Pass me a num ill return it to you plus ten");

    let mut new_string = String::new();
    stdin().read_line(&mut new_string).unwrap();
    // We have to always trim our imputs to remove the \n that the terminal uses to skip the line
    // And to get the int in the string we have to parse it to a integer type ( i picked u16)
    let int_str = new_string.trim().parse::<u16>();
    // The parse method will return a Result Object
    // So we match it and so i can manipulate the error
    match int_str{
        Ok(..) => println!("{}", int_str.unwrap() + 10),
        Err(..) => println!("No numbers in the string, YOU LIED TO ME!"),
    };
}
