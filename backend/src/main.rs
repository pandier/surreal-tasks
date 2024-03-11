#[macro_use]
extern crate rocket;

mod user;
mod routes;

pub use user::User;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/users", routes![routes::user::get])
}
