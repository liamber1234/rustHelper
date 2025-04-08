use CoffeProj::Coffe::CoffeOrder;
use CoffeProj::Coffe::CoffeSize;
use CoffeProj::Coffe::CoffeType;

fn main() {
    let mut sugar_amount = 0;
    
    loop {        
        let coffe_type = handle_type_input();
        let coffe_size = handle_size_input();
        let sugar_result = handle_sugar_input();

        let is_containg_sugar = sugar_result.is_some();
        if is_containg_sugar {
            match sugar_result {
                Some(amount) => sugar_amount = amount,
                None => println!("No sugar for you"),
            };
        }
        
        let order = CoffeOrder::new(coffe_type, coffe_size, Some(sugar_amount));
        order.print_order();
    }
}

/// this function prints the coffee type menu
/// parameters:
/// - None
/// returns:
/// - None
fn print_type_menu() {
    println!("Select coffee type:");
    println!("1. Espresso");
    println!("2. Latte");
    println!("3. Cappuccino");
    println!("4. Americano");
}

/// this function gets string of number and converts it to CoffeType
/// parameters:
/// - coffee_type_input: the string input from the user
/// returns:
/// - Some(CoffeType) if the input is valid
/// - None if the input is invalid
fn convert_to_type(coffee_type_input: i32) -> Option<CoffeType> {
    const ESPRESSO: i32 = CoffeType::Espresso as i32;
    const LATTE: i32 = CoffeType::Latte as i32;
    const CAPPUCCINO: i32 = CoffeType::Cappuccino as i32;
    const AMERICANO: i32 = CoffeType::Americano as i32;

    match coffee_type_input {
        ESPRESSO => Some(CoffeType::Espresso),
        LATTE => Some(CoffeType::Latte),
        CAPPUCCINO => Some(CoffeType::Cappuccino),
        AMERICANO => Some(CoffeType::Americano),
        _ => None,
    }
}

/// this function prints the coffee size menu
/// parameters:
/// - None
/// returns:
/// - None
fn print_size_menu() {
    println!("Select coffee size:");
    println!("1. Small");
    println!("2. Medium");
    println!("3. Large");
}

/// this function gets string of number and converts it to CoffeSize
/// parameters:
/// - coffee_size_input: the string input from the user
/// returns:
/// - Some(CoffeSize) if the input is valid
/// - None if the input is invalid
fn convert_to_size(coffee_size_input: i32) -> Option<CoffeSize> {
    const SMALL: i32 = CoffeSize::Small as i32;
    const MEDIUM: i32 = CoffeSize::Medium as i32;
    const LARGE: i32 = CoffeSize::Large as i32;

    match coffee_size_input {
        SMALL => Some(CoffeSize::Small),
        MEDIUM => Some(CoffeSize::Medium),
        LARGE => Some(CoffeSize::Large),
        _ => None,
    }
}



fn handle_type_input() -> CoffeType {
    let mut user_input = String::new();

    loop {
        print_type_menu();
        user_input.clear();
        if std::io::stdin().read_line(&mut user_input).is_err()
        {
            println!("Error reading input, Please try again.");
            continue;
        }
        
        let user_input_number = user_input.trim().parse::<i32>();
        if user_input_number.is_err() {
            println!("Invalid input, Please enter a number.");
            continue;
        }

        let coffe_type = convert_to_type(user_input_number.expect("Invalid input"));
        if coffe_type.is_none() {
            println!("Invalid coffee type, Please try again.");
            continue;
        }

        return coffe_type.unwrap()
    }
}

fn handle_size_input() -> CoffeSize {
    let mut user_input = String::new();

    loop {
        print_size_menu();
        user_input.clear();
        if std::io::stdin().read_line(&mut user_input).is_err() {
            println!("Error reading input. Please try again.");
            continue;
        }

        let user_input_number = user_input.trim().parse::<i32>();
        if user_input_number.is_err() {
            println!("Invalid input. Please enter a number.");
            continue;
        }
        
        let coffee_size = convert_to_size(user_input_number.expect("Invalid input"));
        if coffee_size.is_none() {
            println!("Invalid coffee size. Please try again.");
            continue;
        }

        return coffee_size.unwrap()
    }
}

/// this function handles the sugar input from the user
/// parameters:
/// - None
/// returns:
/// - Some(sugar_amount) if the user wants sugar
fn handle_sugar_input() -> Option<u8> {
    let mut user_input = String::new();

    println!("Do you want sugar? (y/n)");
    if std::io::stdin().read_line(&mut user_input).is_err() {
        println!("Error reading input. Please try again.");
        return None;
    }
    let sugar_input = user_input.trim();
    
    match sugar_input.trim() {
        "y" => {
            println!("How many sugar cubes?");
            let mut sugar_amount = String::new();
            if std::io::stdin().read_line(&mut sugar_amount).is_err() {
                println!("Error reading input. Please try again.");
                return None;
            }
            let sugar_amount = sugar_amount.trim().parse::<u8>().ok();
            sugar_amount
        }
        "n" => None,
        _ => None,
    }
}