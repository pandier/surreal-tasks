use rocket::{http::Status, response::Responder, serde::json::Json};
use serde_json::json;

pub type RouteResult<T> = std::result::Result<T, RouteError>;

#[derive(Debug)]
pub enum RouteError {
    Internal(eyre::Error),
    BadRequest(String),
}

impl<E> From<E> for crate::RouteError
where
    E: Into<eyre::Error>,
{
    fn from(error: E) -> Self {
        Self::Internal(error.into())
    }
}

impl<'r> Responder<'r, 'static> for RouteError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        match self {
            RouteError::Internal(error) => rocket::response::Debug(error).respond_to(request),
            RouteError::BadRequest(error) => (
                Status::BadRequest,
                Json(json!({
                    "error": error
                })),
            )
                .respond_to(request),
        }
    }
}
