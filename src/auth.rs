use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::{request::FromRequest, serde::json};
use serde::{Deserialize, Serialize};
use rocket::request::{Outcome, Request};
use rocket::http::Status;

const JWT_SECRET: &[u8] = b"test_secret";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // userid
    pub role: String, // user role
    pub username: String,
    pub exp: usize // expiry
}

pub fn create_token(user_id: i32, role: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        role: role.to_string(),
        username: username.to_string(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token, 
        &DecodingKey::from_secret(JWT_SECRET), 
        &Validation::default()
    )?;

    Ok(data.claims)
}

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub role: String,
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        match auth_header {
            Some(header) => {
                let token = header.trim_start_matches("Bearer ").trim();
                match verify_token(token) {
                    Ok(claims) => Outcome::Success(AuthenticatedUser {
                        user_id: claims.sub,
                        role: claims.role,
                        username: claims.username,
                    }),
                    Err(_) => Outcome::Error((Status::Unauthorized, "Invalid token")),
                }
            }
            None => Outcome::Error((Status::Unauthorized, "Missing authorization header")),
        }
    }
}

pub struct StaffOrAdmin {
    pub user_id: i32,
    pub role: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StaffOrAdmin {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<AuthenticatedUser>().await {
            Outcome::Success(u) if u.role == "staff" || u.role == "admin" => {
                Outcome::Success(StaffOrAdmin { user_id: u.user_id, role: u.role })
            }
            Outcome::Success(_) => Outcome::Error((Status::Forbidden, "Staff or admin required")),
            Outcome::Error(e) => Outcome::Error(e),
            Outcome::Forward(f) => Outcome::Forward(f),
        }
    }
}

pub struct AdminUser {
    pub user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<AuthenticatedUser>().await {
            Outcome::Success(u) if u.role == "admin" => {
                Outcome::Success(AdminUser { user_id: u.user_id })
            }
            Outcome::Success(_) => Outcome::Error((Status::Forbidden, "Admin required")),
            Outcome::Error(e) => Outcome::Error(e),
            Outcome::Forward(f) => Outcome::Forward(f),
        }
    }
}