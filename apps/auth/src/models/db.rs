use std::net::IpAddr;

use chrono::{DateTime, Utc}; 
use sqlx::FromRow;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;


#[derive(sqlx::Type, Debug, Clone, Copy, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct UserUuid(pub Uuid);

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct EmailAddress(pub String);

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct PasswordHash(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub uuid: UserUuid,
    pub username: String,
    pub email: EmailAddress,
    pub password_hash: PasswordHash,

    pub role: String,
    pub is_active: bool,
    pub is_verified: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSecurityLog {
    pub user_uuid: UserUuid,
    pub recent_failed_login_attempts: i32,
    pub total_successful_login_attempts: i32,
    pub total_failed_login_attempts: i32,
    pub lockout_until: Option<DateTime<Utc>>,

    pub last_successful_login_at: Option<DateTime<Utc>>,
    pub last_failed_login_at: Option<DateTime<Utc>>,
    pub last_password_change_at: Option<DateTime<Utc>>,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,

    pub last_successful_login_ip: Option<IpAddr>,
    pub last_failed_login_ip: Option<IpAddr>,
    pub last_successful_login_user_agent: Option<String>,
    pub last_failed_login_user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RefreshToken {
    pub id: i32,
    pub subject: UserUuid,
    pub token: String,
    pub issued_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub revoked: bool,
    pub revoked_at: Option<DateTime<Utc>>,
}