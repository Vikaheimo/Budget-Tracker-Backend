use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Version {
    version: &'static str
}

#[get("/")]
pub fn version() -> Json<Version> {
    Json(Version{version: env!("CARGO_PKG_VERSION")})
}
