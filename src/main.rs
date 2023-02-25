mod api;
use api::version::version;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/api/version", routes![version])
}