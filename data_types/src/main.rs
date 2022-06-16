// Function that a stole from internet to see the type of things :D
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // Rust has a ton of data types so we can splite them in Scalar Data Types and Compound Data Types
    // You dont have to type in the primitive types cause the compiler is going to write it
    // But keep the good practis and type your variables :D
    // Scalars

    let x: i32 = 2; // Signed Integer 32 bits
    println!("{}", x);
    // By default integers are i32
    // We have a lot of Integer Types like i8, i16, i32, i64 and the max beeing i128
    // And the value beeing singed says that I can have a negative number too as -93231, -424, 2394, 1213 those are i32
    // i8 = -2^7..2^7 - 1

    let y: u32 = 312; // Unsigned Integers 32 bits
    println!("{}", y);  
    // Numbers that usomething cant be negative
    // u8 = 0..2^8 -1

    let f: f32 = 1.69;// Floating Integer 32 bits (Single Precision)
    println!("{}", f);
    let F: f64 = 2.79; // Floating Integer 64 bits (Double Precision)
    println!("{}", F);
    //By default floats are f64
    
    let true_or_false: bool = false; // Boolean type 1 for True 0 for false
    println!("{}", true_or_false);

    let letter: char = ';'; // We have the char type that is a unique character variable
    println!("{}", letter);

    // Coumpound Types
    // Oberve that a tuple is going to be typed as a tuple of its values

    let tup: (i32, bool, char) = (1, true, 's');
    let tup2: (i64, bool, char) = (1, true, 's'); // Those tuples seem equal but the types are not the same
    // How to access values from a tuple
    // If we try to print the tuple like = println!("{}", tup); 
    // Is going to throw us an error cause it cannot interpret the tuple normaly
    // If we want to access a tuple we need to call it and pass the index of the value that we want
    println!("{}", tup.0); 
    // We cannot iterate over a tuple like in Python

    let mut tup: (i32, bool, char) = (1, true, 's');
    // You can mut a tuple only based in its type if you try to mutate a value to another type value that will not compile
    // tup.1 = "Claudio"; "Claudio" is a str and tup.1 is a bool
    // And you cannot add more values to the tuple cause your changing its type
    tup.0 = 21312;
    println!("{}", tup.0);

    // Arrays
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Here diferent from other languages you cannot add more values to an array cause its len is set in the type
    // The len of the array has its type too is a usize type
    // You'll have to create a different new array to store more values
    // You can change tha value of the array index value
    arr[2] = 23;
    println!("{}", arr[2]);
    // We cannot have an array of multiple types like let arr2 = ["str", 1];
    // We can iterate over arrays transforming them in iterators with the .iter() method
    // And as well we can catch its index by enumerating the arr
    for (index, element) in arr.iter().enumerate(){
        println!("{} index of element => {}", index, element);
    };

    // You have to be aware that when passing a variable to another variable its type comes with it
    let x: u8 = 5;
    let y = x;
    print_type_of(&y);
    // Some ways to change variables type
    let y: u32 = x as u32;
    print!("{}", y);
    print_type_of(&y);
    // To change its value while attr it to another variable we have to put the method .into() 
    // That is going to transform the variable into the type we passed it if its possible 
    let y: i128 = x.into();
    print_type_of(&y);

    
}  
