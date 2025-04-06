use securityChecker::passwordManager::*;

fn main() {
    let mut is_valid = false;

    while !is_valid {
        let mut password = String::new();
        println!("Enter your password: ");
        std::io::stdin().read_line(&mut password).expect("Failed to read input");
        let password = password.trim();

        match check_password_correct(password) {
            Ok(_) => {
                println!("Password is valid.");
                is_valid = true;
            }
            Err(e) => {
                println!("Password is invalid: {}", e);
                println!("Please try again.");
            }
        }
    }
}