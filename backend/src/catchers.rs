use rocket::{http::Status, Request};

#[catch(default)]
pub fn default_catcher(_status: Status, _req: &Request) {}
