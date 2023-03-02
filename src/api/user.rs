use crate::api::helpers::responses::{ErrorResponse, LoginResponse, TokenResponse};
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

    let unsigned_token = Token::new(header, claims);
    let signed_token = match unsigned_token.sign_with_key(&*KEY) {
        Ok(value) => value,
        Err(_) => return ServerError(ErrorResponse::generate_error("Error generating token")),
    };

    Authenticated(TokenResponse::generate_message(signed_token.as_str()))
}

#[post("/logout")]
pub fn logout(auth_token: token::Token<'_>) {
    // Todo remove authentication token
}

