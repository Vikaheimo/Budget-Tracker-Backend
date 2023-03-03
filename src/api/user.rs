use crate::api::helpers::responses::{ErrorResponse, LoginResponse, TokenResponse, SignUpResponse};
use crate::api::helpers::token;
use rocket::serde::json::Value;
use rocket::serde::{json::Json, Deserialize, Serialize};

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
    sub: usize,
    exp: u128,
}

#[derive(Default, Deserialize, Serialize)]
pub struct SignupData {
    username: String,
    password: String,
    email: String,
}

impl SignupData {
    pub fn is_valid(&self) -> bool {
        self.password.is_ascii()
    }
}

lazy_static! {
    static ref KEY: Hmac<Sha256> =
        Hmac::new_from_slice(std::env::var("TOKEN_AUTH_STRING").unwrap().as_bytes()).unwrap();
}

#[post("/login", data = "<login_details>")]
pub fn login(login_details: Json<LoginInfo>) -> LoginResponse {
    use LoginResponse::*;
    // Check authentication!
    // Dummy user
    let user_id = 123;

    match generate_token(user_id) {
        Ok(signed_token) => Authenticated(
            TokenResponse::generate_message(signed_token.as_str())),
        Err(_) => ServerError(ErrorResponse::generate_error("Error generating token")),
    }
}

#[post("/logout")]
pub fn logout(auth_token: token::Token<'_>) {
    // Todo remove authentication token
}

#[post("/signup", data = "<signup_details>")]
pub fn sign_up(signup_details: Json<SignupData>) -> SignUpResponse {
    use SignUpResponse::*;

    if !signup_details.is_valid() {
        return BadSignUpDetails(
            ErrorResponse::generate_error("Password contains non-valid ascii characters"));
    }

    // Dummy id
    let user_id = 123;
    match generate_token(user_id) {
        Ok(signed_token) => Success(
            TokenResponse::generate_message(signed_token.as_str())),
        Err(_) => ServerError(ErrorResponse::generate_error("Error generating token")),
    }
}

fn generate_token(user_id: usize) -> Result<jwt::Token<jwt::Header, Claims, jwt::token::Signed>, jwt::Error>{
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
