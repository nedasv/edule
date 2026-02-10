#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use std::path::Path;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

struct ApiKey;

// Basic authentication setup, requires "X-API-Key" header with "my-secret-key" as content to return content
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = &'static str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-API-Key") {
            Some(key) if key == "my-secret-key" => Outcome::Success(ApiKey),
            _ => Outcome::Error((Status::Unauthorized, "Invalid or missing API key")),
        }
    } 
}

#[get("/")]
async fn get_json(_key: ApiKey) -> Option<NamedFile> {
    NamedFile::open(Path::new("data/rooms.json")).await.ok()
}

#[catch(401)]
fn unauthorized() -> &'static str {
    "Unauthorized, provide a valid X-API-Key header"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_json])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![unauthorized])
}