use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use sqlx::MySqlPool;
use crate::models::User;

#[get("/ping")]
pub async fn ping(pool: &State<MySqlPool>) -> Json<Value> {
    match sqlx::query_scalar::<_, i64>("SELECT 1").fetch_one(pool.inner()).await {
        Ok(_) => Json(json!({"status": "ok", "message": "database connected"})),
        Err(e) => Json(json!({"status": "error", "message": e.to_string()})),
    }
}

#[get("/users")]
pub async fn test_users(pool: &State<MySqlPool>) -> Json<Value> {
    match sqlx::query_as::<_, User>("SELECT * FROM users").fetch_all(pool.inner()).await {
        Ok(rows) => Json(json!({ "status": "ok", "count": rows.len(), "data": rows })),
        Err(e) => Json(json!({ "status": "error", "message": e.to_string() })),
    }
}

#[get("/insert-user")]
pub async fn insert_user(pool: &State<MySqlPool>) -> Json<Value> {
    let hash = bcrypt::hash("testpassword", 4).unwrap();
    let result = sqlx::query(
        "INSERT INTO users (username, email, password_hash, role, full_name)
         VALUES ('testuser', 'test@goldsmiths.ac.uk', ?, 'student', 'Test User')"
    )
    .bind(&hash)
    .execute(pool.inner())
    .await;

    match result {
        Ok(r) => Json(json!({ "status": "ok", "inserted_id": r.last_insert_id() as i32 })),
        Err(e) => Json(json!({ "status": "error", "message": e.to_string() })),
    }
}