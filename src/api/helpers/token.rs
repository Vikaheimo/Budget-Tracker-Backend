//! This file handles auth using request Guards
use std::collections::BTreeMap;
use std::time::{UNIX_EPOCH, SystemTime};

use hmac::Hmac;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use jwt::{self, VerifyWithKey};
use sha2::Sha256;

pub struct Token<'r>(&'r str);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenError {
    Missing,
    Expired,
    BadToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        match req.headers().get_one("bearer") {
            None => Outcome::Failure((Status::BadRequest, TokenError::Missing)),
            Some(key) => Outcome::Success(Token(key)),
        }
    }
}

fn get_token_validty_and_subject(token: String, key: &Hmac<Sha256>) -> Result<String, TokenError> {
    let claims: BTreeMap<String, u128> = match token.verify_with_key(key) {
        Err(_) => return Err(TokenError::BadToken),
        Ok(claims) => claims
    };
    println!("kissa");
    let sub = &claims["sub"];
    let expire_time = claims["exp"];
    
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    if expire_time < current_time{
        return Err(TokenError::Expired);
    }

    Ok(sub.to_string())
}

#[cfg(test)]
mod tests {
    use hmac::Hmac;
    use jwt::{SignWithKey, Header, Token};
    use sha2::Sha256;
    use hmac::Mac;
    use crate::api::{user::{self, Claims}, helpers::token::TokenError};

    use super::get_token_validty_and_subject;

    lazy_static! {
        pub static ref KEY: Hmac<Sha256> =
            Hmac::new_from_slice("ASDkjj312".as_bytes()).unwrap();
    }

    fn generate_expired_token(user_id: u64) -> String{
        let header: Header = Default::default();
        let claims = Claims {
            sub: user_id,
            exp: 123123,
        };
        Token::new(header, claims).sign_with_key(&*KEY).unwrap().as_str().to_owned()
    }
    #[test]
    fn check_valid_jwt_token() {
        let user_id = 959821891;
        let binding = user::generate_token(user_id, &*KEY).unwrap();
        let token_string = binding.as_str();
        
        assert_eq!(get_token_validty_and_subject(token_string.to_string(), &*KEY), Ok(959821891.to_string()))
    }

    #[test]
    fn check_inavlid_jwt_token() {
        let token_string = "eyJhbGciOiJIUzI1NiJ9.ayJzdWIiOjk1OTgyMTg5MSwiZXhwIjoxNjgwMDkzNDcyMjE4fQ.yBwc39EoSv2RS-Fa0chVEpqOpssNidJ6RkA8sdbs1j4".to_owned();
        assert_eq!(get_token_validty_and_subject(token_string, &*KEY), Err(TokenError::BadToken))
    }

    #[test]
    fn check_expired_token() {
        let user_id = 12312321;
        let expired_token = generate_expired_token(user_id);
        assert_eq!(get_token_validty_and_subject(expired_token, &*KEY), Err(TokenError::Expired))
    }
}
