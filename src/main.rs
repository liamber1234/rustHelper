use std::io;
use std::result::Result;

const PLUS_COMMAND: &str = "+";
const MINUS_COMMAND: &str = "-";
const MULTIPLY_COMMAND: &str = "*";
const DIVIDE_COMMAND: &str = "/";

fn main() {
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

    if check_validation(first_number, second_number, operation.clone()) == false {
        println!("Error: invalid input");
        return;
    }
    match do_operate(first_number, second_number, operation) {
        Ok(result) => println!("The result is: {}", result),
        Err(_) => println!("Error: invalid input"),
    }
    
}


/// This function takes inputs of user - (two numbers and an operation) and returns the result of the operation.
fn do_operate(first_number: i32, second_number: i32, operation: String) -> Result<i32, ()> {
    //check the operaion type
    Result::Ok(match operation.trim() {
        PLUS_COMMAND_var => first_number + second_number,
        MINUS_COMMAND_var => first_number - second_number,
        MULTIPLY_COMMAND_var => first_number * second_number,
        DIVIDE_COMMAND_var => first_number / second_number,
    })
}

/// This function takes user input and returns the number as an i32.
/// param input: the user input as a string
/// return: the user input as an i32
fn input_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed");
    let number: i32 = input.trim().parse().expect("Please enter a number");
    number
}

fn check_validation(first_number: i32, second_number: i32, operation: String) -> bool {
    if operation.trim() == DIVIDE_COMMAND && second_number == 0 {
        return false; // Division by zero is not allowed
    }
    
    if operation.trim() != PLUS_COMMAND && operation.trim() != MINUS_COMMAND && operation.trim() != MULTIPLY_COMMAND && operation.trim() != DIVIDE_COMMAND {
        return false; // Invalid operation
    }

    return true;
}