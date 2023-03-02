use rocket::serde::{json::Json, Serialize};

#[derive(Debug, Responder)]
pub enum LoginResponse {
    #[response(status = 400, content_type = "json")]
    AlreadyAuthenticated(Json<ErrorResponse>),
    
    #[response(status = 403, content_type = "json")]
    BadCredentials(Json<ErrorResponse>),

    #[response(status = 200, content_type = "json")]
    Authenticated(Json<TokenResponse>),

    #[response(status = 500, content_type = "json")]
    ServerError(Json<ErrorResponse>),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String
}
impl ErrorResponse {
    pub fn generate_error(error: &str) -> Json<ErrorResponse> {
        Json(ErrorResponse { error: error.to_string() })
    }
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    token: String
}

impl TokenResponse {
    pub fn generate_message(token: &str) -> Json<TokenResponse> {
        Json(TokenResponse { token: token.to_string() })
    }
}
