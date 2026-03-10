use crate::auth::{AdminUser, AuthenticatedUser};
use crate::models::*;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{MySql, MySqlPool};

#[get("/")]
pub async fn list(pool: &State<MySqlPool>, _user: AuthenticatedUser) -> Json<ApiResponse<Vec<Department>>> {
    match sqlx::query_as::<_, Department>("SELECT * FROM departments WHERE is_active = true ORDER BY name")
        .fetch_all(pool.inner()).await
    {
        Ok(d) => Json(ApiResponse::ok(d)),
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[get("/<id>")]
pub async fn get(pool: &State<MySqlPool>, _user: AuthenticatedUser, id: i32) -> Json<ApiResponse<Department>> {
    match sqlx::query_as::<_, Department>("SELECT * FROM departments WHERE id = ?")
        .bind(id).fetch_optional(pool.inner()).await
    {
        Ok(Some(d)) => Json(ApiResponse::ok(d)),
        Ok(None) => Json(ApiResponse::err("Not found")),
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[post("/", format = "json", data = "<body>")]
pub async fn create(pool: &State<MySqlPool>, _admin: AdminUser, body: Json<CreateDepartment>) -> Json<ApiResponse<Department>> {
    let result = sqlx::query(
        "INSERT INTO departments (name, head_of_department_id, building_id, description) VALUES (?,?,?,?)"
    )
    .bind(&body.name)
    .bind(body.head_of_department_id)
    .bind(body.building_id)
    .bind(&body.description)
    .execute(pool.inner()).await;

    match result {
        Ok(res) => {
            let id = res.last_insert_id() as i32;
            let dep = sqlx::query_as::<_, Department>("SELECT * FROM departments WHERE id = ?")
                .bind(id)
                .fetch_one(pool.inner())
                .await
                .unwrap();
            Json(ApiResponse::ok(dep))
        }
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[put("/<id>", format = "json", data = "<body>")]
pub async fn update(pool: &State<MySqlPool>, _admin: AdminUser, id: i32, body: Json<serde_json::Value>) -> Json<ApiResponse<Department>> {
    // Dynamic update, only set the fields the client sent
    let obj = match body.as_object() {
        Some(o) => o,
        None => return Json(ApiResponse::err("Invalid body")),
    };

    let mut sets = Vec::new();
    let mut vals: Vec<Box<dyn std::fmt::Display + Send>> = Vec::new();

    if let Some(v) = obj.get("name").and_then(|v| v.as_str()) {
        sets.push("name = ?"); vals.push(Box::new(v.to_string()));
    }
    if let Some(v) = obj.get("description").and_then(|v| v.as_str()) {
        sets.push("description = ?"); vals.push(Box::new(v.to_string()));
    }
    if let Some(v) = obj.get("head_of_department_id").and_then(|v| v.as_i64()) {
        sets.push("head_of_department_id = ?"); vals.push(Box::new(v));
    }
    if let Some(v) = obj.get("building_id").and_then(|v| v.as_i64()) {
        sets.push("building_id = ?"); vals.push(Box::new(v));
    }

    if sets.is_empty() {
        return Json(ApiResponse::err("No valid fields to update"));
    }

    let sql = format!("UPDATE departments SET {} WHERE id = ?", sets.join(", "));

    let mut q = sqlx::query(&sql);
    for val in &vals {
        q = q.bind(val.to_string());
    }
    q = q.bind(id);
    let _ = q.execute(pool.inner()).await;

    let d = sqlx::query_as::<_, Department>("SELECT * FROM departments WHERE id = ?")
        .bind(id)
        .fetch_one(pool.inner())
        .await
        .unwrap();

    Json(ApiResponse::ok(d))
}

#[delete("/<id>")]
pub async fn delete(pool: &State<MySqlPool>, _admin: AdminUser, id: i32) -> Json<ApiResponse<String>> {
    let _ = sqlx::query("UPDATE departments SET is_active = false WHERE id = ?")
        .bind(id)
        .execute(pool.inner())
        .await;
    
    Json(ApiResponse::ok("Deactivated".to_string()))
}