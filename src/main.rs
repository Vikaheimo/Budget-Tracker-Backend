#[macro_use] extern crate rocket;

mod api;
mod catcher;

use api::version::version;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/api/version", routes![version])
    .register("/",catchers![catcher::not_found])
}