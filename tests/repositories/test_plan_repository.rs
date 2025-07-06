use crate::models::plan::{CreatePlanRequest, UpdatePlanRequest};
use crate::repositories::PlanRepository;
use chrono::Utc;
use mockall::predicate::*;
use mockall::*;
use sqlx::PgPool;

mock! {
    PgPool {}

    impl PgPool {
        fn acquire(&self) -> Result<PoolConnection<Postgres>, Error>;
    }
}

#[tokio::test]
async fn test_create_plan() {
    let pool = MockPgPool::new();
    let repo = PlanRepository::new(pool);

    let request = CreatePlanRequest {
        name: "测试套餐".to_string(),
        price: 1000,
        traffic: 10737418240, // 10GB
        speed_limit: Some(100),
        device_limit: Some(3),
        duration: 30,
        description: Some("测试套餐描述".to_string()),
        sort_order: Some(1),
        status: Some(true),
    };

    let result = repo.create(&request).await;
    assert!(result.is_ok());

    let plan = result.unwrap();
    assert_eq!(plan.name, request.name);
    assert_eq!(plan.price, request.price);
    assert_eq!(plan.traffic, request.traffic);
    assert_eq!(plan.speed_limit, request.speed_limit);
    assert_eq!(plan.device_limit, request.device_limit);
    assert_eq!(plan.duration, request.duration);
    assert_eq!(plan.description, request.description);
    assert_eq!(plan.sort_order, request.sort_order);
    assert_eq!(plan.status, request.status.unwrap_or(true));
}

#[tokio::test]
async fn test_update_plan() {
    let pool = MockPgPool::new();
    let repo = PlanRepository::new(pool);

    let request = UpdatePlanRequest {
        name: Some("更新套餐".to_string()),
        price: Some(2000),
        traffic: Some(21474836480), // 20GB
        speed_limit: Some(200),
        device_limit: Some(5),
        duration: Some(60),
        description: Some("更新套餐描述".to_string()),
        sort_order: Some(2),
        status: Some(false),
    };

    let result = repo.update(1, &request).await;
    assert!(result.is_ok());

    let plan = result.unwrap();
    assert_eq!(plan.name, request.name.unwrap());
    assert_eq!(plan.price, request.price.unwrap());
    assert_eq!(plan.traffic, request.traffic.unwrap());
    assert_eq!(plan.speed_limit, request.speed_limit);
    assert_eq!(plan.device_limit, request.device_limit);
    assert_eq!(plan.duration, request.duration.unwrap());
    assert_eq!(plan.description, request.description);
    assert_eq!(plan.sort_order, request.sort_order);
    assert_eq!(plan.status, request.status.unwrap());
}

#[tokio::test]
async fn test_find_by_id() {
    let pool = MockPgPool::new();
    let repo = PlanRepository::new(pool);

    let result = repo.find_by_id(1).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_find_all() {
    let pool = MockPgPool::new();
    let repo = PlanRepository::new(pool);

    let result = repo.find_all(1, 10).await;
    assert!(result.is_ok());

    let (plans, total) = result.unwrap();
    assert!(!plans.is_empty());
    assert!(total > 0);
}

#[tokio::test]
async fn test_delete() {
    let pool = MockPgPool::new();
    let repo = PlanRepository::new(pool);

    let result = repo.delete(1).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[tokio::test]
async fn test_find_enabled() {
    let pool = MockPgPool::new();
    let repo = PlanRepository::new(pool);

    let result = repo.find_enabled().await;
    assert!(result.is_ok());

    let plans = result.unwrap();
    assert!(!plans.is_empty());
    for plan in plans {
        assert!(plan.status);
    }
}

#[tokio::test]
async fn test_find_by_ids() {
    let pool = MockPgPool::new();
    let repo = PlanRepository::new(pool);

    let ids = vec![1, 2, 3];
    let result = repo.find_by_ids(&ids).await;
    assert!(result.is_ok());

    let plans = result.unwrap();
    assert!(!plans.is_empty());
    for plan in plans {
        assert!(ids.contains(&plan.id));
    }
} 