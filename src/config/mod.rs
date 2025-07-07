use anyhow::Result;
use config;

pub mod admin;
pub mod database;
pub use admin::{get_admin_info, initialize_admin_account, verify_admin_account, AdminInfo};
pub use database::DatabaseConfig;

#[derive(Debug)]
pub struct LogConfig {
    pub level: String,
    pub with_thread_ids: bool,
    pub with_line_number: bool,
    pub with_file: bool,
    pub with_target: bool,
    pub file_path: String,
}

#[derive(Debug)]
pub struct AdminConfig {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Config {
    pub server_addr: String,
    pub server_port: u16,
    pub database: DatabaseConfig,
    pub log: LogConfig,
    pub admin: AdminConfig,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        Ok(Self {
            server_addr: config
                .get_string("server_addr")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: config.get_int("server_port").unwrap_or(8080) as u16,
            database: DatabaseConfig::from_env()?,
            log: LogConfig {
                level: config
                    .get_string("log_level")
                    .unwrap_or_else(|_| "info".to_string()),
                with_thread_ids: config.get_bool("log_with_thread_ids").unwrap_or(true),
                with_line_number: config.get_bool("log_with_line_number").unwrap_or(true),
                with_file: config.get_bool("log_with_file").unwrap_or(true),
                with_target: config.get_bool("log_with_target").unwrap_or(false),
                file_path: config
                    .get_string("log_file_path")
                    .unwrap_or_else(|_| "logs/app.log".to_string()),
            },
            admin: AdminConfig {
                email: config
                    .get_string("admin_email")
                    .unwrap_or_else(|_| "admin@example.com".to_string()),
                password: config
                    .get_string("admin_password")
                    .unwrap_or_else(|_| "admin123".to_string()),
            },
        })
    }
}
