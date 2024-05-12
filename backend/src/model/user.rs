use rocket::{
    http::Status,
    outcome::{try_outcome, IntoOutcome},
    request::{self, FromRequest},
    Request, State,
};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::Claims;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Thing,
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let database = try_outcome!(request.guard::<&State<Surreal<Client>>>().await);
        let claims = try_outcome!(request.guard::<Claims>().await);

        database
            .select(("user", claims.sub))
            .await
            .ok()
            .and_then(|user| user)
            .or_forward(Status::Unauthorized)
    }
}

#[derive(Debug, Serialize)]
pub struct PrivateUser {
    pub id: String,
    pub username: String,
}

impl From<User> for PrivateUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id.id.to_raw(),
            username: value.username,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PublicUser {
    pub id: String,
}

impl From<User> for PublicUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id.id.to_raw(),
        }
    }
}
