use crate::{Auth, Claims, RouteError, RouteResult, Settings, User};
use eyre::{Context, ContextCompat};
use rocket::{serde::json::Json, State};
use serde_json::{json, Value};
use surrealdb::{engine::remote::ws::Client, Surreal};

#[post("/signup", data = "<auth>")]
pub async fn signup(
    auth: Json<Auth>,
    settings: &State<Settings>,
    database: &State<Surreal<Client>>,
) -> RouteResult<Json<Value>> {
    if database
        .query("SELECT * FROM user WHERE username = $username")
        .bind(("username", &auth.username))
        .await?
        .take::<Option<User>>(0)
        .wrap_err("Failed to query user")?
        .is_some()
    {
        return Err(RouteError::BadRequest("user_already_exists".into()));
    }

    let user = database
        .query(
            "CREATE user SET
                username = $username,
                password = crypto::argon2::generate($password);",
        )
        .bind(("username", &auth.username))
        .bind(("password", &auth.password))
        .await?
        .take::<Option<User>>(0)
        .wrap_err("Failed to create user")?
        .wrap_err("Received None after user creation")?;

    let claims = Claims {
        sub: user.id.id.to_raw(),
        username: user.username,
        exp: jsonwebtoken::get_current_timestamp() + 604800,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(settings.jwt_secret.as_ref()),
    )?;

    Ok(Json(json!({
        "token": token
    })))
}
