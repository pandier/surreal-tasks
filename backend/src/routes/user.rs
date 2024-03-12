use rocket::{response::status, serde::json::Json, State};
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{PublicUser, User};

#[get("/<id>")]
pub async fn get(
    id: &str,
    database: &State<Surreal<Client>>,
) -> Result<Result<Json<PublicUser>, status::NotFound<()>>, ()> {
    database
        .select(("user", id))
        .await
        .map_err(|_| ())
        .map(|user: Option<User>| {
            user.ok_or(status::NotFound(()))
                .map(|user| Json(user.into()))
        })
}
