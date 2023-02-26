#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
mod api;
mod catcher;
mod middleware;

use middleware::{cors, auth};
use api::{version::version, user};

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(cors::CORS)
    .attach(auth::Auth)
    .mount("/api/version", routes![version])
    .mount("/api/user", routes![user::login])
    .mount("/", routes![all_options])
    .register("/",catchers![catcher::not_found])
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
pub fn all_options() {
    /* Intentionally left empty */
}

