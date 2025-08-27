use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the letter!");

    // Generate a random letter (1-26 represents A-Z)
    let secret_number = rand::rng().random_range(1..=26);
    let secret_letter = (b'A' + (secret_number - 1) as u8) as char;

    loop {
        println!("Please input your guess (A-Z).");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim().to_uppercase();
        
        // Check if input is a single letter
        if guess.len() != 1 || !guess.chars().next().unwrap().is_ascii_alphabetic() {
            println!("Please type a single letter (A-Z)!");
            continue;
        }

        let guess_char = guess.chars().next().unwrap();
        let guess_number = (guess_char as u8 - b'A' + 1) as u32;
        
        println!("You guessed: {}", guess_char);

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too early in the alphabet!"),
            Ordering::Greater => println!("Too late in the alphabet!"),
            Ordering::Equal => {
                println!("You win! The letter was {}", secret_letter);
                break;
            }
        }
    }
}
