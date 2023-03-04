//! This file handles auth using request Guards
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

pub struct Token<'r>(&'r str);

#[derive(Debug)]
pub enum TokenError {
    Missing
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token<'r> {
    type Error = TokenError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, TokenError::Missing)),
            Some(key) => Outcome::Success(Token(key)),
        }
    }
}
