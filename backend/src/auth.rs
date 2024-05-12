use crate::{Claims, Settings, User};

use eyre::Result;

pub fn create_token(settings: &Settings, user: &User) -> Result<String> {
    let claims: Claims = user.into();
    Ok(jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(settings.jwt_secret.as_ref()),
    )?)
}
