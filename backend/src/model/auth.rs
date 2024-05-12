use rocket::{
    http::Status,
    outcome::try_outcome,
    request::{self, FromRequest},
    Request, State,
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
        let settings = try_outcome!(request.guard::<&State<Settings>>().await);

        match request.headers().get_one("Authorization") {
            None => rocket::outcome::Outcome::Forward(Status::Unauthorized),
            Some(key) => {
                let token = key.trim_start_matches("Bearer").trim();

                match jsonwebtoken::decode::<Claims>(
                    token,
                    &jsonwebtoken::DecodingKey::from_secret(settings.jwt_secret.as_ref()),
                    &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::default()),
                ) {
                    Ok(claims) => rocket::outcome::Outcome::Success(claims.claims),
                    Err(_) => rocket::outcome::Outcome::Forward(Status::Unauthorized),
                }
            }
        }
    }
}
