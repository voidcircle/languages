use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number: u8 = get_random_secret_number();
    let mut attempt: u16 = 1;

    loop {
        println!("Attempt {attempt}: Guess a number between 0 and 100!");

        let mut guess_input: String = String::new();

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read a line");

        let guess: u8 = match guess_input.trim().parse() {
            // you can combine a match arm and if statement.
            Ok(parsed_number) if parsed_number <= 100 => parsed_number,
            _ => {
                println!("Please put one single whole number that is between 0 and 100.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed it in {attempt} attempts.");
                println!("The secret number was: {secret_number}");
                break;
            }
        }

        attempt += 1;
    }

    println!("\nThank you for playing this game.")
}

fn get_random_secret_number() -> u8 {
    rand::thread_rng().gen_range(0..=100)
}
