use securityChecker::passwordManager::*;

fn main() {
    
    loop {
        let mut password = String::new();
        println!("Enter your password: ");
        std::io::stdin().read_line(&mut password).expect("Failed to read input");
        let password = password.trim();

        match check_password_correct(password) {
            Ok(_) => {
                println!("Password is valid.");
                return;
            }
            Err(e) => {
                println!("Password is invalid: {}", e);
                println!("Please try again.");
            }
        }
    }
}