#[macro_use] extern crate rocket;

use dotenvy::dotenv;
mod db;
mod models;

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
}