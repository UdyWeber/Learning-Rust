pub fn run_basic_types_exercises() {
    println!("======== Basic Numeric Types Exercises =======");
    exercise_one();
    exercise_two();
    exercise_three();
    exercise_four();
    exercise_five();
    exercise_seven();
    exercise_eight();
    exercise_nine();
    exercise_ten();
    exercise_eleven();
    exercise_twelf();
    exercise_thirteen();
    exercise_fourteen();
    exercise_fifteen();
    exercise_sixteen();
    exercise_seventeen();
    exercise_eighteen();
    exercise_nineteen();
    exercise_twenty();
    exercise_twenty_one();
}

// Remove something to make it work
fn exercise_one() {
    let x: i32 = 5;
    let mut _y: i32 = 5;

    _y = x;

    let _z = 10; // Type of z ? int

    println!("Success!");
}

// Fill the blank
fn exercise_two() {
    let _v: u16 = 38_u8 as u16;

    println!("Success!");
}

// Modify `assert_eq!` to make it work
fn exercise_three() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// Fill the blanks to make it work
fn exercise_four() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

// Fix errors and panics to make it work
fn exercise_five() {
    // Changed type of variables from u8 to u16 to make it not overflow
    let v1 = 251_u16 + 8_u16;
    let v2 = u16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}

// Modify `assert!` to make it work
fn exercise_seven() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);

    println!("Success!");
}

// Fill the blank to make it work
fn exercise_eight() {
    let x = 1_000.000_1; // ?
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn exercise_nine() {
    // Using f32 with less precise floating points we can achieve the right answer
    // With f64 0.30000000002
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    println!("Success!");
}

//  Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
fn exercise_ten() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    // Can display the ASCII index of char by converting to a number
    for c in 'a'..='z' {
        println!("{}", c as u32);
    }
}

use std::ops::{Range, RangeInclusive};
fn exercise_eleven() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

// Fill the blanks and fix the errors
fn exercise_twelf() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9.6_f32 / 3.2_f32 == 3.0_f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

// Make it work
use std::mem::size_of_val;
fn exercise_thirteen() {
    // In Rust the char type takes four bytes to hold all unicode characters but normally it would hold one
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

fn exercise_fourteen() {
    // let c1 = "中"; This is considered a string by rust cause its held in double quotes
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}

// Make println! work
fn exercise_fifteen() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!");
    }
}

// Make it work
fn exercise_sixteen() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!");
}

// Make it work, don't modify `implicitly_ret_unit` !
fn exercise_seventeen() {
    let v: () = ();

    let _v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());
    assert_eq!(v, explicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

fn exercise_eighteen() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

// Make it work with two ways
fn exercise_nineteen() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
}

fn exercise_twenty() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
 
    println!("Success!");
}

fn exercise_twenty_one() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}
