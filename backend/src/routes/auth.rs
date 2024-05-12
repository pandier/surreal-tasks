use crate::{auth::create_token, Auth, RouteError, RouteResult, Settings, User};
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

    let token = create_token(settings, &user)?;

    Ok(Json(json!({
        "token": token
    })))
}

#[post("/login", data = "<auth>")]
pub async fn login(
    auth: Json<Auth>,
    settings: &State<Settings>,
    database: &State<Surreal<Client>>,
) -> RouteResult<Json<Value>> {
    let mut result = database
        .query(
            "LET $user = (SELECT * FROM ONLY user WHERE username = $username LIMIT 1);
            IF $user != NONE && crypto::argon2::compare($user.password, $password) THEN
                RETURN $user
            END",
        )
        .bind(("username", &auth.username))
        .bind(("password", &auth.password))
        .await?;

    let user = result
        .take::<Option<User>>(1)?
        .ok_or(RouteError::BadRequest("invalid_login".into()))?;

    let token = create_token(settings, &user)?;

    Ok(Json(json!({
        "token": token
    })))
}
