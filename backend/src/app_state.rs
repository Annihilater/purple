use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::{
    config::DatabaseConfig,
    repositories::{CouponRepository, PlanRepository, ServerRepository, UserRepository},
    services::AuthService,
};

/// 应用共享状态
///
/// 包含所有需要在请求处理器间共享的状态，
/// 如数据库连接池、仓库实例、服务实例等
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool, // 简化为 db，因为在 API 中使用 state.db.clone()
    pub user_repository: UserRepository,
    pub plan_repository: PlanRepository,
    pub coupon_repository: CouponRepository,
    pub server_repository: ServerRepository,
    pub auth_service: AuthService,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// 初始化数据库连接池、仓库实例和服务实例
    pub async fn new(database_config: &DatabaseConfig) -> Result<Self> {
        // 创建数据库连接池
        let pool = create_db_pool(database_config).await?;

        // 初始化仓库
        let user_repository = UserRepository::new(pool.clone());
        let plan_repository = PlanRepository::new(pool.clone());
        let coupon_repository = CouponRepository::new(pool.clone());
        let server_repository = ServerRepository::new(pool.clone());

        // 创建服务实例
        let jwt_secret = std::env::var("JWT_SECRET")
            .map_err(|_| anyhow::anyhow!("JWT_SECRET environment variable must be set"))?;
        let auth_service = AuthService::new(user_repository.clone(), jwt_secret);

        Ok(Self {
            db: pool,
            user_repository,
            plan_repository,
            coupon_repository,
            server_repository,
            auth_service,
        })
    }
}

/// 创建数据库连接池
async fn create_db_pool(config: &DatabaseConfig) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.url)
        .await?;

    Ok(pool)
}
