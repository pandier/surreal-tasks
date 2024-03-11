use rocket::serde::json::Json;

use crate::User;

#[get("/<id>")]
pub fn get(id: &str) -> Json<User> {
    Json(User { id })
}
