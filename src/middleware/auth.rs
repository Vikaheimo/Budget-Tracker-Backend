use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Method;

/// Handles authentication in the backend
pub struct Auth;

const NO_AUTH_ROUTES: [&str; 2] = [
    "/version",
    "/user/login"
];

lazy_static! {
    static ref API_NO_AUTH: [String; 2] = NO_AUTH_ROUTES.map(|route| "/api".to_string() + route);
}

#[rocket::async_trait]
impl Fairing for Auth {

    /// Handle authetication for Requests
    fn info(&self) -> Info {
        Info {
            name: "Authentication Fairing",
            kind: Kind::Request,
        }
    }
    
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if API_NO_AUTH.contains(&request.uri().to_string()) || request.method() == Method::Options{
            return;
        }
        
        match request.headers().get_one("auth"){
            None => println!("No auth present!"),
            Some(auth_value) => println!("Header auth present with value {}", auth_value) // todo! check value
        }
    }
}