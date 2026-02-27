#[macro_use] extern crate rocket;

use rocket::fs::{NamedFile, FileServer};
use std::path::Path;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

struct ApiKey;

// Json submitted by user
#[derive(Debug, Deserialise, Serialise, Clone)]
pub struct RoomsJson {
    pub room_name: String,
    pub building: String,
    pub capacity: i64,
    pub floor: i64,
    pub room_type: String,
    pub department_exclusive: bool,
    pub features: Vec<String>,
    pub accessibility: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Room {
     pub id: i64,
    pub room_name: String,
    pub building: String,
    pub capacity: i64,
    pub floor: i64,
    pub room_type: String,
    pub department_exclusive: bool,
    #[sqlx(json)]
    pub features: Vec<String>,
    #[sqlx(json)]
    pub accessibility: Vec<String>,
}

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
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/api")]
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
        .mount("/", routes![index, get_json])
        .mount("/static", FileServer::from("static"))
        .register("/", catchers![unauthorized])
}