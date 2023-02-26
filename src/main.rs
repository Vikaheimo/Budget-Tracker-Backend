#[macro_use] extern crate rocket;

mod api;
mod catcher;
mod middleware;

use middleware::cors;
use api::version::version;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(cors::CORS)
    .mount("/api/version", routes![version])
    .register("/",catchers![catcher::not_found])
}
