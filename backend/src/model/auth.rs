use rocket::{
    http::Status,
    outcome::{try_outcome, IntoOutcome},
    request::{self, FromRequest},
    Request, State,
};
use serde::{Deserialize, Serialize};

use crate::{auth, Settings, User};

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

impl From<&User> for Claims {
    fn from(value: &User) -> Self {
        Self {
            sub: value.id.id.to_raw(),
            username: value.username.to_owned(),
            exp: jsonwebtoken::get_current_timestamp() + 604800,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let settings = try_outcome!(request.guard::<&State<Settings>>().await);

        match request.headers().get_one("Authorization") {
            None => rocket::outcome::Outcome::Error((Status::Unauthorized, ())),
            Some(key) => {
                let token = key.trim_start_matches("Bearer").trim();
                auth::authenticate(settings, token).or_error((Status::Unauthorized, ()))
            }
        }
    }
}
