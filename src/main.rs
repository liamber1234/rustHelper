use CoffeProj::Coffe::CoffeOrder;
use CoffeProj::Coffe::CoffeSize;
use CoffeProj::Coffe::CoffeType;

fn main() {
    let mut sugar_amount = 0;
    let mut coffee_size;
    
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
        
        let order = CoffeOrder::new(coffe_type, coffee_size, is_containg_sugar, Some(sugar_amount));
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
fn convert_to_type(coffee_type_input: &str) -> Option<CoffeType> {
    match coffee_type_input.trim() {
        "1" => Some(CoffeType::Espresso),
        "2" => Some(CoffeType::Latte),
        "3" => Some(CoffeType::Cappuccino),
        "4" => Some(CoffeType::Americano),
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
fn convert_to_size(coffee_size_input: &str) -> Option<CoffeSize> {
    match coffee_size_input.trim() {
        "1" => Some(CoffeSize::Small),
        "2" => Some(CoffeSize::Medium),
        "3" => Some(CoffeSize::Large),
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
            println!("Error reading input. Please try again.");
            continue;
        }
        
        let coffe_type = convert_to_type(user_input.trim());
        if coffe_type.is_none() {
            println!("Invalid coffee type. Please try again.");
            continue;
        }

        return coffe_type.unwrap()
    }
}

handle_size_input() -> CoffeSize {
    let mut user_input = String::new();

    loop {
        print_size_menu();
        user_input.clear();
        if std::io::stdin().read_line(&mut user_input).is_err() {
            println!("Error reading input. Please try again.");
            continue;
        }
        
        let coffee_size = convert_to_size(user_input.trim());
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