#[macro_use] extern crate rocket;

use dotenvy::dotenv;
mod db;
mod models;
mod auth;
mod routes;

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
        .mount("/", routes![index])
        .mount("/api/test", routes![
            routes::test_routes::ping,
            routes::test_routes::test_users,
            routes::test_routes::insert_user,
        ])
}