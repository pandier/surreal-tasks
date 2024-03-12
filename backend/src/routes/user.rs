use eyre::Context;
use rocket::{serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{PublicUser, RouteResult, User};

#[get("/<id>")]
pub async fn get(
    id: &str,
    database: &State<Surreal<Client>>,
) -> RouteResult<Option<Json<PublicUser>>> {
    Ok(database
        .select(("user", id))
        .await
        .wrap_err("failed to select user from database")?
        .map(|user: User| Json(user.into())))
}
