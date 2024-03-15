use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}
