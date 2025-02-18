use rand::Rng;
use std::io::{self, Write};


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Guess the number (between 1 and 10): ");

    loop {

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();


        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue; 
            }
        };

    
        if guess == secret_number {
            println!("Congratulations! You guessed the correct number: {}", secret_number);
            break; 
        } else if guess > secret_number {
            println!("Wrong! You guessed too high!");
        } else if guess < secret_number {
            println!("Wrong! You quessed too low!");
        }
    }
}

