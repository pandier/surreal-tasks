use rocket::response::Responder;

pub type RouteResult<T> = std::result::Result<T, RouteError>;

#[derive(Debug)]
pub struct RouteError(eyre::Error);

impl<E> From<E> for crate::RouteError
where
    E: Into<eyre::Error>,
{
    fn from(error: E) -> Self {
        RouteError(error.into())
    }
}

impl<'r> Responder<'r, 'static> for RouteError {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::response::Debug(self.0).respond_to(request)
    }
}
