#[macro_use]
extern crate rocket;

mod database;
mod routes;
mod user;

use eyre::{Context, Result};
pub use user::{User, PublicUser};

#[rocket::main]
async fn main() -> Result<()> {
    let database = database::init()
        .await
        .wrap_err("Failed to initialize database")?;

    let _ = rocket::build()
        .manage(database)
        .mount("/users", routes![routes::user::get])
        .launch()
        .await?;

    Ok(())
}
