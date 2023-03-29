//! This File handles password hashes

use bcrypt::{DEFAULT_COST, hash, verify};

lazy_static! {
    pub static ref PEPPER: String = std::env::var("PASSWORD_PEPPER").unwrap();
}

/// Hashes a given password with BCrypt. Uses a salt and a Pepper
pub fn hash_password(password: &String, pepper: &str) -> Result<String, bcrypt::BcryptError> {
    hash(format!("{}{}",password, pepper), DEFAULT_COST)
}

/// Checks if a hash is equal to the password
pub fn is_same_password_as_hash(password: &String, hash: String, pepper: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(format!("{}{}",password, pepper), &hash)
}

#[cfg(test)]
mod tests {
    use crate::helpers::hash::is_same_password_as_hash;

    use super::hash_password;

    lazy_static! {
        static ref PEPPER: &'static str = "TEST_PEPPER";
    }

    #[test]
    fn new_password_can_be_checked() {
        let test_password = "password123".to_owned();
        let hashed_password = hash_password(&test_password, &PEPPER).unwrap();
        assert!(is_same_password_as_hash(&test_password, hashed_password, &PEPPER).unwrap())
    }

    #[test]
    fn hash_and_password_dont_match() {
        let bad_password = "amongus123".to_owned();
        let hash = "$2b$12$TrNWobOIox0ZGHiVXPMY4Ot8KtBL9HA3qK2l/ot1/nuJCYn3GEIdi".to_owned();
        assert!(!is_same_password_as_hash(&bad_password, hash, &PEPPER).unwrap())
    }
}
