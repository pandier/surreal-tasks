use rocket::serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Thing,
    pub email: String,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
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
