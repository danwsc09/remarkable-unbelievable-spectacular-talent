use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess number");
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..101);
    println!("Secret number: {}", secret_number);
    
    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error while reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input.");
                continue
            }
        };

        println!("You guessed: {}", guess);

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
