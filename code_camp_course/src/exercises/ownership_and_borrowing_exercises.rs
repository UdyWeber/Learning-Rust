pub fn run_ownership_and_borrowing_exercises() {
    println!("======== Ownership and Borrowing Exercises =======");
    exercise_one();
    exercise_two();
}

fn exercise_one() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = &x; // Reference to the String
    println!("{},{}", x, y);
}

// Don't modify code in main!
fn exercise_two() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
