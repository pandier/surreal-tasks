use crate::{Claims, Settings, User};

use eyre::Result;

pub fn authenticate(settings: &Settings, token: &str) -> Option<Claims> {
    jsonwebtoken::decode::<Claims>(
        token,
        &jsonwebtoken::DecodingKey::from_secret(settings.jwt_secret.as_ref()),
        &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::default()),
    )
    .ok()
    .map(|data| data.claims)
}

pub fn create_token(settings: &Settings, user: &User) -> Result<String> {
    let claims: Claims = user.into();
    Ok(jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(settings.jwt_secret.as_ref()),
    )?)
}
