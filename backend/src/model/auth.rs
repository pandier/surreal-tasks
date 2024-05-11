use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: u64,
}
