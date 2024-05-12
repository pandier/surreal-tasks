use eyre::Context;
use rocket::{serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{model::user::PrivateUser, PublicUser, RouteResult, User};

#[get("/<id>")]
pub async fn get(
    id: &str,
    database: &State<Surreal<Client>>,
) -> RouteResult<Option<Json<PublicUser>>> {
    Ok(database
        .select(("user", id))
        .await
        .wrap_err("Failed to select user from database")?
        .map(|user: User| Json(user.into())))
}

#[get("/@me")]
pub async fn get_current(user: User) -> Json<PrivateUser> {
    Json(user.into())
}
