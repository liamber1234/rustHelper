use std::io;
use std::result::Result;

fn main() {
    let mut fisrt_input = String::new();
    let mut second_input = String::new();
    let mut operation = String::new();

    println!("Rust Calculator");    

    // Get user input for the first number 
    println!("Enter first number: ");
    io::stdin().read_line(&mut fisrt_input).expect("failed");
    let first_number: i32 = fisrt_input.trim().parse().expect("Please enter a number");

    // Get user input for the second number
    println!("Enter second number: ");
    io::stdin().read_line(&mut second_input).expect("failed");
    let second_number: i32 = second_input.trim().parse().expect("Please enter a number");

    // Get user input for the operation
    println!("Choose operation: +, -, *, /:");
    io::stdin().read_line(&mut operation).expect("failed");

    let result = operate(first_number, second_number, operation);
    if !result.is_err() {
        println!("The result is: {}", result.unwrap());
    }
}


/// This function takes inputs of user - (two numbers and an operation) and returns the result of the operation.
fn operate(first_number: i32, second_number: i32, operation: String) -> Result<i32, bool> {
    //check the operaion type
    if operation.trim() == "+" {
        return Result::Ok(first_number + second_number);
    } else if operation.trim() == "-" {
        return Result::Ok(first_number - second_number);
    } else if operation.trim() == "*" {
        return Result::Ok(first_number * second_number);
    } else if operation.trim() == "/" {
        if second_number != 0 {
            return Result::Ok(first_number / second_number);
        } else {
            println!("You cant divide by 0");
            return Result::Err(false);
        }
    } else {
        println!("false, the operations options are: +, -, *, /");
        return Result::Err(false);
    }
}