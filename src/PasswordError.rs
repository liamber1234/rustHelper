use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
/// enum represents different types of errors that can occur when checking a password
pub enum PasswordError {
    #[error("Password isn't in the valid length")]
    InvalidLength,

    #[error("Passowrd isn't including a capital letter")]
    MissingCapital,

    #[error("Password isn't including a small letter")]
    MissingSmall,

    #[error("Password isn't including a number")]
    MissingNumber,
}
