use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub invite_user_id: Option<i32>,
    pub telegram_id: Option<i64>,
    pub email: String,
    pub password: String,
    pub password_algo: Option<String>,
    pub password_salt: Option<String>,
    pub balance: i32,
    pub discount: Option<i32>,
    pub commission_type: bool,
    pub commission_rate: Option<i32>,
    pub commission_balance: i32,
    pub t: i32,
    pub u: i64,
    pub d: i64,
    pub transfer_enable: i64,
    pub banned: bool,
    pub is_admin: bool,
    pub last_login_at: Option<i32>,
    pub is_staff: bool,
    pub last_login_ip: Option<i32>,
    pub uuid: String,
    pub group_id: Option<i32>,
    pub plan_id: Option<i32>,
    pub speed_limit: Option<i32>,
    pub remind_expire: bool,
    pub remind_traffic: bool,
    pub token: String,
    pub expired_at: i64,
    pub remarks: Option<String>,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub invite_user_id: Option<i32>,
    pub uuid: String,
    pub token: String,
} 