fn main() {
    //All variables that we define are imutable
    let x: u32 = 4;

    println!("The value of x is: {}", x);

    //If we want to reassing a value to a variable we have to have a mutable variable first
    //Otherwise the program will break

    let mut y: u32 = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("But now the value of y is: {}", y);

    // We can also redefine a variable in our code were we doesnt need to define a variable mutable
    // When we are redefinig a variable in our program we can also change its type

    let z: u32 = 6;
    println!("The value of z is: {}", z);
    let z: u32 = 7;
    println!("The value of the new z variable is: {}", z);

    //We define as a reference of str
    let z: &str = "Eight";
    println!("The value of the new typed z is: {}", z);

    // Constant Variables
    // We have some conventions that we need to follow to do a Constant variable
    // First we have to have a const argument
    // Second we have to name it all capitals, separated by underscores
    // Third we need to type it and assing a value to it
    
    const SECONDS_IN_A_MINUTE: u32 = 60;
    println!("A Minute has: {} seconds", SECONDS_IN_A_MINUTE)
}
