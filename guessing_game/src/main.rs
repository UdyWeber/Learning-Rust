use std::io;
use std::cmp::Ordering;
use rand::prelude::*;

fn main() {

    println!("Guess the number!");
    
    let random_number: u32 = rand::thread_rng().gen_range(1..101);

    let mut guess_counter: u32 = 0;

    loop{

        if guess_counter > 15 {
            println!("You lost! :( ");
            println!("The random number was: {}", random_number);
            break;
        }

        println!("Please input your guess.");
        
        let mut guess:String = String::new();

        io::stdin().read_line(&mut guess).expect("Could read the line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("Here's your guess: {}", guess);

        match guess.cmp(&random_number){
            Ordering::Less => {
                println!("Guess is lesser than the random number");
                guess_counter += 1;
            },
            Ordering::Greater => {
                println!("Guess is greater than the random number");
                guess_counter += 1;
            },
            Ordering::Equal => {
                println!("You won!");
                println!("You have finished the game with {} guesses", guess_counter);
                {
                    if guess_counter <= 3{
                        println!("You're a lucky one!")
                    } else if  guess_counter <= 8{
                        println!("You're avarage")
                    } else{
                        println!("You're pretty bad at guessing, jeez")
                    }
                }
                break;
            },
        };

    }
}