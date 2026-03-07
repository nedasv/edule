use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

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

