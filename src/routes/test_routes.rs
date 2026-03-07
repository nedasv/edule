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
