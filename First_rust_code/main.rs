fn main(){
    let a: (i32, &str) = (32, "Opa");
    let dictionary = Vec2{x: 3, y: 5};
    println!("{}", a.1);
    println!("{}", calculate("", dictionary.x, dictionary.y));
}
// Struct = Class
struct Vec2{
    x: i32,
    y: i32,
}

fn calculate(operation: &str, x: i32 , y: i32) -> i32{
    if operation == "sum"{
        return x + y
    } else if operation == "sub"{
        return x - y
    } else {
        return x * y
    }
}