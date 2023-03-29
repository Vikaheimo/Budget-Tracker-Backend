use crate::api::helpers::responses::{ErrorResponse, LoginResponse, TokenResponse, SignUpResponse};
use rocket::serde::{json::Json, Deserialize, Serialize};
use crate::models::{user, self};
use crate::database::{self, DatabaseError};
use crate::helpers::hash;
use crate::helpers::user_id_generator;

use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

const MS_IN_HOUR: u128 = 60 * 60 * 1000;
const AUTH_TIME_AS_MILLIS: u128 = MS_IN_HOUR;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Claims {
    sub: u64,
    exp: u128,
}

#[derive(Default, Deserialize, Serialize, Clone)]
pub struct SignupData {
    username: String,
    password: String,
    email: String,
}

impl SignupData {
    /// Check if password contains valid ASCII and email has an '@'
    pub fn is_valid(&self) -> bool {
        self.password.is_ascii() && self.email.contains("@")
    }
}

lazy_static! {
    static ref KEY: Hmac<Sha256> =
        Hmac::new_from_slice(std::env::var("TOKEN_AUTH_STRING").unwrap().as_bytes()).unwrap();
}

#[post("/login", data = "<login_details>")]
pub fn login(login_details: Json<LoginInfo>) -> LoginResponse {
    use LoginResponse::*;

    // Get user details
    let user = match database::users::get_user_by_username(&login_details.username) {
        Ok(value) => value,
        Err(DatabaseError::DoesNotExist) => 
            return BadCredentials(ErrorResponse::generate_error("Invalid credentials")),
        Err(_) => 
            return ServerError(ErrorResponse::generate_error("Error connecting to the database")),
    };
    let user::User {user_id, password: password_hash, ..} = user;

    // Check authentication
    match hash::is_same_password_as_hash(&login_details.password, password_hash) {
        Err(_) => return ServerError(ErrorResponse::generate_error("Password hashing failed")),
        Ok(false) => return BadCredentials(ErrorResponse::generate_error("Invalid credentials")),
        Ok(true) => (),
    }

    match generate_token(user_id as u64) {
        Ok(signed_token) => Authenticated(
            TokenResponse::generate_message(signed_token.as_str())),
        Err(_) => ServerError(ErrorResponse::generate_error("Error generating token")),
    }
}

#[post("/signup", data = "<signup_details>")]
pub fn sign_up(signup_details: Json<SignupData>) -> SignUpResponse {
    use SignUpResponse::*;

    if !signup_details.is_valid() {
        return BadSignUpDetails(
            ErrorResponse::generate_error("Sign Up details are malformed"));
    }

    // Hash the password
    let hashed_password = match hash::hash_password(&signup_details.password){
        Ok(value) => value,
        Err(_) => return ServerError(ErrorResponse::generate_error("Password hashing failed"))
    };
    let user_id = user_id_generator::generate_random_id();

    let new_user = models::user::NewUser {
        username: signup_details.username.as_str(),
        password: hashed_password.as_str(),
        email: signup_details.email.as_str(),
        user_id: user_id,
    };

    match database::users::create_user(new_user) {
        Ok(_) => (),
        Err(database::DatabaseError::AlreadyExists) => 
            return ServerError(ErrorResponse::generate_error("Username is already in use")),
        Err(_) => 
            return ServerError(ErrorResponse::generate_error("Error connecting to the database")),
    }

    match generate_token(user_id as u64) {
        Ok(signed_token) => Success(
            TokenResponse::generate_message(signed_token.as_str())),
        Err(_) => ServerError(ErrorResponse::generate_error("Error generating token")),
    }
}

fn generate_token(user_id: u64) -> Result<jwt::Token<jwt::Header, Claims, jwt::token::Signed>, jwt::Error>{
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let expires = current_time + AUTH_TIME_AS_MILLIS;

    let header: Header = Default::default();
    let claims = Claims {
        sub: user_id,
        exp: expires,
    };

    return Token::new(header, claims).sign_with_key(&*KEY)
}
