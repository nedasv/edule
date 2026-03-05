use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn init_pool() -> Result<MySqlPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "mysql://root:password@localhost:3306/edule_timetable".to_string()
    });

    MySqlPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
}