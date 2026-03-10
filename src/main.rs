#[macro_use] 
extern crate rocket;

use dotenvy::dotenv;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

mod db;
mod models;
mod auth;
mod routes;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info { name: "CORS Headers", kind: Kind::Response }
    }
    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, PATCH, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
        res.set_header(Header::new("Access-Control-Max-Age", "86400"));
    }
}

#[options("/<_..>")]
fn cors_preflight() -> &'static str { "" }

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    // Loads variables from .env file
    dotenv().ok(); 

    let pool = db::init_pool().await.expect("Failed to create db pool");

    rocket::build()
        .manage(pool)
        .attach(Cors)
        .mount("/", routes![index])
        .mount("/api/auth", routes![
            routes::auth_routes::login,
            routes::auth_routes::register,
            routes::auth_routes::me,
        ])
        .mount("/api/departments", routes![
            routes::department_routes::list,
            routes::department_routes::get,
            routes::department_routes::create,
            routes::department_routes::update,
            routes::department_routes::delete,
        ])
        .mount("/api/room-types", routes![
            routes::room_type_routes::list,
            routes::room_type_routes::create,
            routes::room_type_routes::delete,
            routes::room_type_routes::update, 
        ])
        .mount("/api/rooms", routes![
            routes::room_routes::list_buildings,
            routes::room_routes::create_building,
            routes::room_routes::list_rooms,
            routes::room_routes::get_room,
            routes::room_routes::create_room,
            routes::room_routes::update_room,
            routes::room_routes::delete_room,
            routes::room_routes::import_rooms_json,
        ])
        .mount("/api/test", routes![
            routes::test_routes::ping,
            routes::test_routes::test_users,
            routes::test_routes::insert_user,
            routes::test_routes::create_token_test, // curl http://localhost:8000/api/test/create-token
            routes::test_routes::verify_token_test, // curl http://localhost:8000/api/test/verify-token/TOKEN_HERE
            routes::test_routes::invalid_token_test, // curl http://localhost:8000/api/test/verify-invalid-token
            routes::test_routes::get_tokens,
        ])
}