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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Building {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
    pub campus: Option<String>,
    pub floors: Option<i32>,
    pub has_elevator: bool,
    pub has_wheelchair_access: bool,
}

#[derive(Debug, Deserialize)]   
pub struct CreateBuilding {
    pub name: String,
    pub code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub address: Option<String>,
    pub campus: Option<String>,
    pub floors: Option<i32>,
    pub has_elevator: Option<bool>,
    pub has_wheelchair_access: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Room {
    pub id: i32,
    pub building_id: i32,
    pub room_type_id: i32,
    pub name: String,
    pub code: String,
    pub capacity: i32,
    pub floor_number: Option<i32>,
    pub has_projector: bool,
    pub has_whiteboard: bool,
    pub has_smartboard: bool,
    pub has_video_conferencing: bool,
    pub has_recording_equipment: bool,
    pub has_computers: bool,
    pub computer_count: Option<i32>,
    pub has_lab_equipment: bool,
    pub has_wheelchair_access: bool,
    pub has_hearing_loop: bool,
    pub has_air_conditioning: bool,
    pub has_natural_light: bool,
    pub power_outlets: Option<i32>,
    pub available_from: Option<NaiveTime>,
    pub available_until: Option<NaiveTime>,
    pub available_days: Option<serde_json::Value>,
    pub maintenance_status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoom {
    pub building_id: i32,
    pub room_type_id: i32,
    pub name: String,
    pub code: String,
    pub capacity: i32,
    pub floor_number: Option<i32>,
    pub has_projector: Option<bool>,
    pub has_whiteboard: Option<bool>,
    pub has_smartboard: Option<bool>,
    pub has_video_conferencing: Option<bool>,
    pub has_recording_equipment: Option<bool>,
    pub has_computers: Option<bool>,
    pub computer_count: Option<i32>,
    pub has_lab_equipment: Option<bool>,
    pub has_wheelchair_access: Option<bool>,
    pub has_hearing_loop: Option<bool>,
    pub has_air_conditioning: Option<bool>,
    pub has_natural_light: Option<bool>,
    pub power_outlets: Option<i32>,
    pub available_from: Option<String>,
    pub available_until: Option<String>,
    pub available_days: Option<Vec<String>>,
    pub maintenance_status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoom {
    pub name: Option<String>,
    pub room_type_id: Option<i32>,
    pub capacity: Option<i32>,
    pub has_projector: Option<bool>,
    pub has_whiteboard: Option<bool>,
    pub has_smartboard: Option<bool>,
    pub has_video_conferencing: Option<bool>,
    pub has_recording_equipment: Option<bool>,
    pub has_computers: Option<bool>,
    pub computer_count: Option<i32>,
    pub has_lab_equipment: Option<bool>,
    pub has_wheelchair_access: Option<bool>,
    pub has_hearing_loop: Option<bool>,
    pub has_air_conditioning: Option<bool>,
    pub has_natural_light: Option<bool>,
    pub power_outlets: Option<i32>,
    pub available_from: Option<String>,
    pub available_until: Option<String>,
    pub available_days: Option<Vec<String>>,
    pub maintenance_status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub department_id: Option<i32>,
    pub session_type: String,
    pub parent_course_id: Option<i32>,
    pub description: Option<String>,
    pub credits: Option<i32>,
    pub year_of_study: Option<i32>,
    pub semester: String,
    pub academic_year: String,
    pub sessions_per_week: Option<i32>,
    pub session_duration_minutes: Option<i32>,
    pub required_room_type_id: Option<i32>,
    pub requires_projector: bool,
    pub requires_computers: bool,
    pub requires_lab_equipment: bool,
    pub requires_video_conferencing: bool,
    pub requires_recording: bool,
    pub min_capacity: Option<i32>,
    pub max_capacity: Option<i32>,
    pub preferred_days: Option<serde_json::Value>,
    pub excluded_days: Option<serde_json::Value>,
    pub preferred_time_start: Option<NaiveTime>,
    pub preferred_time_end: Option<NaiveTime>,
    pub avoid_back_to_back: bool,
    pub instructor_id: Option<i32>,
    pub max_enrollment: Option<i32>,
    pub current_enrollment: Option<i32>,
    pub group_label: Option<String>,
    pub total_groups: Option<i32>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateCourse {
    pub code: String,
    pub name: String,
    pub department_id: Option<i32>,
    pub session_type: String,
    pub parent_course_id: Option<i32>,
    pub description: Option<String>,
    pub credits: Option<i32>,
    pub year_of_study: Option<i32>,
    pub semester: String,
    pub academic_year: String,
    pub sessions_per_week: Option<i32>,
    pub session_duration_minutes: Option<i32>,
    pub required_room_type_id: Option<i32>,
    pub requires_projector: Option<bool>,
    pub requires_computers: Option<bool>,
    pub requires_lab_equipment: Option<bool>,
    pub requires_video_conferencing: Option<bool>,
    pub requires_recording: Option<bool>,
    pub min_capacity: Option<i32>,
    pub max_capacity: Option<i32>,
    pub preferred_days: Option<Vec<String>>,
    pub excluded_days: Option<Vec<String>>,
    pub preferred_time_start: Option<String>,
    pub preferred_time_end: Option<String>,
    pub avoid_back_to_back: Option<bool>,
    pub instructor_id: Option<i32>,
    pub max_enrollment: Option<i32>,
    pub group_label: Option<String>,
    pub total_groups: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CourseWithSessions {
    #[serde(flatten)] // fields merge as json object rather than under course key
    pub course: Course,
    pub sessions: Vec<Course>,
    pub department_name: Option<String>,
    pub room_type_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CourseGroup {
    pub id: i32,
    pub parent_course_id: i32,
    pub child_course_id: i32,
    pub is_mandatory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Timetable {
    pub id: i32,
    pub name: String,
    pub semester: String,
    pub academic_year: String,
    pub status: String,
    pub optimization_score: Option<f64>,
    pub created_by: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TimetableSlot {
    pub id: i32,
    pub timetable_id: i32,
    pub course_id: i32,
    pub room_id: i32,
    pub instructor_id: Option<i32>,
    pub day_of_week: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub slot_type: String,
    pub recurrence: Option<String>,
    pub group_name: Option<String>,
    pub is_locked: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CourseInfo {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub session_type: String,
    pub department_name: String,
    pub parent_course_id: Option<i32>,
    pub group_label: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RoomInfo {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub building_name: String,
    pub building_code: String,
    pub room_type_name: String,
    pub capacity: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub floor_number: Option<i32>,
}

#[derive(Debug, Serialize, Clone)]
pub struct LecturerInfo {
    pub id: i32,
    pub full_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct SlotView {
    pub id: i32,
    pub timetable_id: i32,
    pub day_of_week: String,
    pub start_time: String,
    pub end_time: String,
    pub slot_type: String,
    pub recurrence: Option<String>,
    pub group_name: Option<String>,
    pub is_locked: bool,
    pub notes: Option<String>,
    pub course: Option<CourseInfo>,
    pub room: Option<RoomInfo>,
    pub lecturer: Option<LecturerInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerationConfig {
    pub day_start: Option<String>,
    pub day_end: Option<String>,
    pub slot_duration_minutes: Option<i32>,
    pub days: Option<Vec<String>>,
    pub weight_room_utilization: Option<f64>,
    pub weight_minimize_gaps: Option<f64>,
    pub weight_preferred_times: Option<f64>,
    pub weight_minimize_walking: Option<f64>,
    pub weight_room_type_match: Option<f64>,
    pub weight_equipment_match: Option<f64>,
    pub weight_capacity_fit: Option<f64>,
    pub weight_accessibility: Option<f64>,
    pub weight_instructor_preference: Option<f64>,
    pub weight_spread_across_week: Option<f64>,
    pub max_consecutive_hours: Option<i32>,
    pub min_break_minutes: Option<i32>,
    pub lunch_break_start: Option<String>,
    pub lunch_break_end: Option<String>,
    pub respect_room_availability: Option<bool>,
    pub max_walking_minutes_between_classes: Option<i32>,
    pub walking_speed_kmh: Option<f64>,
    pub ai_optimization_passes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub name: String,
    pub semester: String,
    pub academic_year: String,
    pub config: GenerationConfig,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSlot {
    pub room_id: Option<i32>,
    pub day_of_week: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub lecturer_id: Option<i32>,
    pub notes: Option<String>,
    pub is_locked: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SwapRoomRequest {
    pub slot_id: i32,
    pub new_room_id: i32,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChangeRequest {
    pub id: i32,
    pub requested_by: i32,
    pub slot_id: i32,
    pub change_type: String,
    pub reason: String,
    pub proposed_room_id: Option<i32>,
    pub proposed_day: Option<String>,
    pub proposed_start_time: Option<NaiveTime>,
    pub proposed_end_time: Option<NaiveTime>,
    pub status: String,
    pub reviewed_by: Option<i32>,
    pub review_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateChangeRequest {
    pub slot_id: i32,
    pub change_type: String,
    pub reason: String,
    pub proposed_room_id: Option<i32>,
    pub proposed_day: Option<String>,
    pub proposed_start_time: Option<String>,
    pub proposed_end_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewChangeRequest {
    pub status: String,
    pub review_notes: Option<String>,
}