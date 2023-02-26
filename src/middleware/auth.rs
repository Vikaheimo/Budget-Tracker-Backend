use rocket::{Request, Data};
use rocket::fairing::{Fairing, Info, Kind};
/// Handles authentication in the backend
pub struct Auth;

const NO_AUTH_ROUTES: [&str; 2] = [
    "/version",
    "/user"
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
        if API_NO_AUTH.contains(&request.uri().to_string()) {
            return;
        }
        
        match request.cookies().get("auth") {
            None => println!("No auth present!"),
            Some(auth_cookie) => println!("Cookie present with value {}", auth_cookie.value()) // todo! check cookie
        }
    }
}