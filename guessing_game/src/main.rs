use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    //prompts user to guess
    println!("Guess the number");

    //need to have a secret number
    let secret_number = rand::rng().random_range(1..=100);

    //print what the secret number is with some text
    println!("The secret number is: {secret_number}");

    loop {
        // place for user to input
        println!("Please input your guess: ");
        
        //mutable variable for guess
        let mut guess = String::new(); 

        //checks guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}