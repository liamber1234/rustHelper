use regex::Regex;
use crate::PasswordError::PasswordError;

/// Minimum password length
const MINIMUM_LENGTH: usize = 8;

/// Maximum password length
const MAXIMUM_LENGTH: usize = 16;

pub fn check_password_correct(password: &str) -> Result<(), PasswordError> {
    
    match valid_length(password) {
        Ok(result) => {
            if !result {
                return Err(PasswordError::InvalidLength);
            }
        }
        Err(_) => return Err(PasswordError::InvalidLength),
        _ => return Err(PasswordError::InvalidLength),
    }
    
    match includes_capital(password) {
        Ok(result) => {
            if !result {
                return Err(PasswordError::MissingCapital);
            }
        }
        Err(_) => return Err(PasswordError::MissingCapital),
        _ => return Err(PasswordError::MissingCapital),
    }

    match includes_small(password) {
        Ok(result) => {
            if !result {
                return Err(PasswordError::MissingSmall);
            }
        }
        Err(_) => return Err(PasswordError::MissingSmall),
        _   => return Err(PasswordError::MissingSmall),
    }
    
    match includes_number(password) {
        Ok(result) => {
            if !result {
                return Err(PasswordError::MissingNumber);
            }
        }
        Err(_) => return Err(PasswordError::MissingNumber),
        _ => return Err(PasswordError::MissingNumber),
    }

    return Ok(());
}

/// this function checks wether the password contains at least one capital letter
/// parameters:
/// - password: the password to check
/// returns:
/// - true if the password contains at least one capital letter, false otherwise
pub fn includes_capital(password: &str) -> Result<bool, regex::Error> {
    let regex = Regex::new(r"[A-Z]")?;
        
    Ok(regex.is_match(password))
}

/// this function checks wether the password contains at least one small letter
/// parameters:
/// - password: the password to check
/// returns:
/// - true if the password contains at least one small letter, false otherwise
pub fn includes_small(password: &str) -> Result<bool, regex::Error> {
        let regex = Regex::new(r"[a-z]")?;
        
        Ok(regex.is_match(password))
}

/// this function checks wether the password contains at least one number
/// parameters:
/// - password: the password to check
/// returns:
/// - true if the password contains at least one number, false otherwise
pub fn includes_number(password: &str) -> Result<bool, regex::Error> {
    let regex = Regex::new(r"[0-9]")?;
    
    Ok(regex.is_match(password))
}

/// this function checks if the password is in the valid length
/// parameters:
/// - password: the password to check
/// returns:
/// - true if the password is in the valid length, false otherwise
pub fn valid_length(password: &str) -> Result<bool, ()> {
    if MINIMUM_LENGTH <= password.len() && password.len() <= MAXIMUM_LENGTH {
        Ok(true)
    } else {
        Err(())
    }
}