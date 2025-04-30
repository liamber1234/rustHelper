use securityChecker::passwordManager::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let password = "Valid123";
        assert!(check_password_correct(password).is_ok());
    }

    #[test]
    fn test_password_too_short() {
        let password = "Short1";
        assert_eq!(
            check_password_correct(password),
            Err(PasswordError::InvalidLength)
        );
    }

    #[test]
    fn test_password_too_long() {
        let password = "ThisPasswordIsWayTooLong123";
        assert_eq!(
            check_password_correct(password),
            Err(PasswordError::InvalidLength)
        );
    }

    #[test]
    fn test_password_missing_capital() {
        let password = "valid123";
        assert_eq!(
            check_password_correct(password),
            Err(PasswordError::MissingCapital)
        );
    }

    #[test]
    fn test_password_missing_small() {
        let password = "VALID123";
        assert_eq!(
            check_password_correct(password),
            Err(PasswordError::MissingSmall)
        );
    }

    #[test]
    fn test_password_missing_number() {
        let password = "ValidPass";
        assert_eq!(
            check_password_correct(password),
            Err(PasswordError::MissingNumber)
        );
    }

    #[test]
    fn test_includes_capital() {
        assert!(includes_capital("A"));
        assert!(!includes_capital("a"));
        assert!(!includes_capital("1"));
    }

    #[test]
    fn test_includes_small() {
        assert!(includes_small("a"));
        assert!(!includes_small("A"));
        assert!(!includes_small("1"));
    }

    #[test]
    fn test_includes_number() {
        assert!(includes_number("1"));
        assert!(!includes_number("a"));
        assert!(!includes_number("A"));
    }

    #[test]
    fn test_valid_length() {
        assert!(valid_length("Valid123"));
        assert!(!valid_length("Short1"));
        assert!(!valid_length("ThisPasswordIsWayTooLong123"));
    }
}

