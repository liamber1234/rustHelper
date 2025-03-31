use std::io;

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

    //check the operaion type
    if operation.trim() == "+" {
        println!("Answer:{}", first_number + second_number);
    } else if operation.trim() == "-" {
        println!("Answer:{}", first_number - second_number);
    } else if operation.trim() == "*" {
        println!("Answer:{}", first_number * second_number);
    } else if operation.trim() == "/" {
        if second_number != 0 {
            println!("Answer:{}", first_number / second_number);
        } else {
            println!("You cant divide by 0");
        }
    } else {
        println!("false, the operations options are: +, -, *, /");
    }
}