use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate="rocket::serde")]
pub struct User<'a> {
    pub id: &'a str,
}
