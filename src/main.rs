use std::process::exit;
use std::result::Result;
use std::cmp::Ordering;
use std::io;
use rand::Rng; // 0.8.5

/// Minimum guess number
const MINIMUM_NUMBER: i32 = 1;

/// Maximum guess number
const MAXIMUM_NUMBER: i32 = 101;

fn main() {
    let mut random_guess = String::new();    
    let random = rand::thread_rng().gen_range(MINIMUM_NUMBER..MAXIMUM_NUMBER);

    loop {
        println!("Enter your guess: ");
        io::stdin().read_line(&mut random_guess).expect("failed");
        let input_number: i32 = random_guess.trim().parse().expect("Please enter a number");
        random_guess.clear();
        let mut result = check_range(input_number);
        match result {
            Ok(_) => {},
            Err(ref e) => {
                println!("{}", result.unwrap_err());
                break;
            }
        }

        result = check_guess_correct(input_number, random);
        match result {
            Ok(_) => {
                println!("You guessed the number!");
                break;
            },
            Err(ref e) => {
                println!("{}", result.unwrap_err());
            }
        }
        println!("Try again!");
    }   
}

/// Function to check if the guess is in the valid range
/// parameters:
/// - guess: the number to check
/// returns:
/// - none
fn check_range(guess : i32) -> Result<(), String> {
    if guess < MINIMUM_NUMBER || guess >= MAXIMUM_NUMBER {
        return Result::Err("Number is not in the correct range - bye bye".to_string());
    }
    return Result::Ok(());
}

/// Function to check if the guess is correct
/// parameters:
/// - guess: the number to check
/// - random: the number to check with
/// returns:
/// - Result: Ok if the guess is correct, Err if it is not
fn check_guess_correct(guess: i32, random: i32) -> Result<(), String> {
    match guess.cmp(&random) {
        Ordering::Equal => return Result::Ok(()),
        Ordering::Less => return Result::Err("Your guess is too low".to_string()),
        Ordering::Greater => return Result::Err("Your guess is too high".to_string()),
    }  
}