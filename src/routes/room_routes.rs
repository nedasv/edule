use crate::auth::{AdminUser, AuthenticatedUser};
use crate::models::*;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::{MySql, MySqlPool};

#[get("/buildings")]
pub async fn list_buildings(pool: &State<MySqlPool>, _user: AuthenticatedUser) -> Json<ApiResponse<Vec<Building>>> {
    match sqlx::query_as::<_, Building>("SELECT * FROM buildings ORDER BY name")
        .fetch_all(pool.inner())
        .await
    {
        Ok(b) => Json(ApiResponse::ok(b)),
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[post("/buildings", format = "json", data = "<body>")]
pub async fn create_building(pool: &State<MySqlPool>, _admin: AdminUser, body: Json<CreateBuilding>) -> Json<ApiResponse<Building>> {
    let result = sqlx::query(
        "INSERT INTO buildings (name, code, latitude, longitude, address, campus, floors, has_elevator, has_wheelchair_access) VALUES (?,?,?,?,?,?,?,?,?)"
    )
    .bind(&body.name).
    bind(&body.code)
    .bind(body.latitude)
    .bind(body.longitude)
    .bind(&body.address)
    .bind(&body.campus)
    .bind(body.floors.unwrap_or(1))
    .bind(body.has_elevator.unwrap_or(false))
    .bind(body.has_wheelchair_access.unwrap_or(false))
    .execute(pool.inner())
    .await;

    match result {
        Ok(r) => {
            let id = r.last_insert_id() as i32;
            let b = sqlx::query_as::<_, Building>("SELECT * FROM buildings WHERE id = ?")
                .bind(id).fetch_one(pool.inner()).await.unwrap();
            Json(ApiResponse::ok(b))
        }
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[get("/?<building_id>&<room_type_id>&<min_capacity>")]
pub async fn list_rooms(
    pool: &State<MySqlPool>, _user: AuthenticatedUser,
    building_id: Option<i32>, room_type_id: Option<i32>, min_capacity: Option<i32>,
) -> Json<ApiResponse<Vec<Room>>> {
    let mut sql = "SELECT * FROM rooms WHERE 1=1".to_string();
    if building_id.is_some() { sql += " AND building_id = ?"; }
    if room_type_id.is_some() { sql += " AND room_type_id = ?"; }
    if min_capacity.is_some() { sql += " AND capacity >= ?"; }
    sql += " ORDER BY code";

    let mut q = sqlx::query_as::<_, Room>(&sql);
    if let Some(v) = building_id { q = q.bind(v); }
    if let Some(v) = room_type_id { q = q.bind(v); }
    if let Some(v) = min_capacity { q = q.bind(v); }

    match q.fetch_all(pool.inner()).await {
        Ok(rooms) => Json(ApiResponse::ok(rooms)),
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[get("/<id>")]
pub async fn get_room(pool: &State<MySqlPool>, _user: AuthenticatedUser, id: i32) -> Json<ApiResponse<Room>> {
    match sqlx::query_as::<_, Room>("SELECT * FROM rooms WHERE id = ?")
        .bind(id).fetch_optional(pool.inner()).await
    {
        Ok(Some(r)) => Json(ApiResponse::ok(r)),
        Ok(None) => Json(ApiResponse::err("Room not found")),
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[post("/", format = "json", data = "<body>")]
pub async fn create_room(pool: &State<MySqlPool>, _admin: AdminUser, body: Json<CreateRoom>) -> Json<ApiResponse<Room>> {
    let days_json = body.available_days.as_ref().map(|d| serde_json::to_string(d).unwrap_or_default());

    let result = sqlx::query(
        r#"INSERT INTO rooms (building_id, room_type_id, name, code, capacity, floor_number,
            has_projector, has_whiteboard, has_smartboard, has_video_conferencing,
            has_recording_equipment, has_computers, computer_count, has_lab_equipment,
            has_wheelchair_access, has_hearing_loop, has_air_conditioning, has_natural_light,
            power_outlets, available_from, available_until, available_days, maintenance_status, notes)
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"#
    )
    .bind(body.building_id)
    .bind(body.room_type_id)
    .bind(&body.name)
    .bind(&body.code)
    .bind(body.capacity)
    .bind(body.floor_number.unwrap_or(0))
    .bind(body.has_projector.unwrap_or(false))
    .bind(body.has_whiteboard.unwrap_or(false))
    .bind(body.has_smartboard.unwrap_or(false))
    .bind(body.has_video_conferencing.unwrap_or(false))
    .bind(body.has_recording_equipment.unwrap_or(false))
    .bind(body.has_computers.unwrap_or(false))
    .bind(body.computer_count.unwrap_or(0))
    .bind(body.has_lab_equipment.unwrap_or(false))
    .bind(body.has_wheelchair_access.unwrap_or(false))
    .bind(body.has_hearing_loop.unwrap_or(false))
    .bind(body.has_air_conditioning.unwrap_or(false))
    .bind(body.has_natural_light.unwrap_or(false))
    .bind(body.power_outlets.unwrap_or(0))
    .bind(&body.available_from)
    .bind(&body.available_until)
    .bind(&days_json)
    .bind(body.maintenance_status.as_deref().unwrap_or("operational"))
    .bind(&body.notes)
    .execute(pool.inner())
    .await;

    match result {
        Ok(r) => {
            let id = r.last_insert_id() as i32;
            let room = sqlx::query_as::<_, Room>("SELECT * FROM rooms WHERE id = ?")
                .bind(id).fetch_one(pool.inner()).await.unwrap();
            Json(ApiResponse::ok(room))
        }
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[put("/<id>", format = "json", data = "<body>")]
pub async fn update_room(pool: &State<MySqlPool>, _admin: AdminUser, id: i32, body: Json<UpdateRoom>) -> Json<ApiResponse<Room>> {
    let days_json = body.available_days.as_ref().map(|d| serde_json::to_string(d).unwrap_or_default());

    let _ = sqlx::query(
        r#"UPDATE rooms SET
            name = COALESCE(?, name), room_type_id = COALESCE(?, room_type_id),
            capacity = COALESCE(?, capacity),
            has_projector = COALESCE(?, has_projector), has_whiteboard = COALESCE(?, has_whiteboard),
            has_smartboard = COALESCE(?, has_smartboard), has_video_conferencing = COALESCE(?, has_video_conferencing),
            has_recording_equipment = COALESCE(?, has_recording_equipment),
            has_computers = COALESCE(?, has_computers), computer_count = COALESCE(?, computer_count),
            has_lab_equipment = COALESCE(?, has_lab_equipment),
            has_wheelchair_access = COALESCE(?, has_wheelchair_access),
            has_hearing_loop = COALESCE(?, has_hearing_loop),
            has_air_conditioning = COALESCE(?, has_air_conditioning),
            has_natural_light = COALESCE(?, has_natural_light),
            power_outlets = COALESCE(?, power_outlets),
            available_from = COALESCE(?, available_from), available_until = COALESCE(?, available_until),
            available_days = COALESCE(?, available_days),
            maintenance_status = COALESCE(?, maintenance_status), notes = COALESCE(?, notes)
        WHERE id = ?"#
    )
    .bind(&body.name)
    .bind(body.room_type_id)
    .bind(body.capacity)
    .bind(body.has_projector)
    .bind(body.has_whiteboard)
    .bind(body.has_smartboard)
    .bind(body.has_video_conferencing)
    .bind(body.has_recording_equipment)
    .bind(body.has_computers)
    .bind(body.computer_count)
    .bind(body.has_lab_equipment)
    .bind(body.has_wheelchair_access)
    .bind(body.has_hearing_loop)
    .bind(body.has_air_conditioning)
    .bind(body.has_natural_light)
    .bind(body.power_outlets)
    .bind(&body.available_from)
    .bind(&body.available_until)
    .bind(&days_json)
    .bind(&body.maintenance_status)
    .bind(&body.notes)
    .bind(id)
    .execute(pool.inner())
    .await;

    let room = sqlx::query_as::<_, Room>("SELECT * FROM rooms WHERE id = ?")
        .bind(id).fetch_one(pool.inner()).await.unwrap();
    Json(ApiResponse::ok(room))
}

#[delete("/<id>")]
pub async fn delete_room(pool: &State<MySqlPool>, _admin: AdminUser, id: i32) -> Json<ApiResponse<String>> {
    match sqlx::query("DELETE FROM rooms WHERE id = ?").bind(id).execute(pool.inner()).await {
        Ok(_) => Json(ApiResponse::ok("Deleted".to_string())),
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}

#[post("/import/json", format = "json", data = "<body>")]
pub async fn import_rooms_json(pool: &State<MySqlPool>, _admin: AdminUser, body: Json<Vec<CreateRoom>>) -> Json<ApiResponse<String>> {
    let mut count = 0u32;
    for room in body.into_inner() {
        let _ = sqlx::query(
            "INSERT INTO rooms (building_id, room_type_id, name, code, capacity, has_projector, has_computers, has_lab_equipment) VALUES (?,?,?,?,?,?,?,?)"
        )
        .bind(room.building_id)
        .bind(room.room_type_id)
        .bind(&room.name)
        .bind(&room.code)
        .bind(room.capacity)
        .bind(room.has_projector.unwrap_or(false))
        .bind(room.has_computers.unwrap_or(false))
        .bind(room.has_lab_equipment.unwrap_or(false))
        .execute(pool.inner())
        .await;
        count += 1;
    }
    Json(ApiResponse::ok(format!("Imported {} rooms", count)))
}