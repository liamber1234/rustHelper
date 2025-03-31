use std::process::exit;
use std::io;
use rand::Rng; // 0.8.5

const MINIMUM_NUMBER: i32 = 1;
const MAXIMUM_NUMBER: i32 = 101;

fn main() {
    let mut random_guess = String::new();    
    let random = rand::thread_rng().gen_range(MINIMUM_NUMBER..MAXIMUM_NUMBER);

    loop {
        println!("Enter your guess: ");
        io::stdin().read_line(&mut random_guess).expect("failed");
        let input_number: i32 = random_guess.trim().parse().expect("Please enter a number");
        random_guess.clear();
        check_range(input_number);
        check_guess(input_number, random);
    }   
}

// Function to check if the guess is in the valid range
fn check_range(guess : i32) {
    if guess < MINIMUM_NUMBER || guess >= MAXIMUM_NUMBER {
        println!("Number is not in the correct range - bye bye");
        exit(0);
    }
}

// Function to check if the guess is correct
fn check_guess(guess: i32, random: i32) {
    if guess == random {
        println!("You guessed the number!");
        exit(0);
    } else if guess < random {
        println!("Your guess is too low");
    } else {
        println!("Your guess is too high");
    }
}