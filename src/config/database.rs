use config::Config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
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
        let config = Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        Ok(Self {
            url: config.get_string("database_url")?,
            max_connections: config.get_int("database_max_connections").unwrap_or(5) as u32,
            min_connections: config.get_int("database_min_connections").unwrap_or(1) as u32,
            connect_timeout: Duration::from_secs(
                config.get_int("database_connect_timeout").unwrap_or(10) as u64,
            ),
            max_lifetime: Duration::from_secs(
                config.get_int("database_max_lifetime").unwrap_or(1800) as u64,
            ),
            idle_timeout: Duration::from_secs(
                config.get_int("database_idle_timeout").unwrap_or(300) as u64,
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
