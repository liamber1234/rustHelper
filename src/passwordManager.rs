use regex::Regex;
use thiserror::Error;
use crate::PasswordError::PasswordError;

/// Minimum password length
const MINIMUM_LENGTH: usize = 8;

/// Maximum password length
const MAXIMUM_LENGTH: usize = 16;

pub fn check_password_correct(password: &str) -> Result<(), PasswordError> {
    if !valid_length(password) {
        return Err(PasswordError::InvalidLength);
    }
    if !includes_capital(password) {
        return Err(PasswordError::MissingCapital);
    }
    if !includes_small(password) {
        return Err(PasswordError::MissingSmall);
    }
    if !includes_number(password) {
        return Err(PasswordError::MissingNumber);
    }
    Ok(())
}

/// this function checks wether the password contains at least one capital letter
/// parameters:
/// - password: the password to check
/// returns:
/// - true if the password contains at least one capital letter, false otherwise
pub fn includes_capital(password: &str) -> bool {
    let regex = Regex::new(r"[A-Z]").unwrap();
    return regex.is_match(password);
}

/// this function checks wether the password contains at least one small letter
/// parameters:
/// - password: the password to check
/// returns:
/// - true if the password contains at least one small letter, false otherwise
pub fn includes_small(password: &str) -> bool {
    let regex = Regex::new(r"[a-z]").unwrap();
    regex.is_match(password)
}

/// this function checks wether the password contains at least one number
/// parameters:
/// - password: the password to check
/// returns:
/// - true if the password contains at least one number, false otherwise
pub fn includes_number(password: &str) -> bool {
    let regex = Regex::new(r"[0-9]").unwrap();
    regex.is_match(password)
}

/// this function checks if the password is in the valid length
/// parameters:
/// - password: the password to check
/// returns:
/// - true if the password is in the valid length, false otherwise
pub fn valid_length (password: &str) -> bool {
    MINIMUM_LENGTH <= password.len() && password.len() <= MAXIMUM_LENGTH
}