use crate::{Auth, RouteResult, User};
use eyre::Context;
use rocket::{http::Status, serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[post("/signup", data = "<auth>")]
pub async fn signup(auth: Json<Auth>, database: &State<Surreal<Client>>) -> RouteResult<Status> {
    let existing: Option<User> = database
        .query("SELECT * FROM user WHERE username = $username")
        .bind(("username", &auth.username))
        .await?
        .take(0)?;
    if existing.is_some() {
        return Ok(Status::BadRequest);
    }

    database
        .query("CREATE user SET username = $username, password = $password")
        .bind(("username", &auth.username))
        .bind(("password", &auth.password))
        .await?
        .take::<Option<User>>(0)
        .wrap_err("failed to create user")?;
    Ok(Status::Ok)
}
