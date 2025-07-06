use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub password_algo: Option<String>,
    pub password_salt: Option<String>,
    pub telegram_id: Option<i64>,
    pub invite_user_id: Option<i32>,
    pub balance: Option<i32>,
    pub discount: Option<i32>,
    pub commission_type: Option<bool>,
    pub commission_rate: Option<i32>,
    pub commission_balance: Option<i32>,
    pub t: Option<i32>,
    pub u: Option<i64>,
    pub d: Option<i64>,
    pub transfer_enable: Option<i64>,
    pub banned: Option<bool>,
    pub is_admin: Option<bool>,
    pub is_staff: Option<bool>,
    pub last_login_at: Option<i32>,
    pub last_login_ip: Option<i32>,
    pub uuid: String,
    pub group_id: Option<i32>,
    pub plan_id: Option<i32>,
    pub speed_limit: Option<i32>,
    pub token: String,
    pub remind_expire: Option<bool>,
    pub remind_traffic: Option<bool>,
    pub expired_at: Option<i64>,
    pub remarks: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub invite_user_id: Option<i32>,
    pub uuid: String,
    pub token: String,
}
