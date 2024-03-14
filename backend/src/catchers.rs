use rocket::http::Status;
use rocket::Request;

#[catch(default)]
pub fn default_catcher(_status: Status, _req: &Request) {}
