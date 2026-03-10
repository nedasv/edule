use crate::auth::{AdminUser, AuthenticatedUser};
use crate::models::*;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{MySql, MySqlPool};

#[get("/")]
pub async fn list(pool: &State<MySqlPool>, _user: AuthenticatedUser) -> Json<ApiResponse<Vec<RoomType>>> {
    match sqlx::query_as::<_, RoomType>("SELECT * FROM room_types ORDER BY name")
        .fetch_all(pool.inner())
        .await
    {
        Ok(rt) => Json(ApiResponse::ok(rt)),
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[post("/", format = "json", data = "<body>")]
pub async fn create(pool: &State<MySqlPool>, _admin: AdminUser, body: Json<CreateRoomType>) -> Json<ApiResponse<RoomType>> {
    let result = sqlx::query("INSERT INTO room_types (name, description) VALUES (?,?)")
        .bind(&body.name)
        .bind(&body.description)
        .execute(pool.inner())
        .await;

    match result {
        Ok(r) => {
            let id = r.last_insert_id() as i32;
            let rt = sqlx::query_as::<_, RoomType>("SELECT * FROM room_types WHERE id = ?")
                .bind(id)
                .fetch_one(pool.inner())
                .await
                .unwrap();
            Json(ApiResponse::ok(rt))
        }
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[delete("/<id>")]
pub async fn delete(pool: &State<MySqlPool>, _admin: AdminUser, id: i32) -> Json<ApiResponse<String>> {
    // Prevent deleting a type that's in use
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM rooms WHERE room_type_id = ?")
        .bind(id)
        .fetch_one(pool.inner())
        .await
        .unwrap_or((0,));

    if count.0 > 0 {
        return Json(ApiResponse::err(&format!("Cannot delete: {} rooms use this type", count.0)));
    }

    let _ = sqlx::query("DELETE FROM room_types WHERE id = ?")
        .bind(id)
        .execute(pool.inner())
        .await;
    
    Json(ApiResponse::ok("Deleted".to_string()))
}

#[put("/<id>", format = "json", data = "<body>")]
pub async fn update(
    pool: &State<MySqlPool>,
    _admin: AdminUser,
    id: i32,
    body: Json<CreateRoomType>,
) -> Json<ApiResponse<RoomType>> {
    let result = sqlx::query("UPDATE room_types SET name = ?, description = ? WHERE id = ?")
        .bind(&body.name)
        .bind(&body.description)
        .bind(id)
        .execute(pool.inner())
        .await;

    match result {
        Ok(r) => {
            if r.rows_affected() == 0 {
                return Json(ApiResponse::err("Room type not found"));
            }
            let rt = sqlx::query_as::<_, RoomType>("SELECT * FROM room_types WHERE id = ?")
                .bind(id)
                .fetch_one(pool.inner())
                .await
                .unwrap();
            Json(ApiResponse::ok(rt))
        }
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}