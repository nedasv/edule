use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

// Api response

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {success: true, data: Some(data), error: None}
    }

    pub fn err(msg: &str) -> Self {
        Self {success: false, data: None, error: Some(msg.to_string())}
    }
}

// Mysql tables

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Department {
    pub id: i32,
    pub name: String,
    pub head_of_department_id: Option<i32>,
    pub building_id: Option<i32>,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateDepartment {
    pub name: String,
    pub head_of_department_id: Option<i32>,
    pub building_id: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RoomType {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomType {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // prevents sending password hash to the client
    pub password_hash: String,
    pub full_name: String,
    pub role: String,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
    pub department_id: Option<i32>,
}

// Login / Register

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub role: Option<String>,
    pub department_id: Option<i32>,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserPublic {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub department_id: Option<i32>,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
}


impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        Self {
            id: u.id, 
            username: u.username, 
            email: u.email,
            full_name: u.full_name, 
            role: u.role,
            department_id: u.department_id,
            student_id: u.student_id, 
            staff_id: u.staff_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}
