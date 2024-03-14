#[macro_use]
extern crate rocket;

mod catchers;
mod database;
mod error;
mod routes;
mod user;

pub use error::{RouteError, RouteResult};
use eyre::{Result, WrapErr};
pub use user::{PublicUser, User};

#[rocket::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let database = database::init()
        .await
        .wrap_err("Failed to initialize database")?;

    let _ = rocket::build()
        .manage(database)
        .register("/", catchers![catchers::default_catcher])
        .mount("/users", routes![routes::user::get])
        .launch()
        .await?;

    Ok(())
}
