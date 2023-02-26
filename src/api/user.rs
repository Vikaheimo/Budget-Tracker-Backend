use rocket::{serde::{json::Json, Serialize, Deserialize}, request::FromRequest, http::{CookieJar, Cookie}};
use crate::api::helpers::responses::{LoginResponse, TokenResponse, ErrorResponse};
use crate::api::helpers::token::Token;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    username: String,
    password: String,
}


#[post("/login", data = "<login_details>")]
pub fn login(login_details: Json<LoginInfo>, cookies: &CookieJar<'_>) -> LoginResponse {
    use LoginResponse::*;
    if let Some(_) = cookies.get("auth") {
        return AlreadyAuthenticated(ErrorResponse::generate_error("Already authenticated"))
    }
    // TODO, check for authentication!
    cookies.add(Cookie::new("auth", "TODO"));
    Authenticated(TokenResponse::generate_message("TODO"))
}



#[post("/logout")]
pub fn logout(auth_token: Token<'_>){
    // Todo remove authentication token
}
