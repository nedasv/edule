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
        .mount("/api/test", routes![
            routes::test_routes::ping,
            routes::test_routes::test_users,
            routes::test_routes::insert_user,
            routes::test_routes::create_token_test, // curl http://localhost:8000/api/test/create-token
            routes::test_routes::verify_token_test, // curl http://localhost:8000/api/test/verify-token/TOKEN_HERE
            routes::test_routes::invalid_token_test, // curl http://localhost:8000/api/test/verify-invalid-token
        ])
}