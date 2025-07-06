use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub max_lifetime: Duration,
    pub idle_timeout: Duration,
}

impl DatabaseConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            min_connections: env::var("DATABASE_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "1".to_string())
                .parse()?,
            connect_timeout: Duration::from_secs(
                env::var("DATABASE_CONNECT_TIMEOUT")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()?
            ),
            max_lifetime: Duration::from_secs(
                env::var("DATABASE_MAX_LIFETIME")
                    .unwrap_or_else(|_| "1800".to_string())
                    .parse()?
            ),
            idle_timeout: Duration::from_secs(
                env::var("DATABASE_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "300".to_string())
                    .parse()?
            ),
        })
    }

    pub async fn create_pool(&self) -> anyhow::Result<DbPool> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .acquire_timeout(self.connect_timeout)
            .max_lifetime(Some(self.max_lifetime))
            .idle_timeout(Some(self.idle_timeout))
            .connect(&self.url)
            .await?;

        Ok(pool)
    }
} 