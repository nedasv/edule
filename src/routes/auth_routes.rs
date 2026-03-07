use sqlx::MySqlPool;
use crate::{auth, models::*};
use rocket::serde::json::Json;
use rocket::State;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::auth::AuthenticatedUser;



#[post("/login", format = "json", data = "<body>")]
pub async fn login(pool: &State<MySqlPool>, body: Json<LoginRequest>) -> Json<ApiResponse<AuthResponse>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(&body.username)
        .fetch_optional(pool.inner())
        .await;

    match user {
        Ok(Some(user)) => {
            if verify(&body.password, &user.password_hash).unwrap_or(false) {
                match auth::create_token(user.id, &user.role, &user.username) {
                    Ok(token) => Json(ApiResponse::ok(AuthResponse {
                        token,
                        user: user.into(),
                    })),
                    Err(_) => Json(ApiResponse::err("Failed to create token"))
                }
            } else {
                Json(ApiResponse::err("Invalid details"))
            }
        },
        Ok(None) => Json(ApiResponse::err("Invalid details")),
        Err(e) => Json(ApiResponse::err(&format!("Database error: {}", e)))
    }
}

#[post("/register", format = "json", data = "<body>")]
pub async fn register(pool: &State<MySqlPool>, body: Json<RegisterRequest>) -> Json<ApiResponse<AuthResponse>> {
    let password_hash = match hash(&body.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return Json(ApiResponse::err("Failed to create hash"))
    };

    let role = body.role.clone().unwrap_or_else(|| String::from("student"));

    let result = sqlx::query(
        "INSERT INTO users (username, email, password_hash, full_name, role, department_id, student_id, staff_id) VALUES (?,?,?,?,?,?,?,?)"
    )
        .bind(&body.username)
        .bind(&body.email)
        .bind(&password_hash)
        .bind(&body.full_name)
        .bind(&role)
        .bind(body.department_id)
        .bind(&body.student_id)
        .bind(&body.staff_id)
        .execute(pool.inner())
        .await;

    match result {
        Ok(r) => {
            let new_id = r.last_insert_id() as i32;
            let token = auth::create_token(new_id, &role, &body.username).unwrap();
            Json(ApiResponse::ok(AuthResponse {
                token,
                user: UserPublic {
                    id: new_id,
                    username: body.username.clone(),
                    email: body.email.clone(),
                    full_name: body.full_name.clone(),
                    role,
                    department_id: body.department_id,
                    student_id: body.student_id.clone(),
                    staff_id: body.staff_id.clone(),
                },
            }))
        }
        Err(e) => Json(ApiResponse::err(&format!("Registration failed: {}", e))),
    }
}

#[get("/me")]
pub async fn me(pool: &State<MySqlPool>, user: AuthenticatedUser,) -> Json<ApiResponse<UserPublic>> {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user.user_id)
        .fetch_optional(pool.inner())
        .await
    {
        Ok(Some(u)) => Json(ApiResponse::ok(u.into())),
        Ok(None) => Json(ApiResponse::err("User not found")),
        Err(e) => Json(ApiResponse::err(&format!("{}", e))),
    }
}