use std::io;
use std::result::Result;

const PLUS_COMMAND: &str = "+";
const MINUS_COMMAND: &str = "-";
const MULTIPLY_COMMAND: &str = "*";
const DIVIDE_COMMAND: &str = "/";

fn main() {
    let mut fisrt_input = String::new();
    let mut second_input = String::new();
    let mut operation = String::new();

    println!("Rust Calculator");    

    // Get user input for the first number 
    println!("Enter first number: ");
    let first_number : i32 = input_number();
    
    // Get user input for the second number
    println!("Enter second number: ");
    let second_number: i32 = input_number();

    // Get user input for the operation
    println!("Choose operation: +, -, *, /:");
    io::stdin().read_line(&mut operation).expect("failed");

    match operate(first_number, second_number, operation) {
        Ok(result) => println!("The result is: {}", result),
        Err(_) => println!("Error: invalid input"),
    }
    
}


/// This function takes inputs of user - (two numbers and an operation) and returns the result of the operation.
fn operate(first_number: i32, second_number: i32, operation: String) -> Result<i32, bool> {
    //check the operaion type
    match operation.trim() {
        PLUS_COMMAND => return Result::Ok(first_number + second_number),
        MINUS_COMMAND => return Result::Ok(first_number - second_number),
        MULTIPLY_COMMAND => return Result::Ok(first_number * second_number),
        DIVIDE_COMMAND => {
            if second_number == 0 {
                return Result::Err(false)
            } else {
                return Result::Ok(first_number / second_number)
            }
        },
        _ => return Result::Err(false),
    }
}

fn input_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed");
    let number: i32 = input.trim().parse().expect("Please enter a number");
    return number;
}