use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Thing,
    pub username: String,
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
