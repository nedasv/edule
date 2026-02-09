#[macro_use] extern crate rocket;

use rocket::fs::NamedFile;
use std::path::Path;

#[get("/")]
async fn get_json() -> Option<NamedFile> {
    NamedFile::open(Path::new("data/rooms.json")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_json])
}