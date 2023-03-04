//! This File handles password hashes

use bcrypt::{DEFAULT_COST, hash, verify};

lazy_static! {
    static ref PEPPER: String = std::env::var("PASSWORD_PEPPER").unwrap();
}

/// Hashes a given password with BCrypt. Uses a salt and a Pepper
pub fn hash_password(password: &String) -> Result<String, bcrypt::BcryptError> {
    hash(format!("{}{}",password, *PEPPER), DEFAULT_COST)
}

/// Checks if a hash is equal to the password
pub fn is_same_password_as_hash(password: &String, hash: String) -> Result<bool, bcrypt::BcryptError> {
    verify(format!("{}{}",password, *PEPPER), &hash)
}