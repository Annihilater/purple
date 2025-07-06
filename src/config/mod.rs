use std::env;
use dotenv::dotenv;

mod database;
pub use database::{DatabaseConfig, DbPool};

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: String,
    pub server_port: u16,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv().ok();
        
        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;
        let database = DatabaseConfig::from_env()?;

        Ok(Self {
            server_addr,
            server_port,
            database,
        })
    }
} 