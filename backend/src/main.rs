#[macro_use]
extern crate rocket;

mod catchers;
mod database;
mod error;
mod model;
mod routes;
mod settings;

pub use error::{RouteError, RouteResult};
use eyre::{Result, WrapErr};
pub use model::{
    auth::Auth,
    user::{PublicUser, User},
};
pub use settings::Settings;

#[rocket::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let settings = Settings::from_env();

    let database = database::init(&settings)
        .await
        .wrap_err("Failed to initialize database")?;

    let _ = rocket::build()
        .manage(settings)
        .manage(database)
        .register("/", catchers![catchers::default_catcher])
        .mount("/users", routes![routes::user::get])
        .mount("/auth", routes![routes::auth::signup])
        .launch()
        .await?;

    Ok(())
}
