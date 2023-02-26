#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

mod api;
mod catcher;
mod middleware;

use middleware::{cors, auth};
use api::version::version;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(cors::CORS)
    .attach(auth::Auth)
    .mount("/api/version", routes![version])
    .register("/",catchers![catcher::not_found])
}
