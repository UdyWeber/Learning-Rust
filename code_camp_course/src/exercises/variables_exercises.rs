pub fn run_variables_exercises() {
    println!("======== Variables Exercises =======");
    exercise_one();
    exercise_two();
    exercise_three();
    exercise_four();
    exercise_five();
    exercise_six();
    exercise_seven();
    exercise_eight();
    exercise_nine();
}

// Fix the error below with least amount of modification to the code
fn exercise_one() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

// Fill the blanks in the code to make it compile
fn exercise_two() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

// Fix the error below with least amount of modification
fn exercise_three() {
    let y: i32 = 5;
    let x: i32 = 10;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

// Fix the error with the use of define_x
fn exercise_four() {
    let x = "hello";
    println!("{}, world", x)
}

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn exercise_five() {
    let x: i32 = 12;
    {
        let x = 5;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x); // Prints "42".
}

// Remove a line in the code to make it compile
fn exercise_six() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    // let _x = _x; Removed this line cause it ways assigned without being mutable
    _x += 3;

    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}

#[allow(unused_variables)]
fn exercise_seven() {
    // let _x = 10;
    let x = 10;
}

// Fix the error below with least amount of modification
fn exercise_eight() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

fn exercise_nine() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}
