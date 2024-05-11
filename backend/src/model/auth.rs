use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::Settings;

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        match request.headers().get_one("Authorization") {
            None => rocket::outcome::Outcome::Error((Status::Unauthorized, ())),
            Some(key) => {
                let token = key.trim_start_matches("Bearer").trim();
                let secret = Settings::from_env().jwt_secret; // TODO: Find a better way to do this

                match jsonwebtoken::decode::<Claims>(
                    token,
                    &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
                    &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::default()),
                ) {
                    Ok(claims) => rocket::outcome::Outcome::Success(claims.claims),
                    Err(_) => rocket::outcome::Outcome::Error((Status::Unauthorized, ())),
                }
            }
        }
    }
}
