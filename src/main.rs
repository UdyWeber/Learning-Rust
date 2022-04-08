fn main(){
    let a: (i32, &str) = (32, "Opa");
    let dictionary = Vec2{x: 3, y: 5};
    println!("{}", a.1);
    
    let x: String = match calculate("", dictionary.x, dictionary.y){
        Ok(x) => x.to_string(),
        Err(x) => x.into(),
    };
    
    //{:?} debugs the function return 
    println!("Debug = {:?}, Normal= {}",  x, x);
}
// Struct = Class
struct Vec2{
    x: i32,
    y: i32,
}

fn calculate(operation: &str, x: i32 , y: i32) -> Result<i32, &str>{
    if operation == "sum"{
        return Ok(x + y)
    } else if operation == "sub"{
        return Ok(x - y)
    } else {
        return Err("Deu ruim fi")
    }
}