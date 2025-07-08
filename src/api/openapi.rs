use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{
    health::HealthResponse,
    plan::BatchUpdatePlanStatusRequest,
    subscribe::{
        NodeStatus, NodeStatusResponse, NodeTestResult, SubscribeQuery, TestResultResponse,
        TestSubscribeRequest, TestSummary,
    },
    user::{CreateUserRequest, GetUsersQuery, UpdateUserRequest, UpdateUserStatusRequest},
};
use crate::common::{
    BatchUpdateApiResponse, BatchUpdateData, CouponApiResponse, CouponPageApiResponse,
    CouponValidationApiResponse, CouponValidationData, EmptyApiResponse, ErrorCode,
    HealthApiResponse, HealthData, PlanApiResponse, PlanAvailabilityApiResponse,
    PlanAvailabilityData, PlanListApiResponse, PlanPageApiResponse, PlanPricingApiResponse,
    PlanPricingData, PlanStatsApiResponse, PlanStatsData, ProjectInfoApiResponse, ProjectInfoData,
    TokenApiResponse, UserApiResponse, UserIdApiResponse, UserIdData, UserPageApiResponse,
};
use crate::models::{
    auth::{Claims, LoginRequest, RegisterRequest, TokenResponse},
    coupon::{
        Coupon, CouponListResponse, CouponResponse, CreateCouponRequest, UpdateCouponRequest,
        ValidateCouponResponse,
    },
    info::ProjectInfo,
    plan::{
        CreatePlanRequest, Plan, PlanAvailability, PlanListResponse, PlanPricing, PlanResponse,
        PlanStats, PriceOption, UpdatePlanRequest,
    },
    subscribe::{
        ExpireInfo, LoginStats, PlanInfo, ResetTokenRequest, ResetTokenResponse, ServerNode,
        SubscribeConfig, SubscribeInfoHeader, SubscribeLinkResponse, SubscribeStatsResponse,
        TrafficInfo, TrafficReportRequest, UserStatus, UserSubscribeInfo, UserTrafficData,
    },
    user::{User, UserResponse as UserModel},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::info::get_project_info,
        crate::api::health::health_check,
        crate::api::auth::register,
        crate::api::auth::login,
        crate::api::user::create_user,
        crate::api::user::get_users,
        crate::api::user::get_user,
        crate::api::user::update_user,
        crate::api::user::delete_user,
        crate::api::user::update_user_status,
        crate::api::plan::create_plan,
        crate::api::plan::list_plans,
        crate::api::plan::get_plan,
        crate::api::plan::update_plan,
        crate::api::plan::delete_plan,
        crate::api::plan::get_enabled_plans,
        crate::api::plan::get_plan_stats,
        crate::api::plan::get_plan_pricing,
        crate::api::plan::check_plan_availability,
        crate::api::plan::batch_update_plan_status,
        crate::api::coupon::create_coupon,
        crate::api::coupon::list_coupons,
        crate::api::coupon::get_coupon,
        crate::api::coupon::update_coupon,
        crate::api::coupon::delete_coupon,
        crate::api::coupon::verify_coupon,
        crate::api::subscribe::get_subscribe_info,
        crate::api::subscribe::get_subscribe_link,
        crate::api::subscribe::reset_subscribe_token,
        crate::api::subscribe::get_subscribe_stats,
        crate::api::subscribe::get_subscribe_config,
        crate::api::subscribe::report_traffic,
        crate::api::subscribe::get_nodes_status,
        crate::api::subscribe::test_subscribe_connectivity,
    ),
    components(
        schemas(
            // 用户相关
            User,
            CreateUserRequest,
            UpdateUserRequest,
            UpdateUserStatusRequest,
            GetUsersQuery,
            UserModel,
            UserIdData,
            
            // 套餐相关
            Plan,
            CreatePlanRequest,
            UpdatePlanRequest,
            PlanResponse,
            PlanListResponse,
            PlanStats,
            PlanPricing,
            PlanAvailability,
            PriceOption,
            BatchUpdatePlanStatusRequest,
            PlanStatsData,
            PlanPricingData,
            PlanAvailabilityData,
            BatchUpdateData,
            
            // 优惠券相关
            Coupon,
            CreateCouponRequest,
            UpdateCouponRequest,
            CouponResponse,
            CouponListResponse,
            ValidateCouponResponse,
            CouponValidationData,
            
            // 认证相关
            RegisterRequest,
            LoginRequest,
            TokenResponse,
            Claims,
            
            // 健康检查
            HealthResponse,
            HealthData,
            
            // 项目信息
            ProjectInfo,
            ProjectInfoData,
            
            // 错误和响应类型
            ErrorCode,
            EmptyApiResponse,
            HealthApiResponse,
            ProjectInfoApiResponse,
            TokenApiResponse,
            UserIdApiResponse,
            UserApiResponse,
            UserPageApiResponse,
            PlanApiResponse,
            PlanListApiResponse,
            PlanPageApiResponse,
            PlanStatsApiResponse,
            PlanPricingApiResponse,
            PlanAvailabilityApiResponse,
            BatchUpdateApiResponse,
            CouponApiResponse,
            CouponPageApiResponse,
            CouponValidationApiResponse,
            
            // 订阅相关的Schema
            UserSubscribeInfo,
            TrafficInfo,
            ExpireInfo,
            PlanInfo,
            UserStatus,
            ResetTokenRequest,
            ResetTokenResponse,
            SubscribeConfig,
            SubscribeInfoHeader,
            ServerNode,
            TrafficReportRequest,
            UserTrafficData,
            SubscribeLinkResponse,
            SubscribeStatsResponse,
            LoginStats,
            NodeStatusResponse,
            NodeStatus,
            TestSubscribeRequest,
            TestResultResponse,
            NodeTestResult,
            TestSummary,
            SubscribeQuery,
        )
    ),
    tags(
        (name = "project_info", description = "Project information endpoints"),
        (name = "health", description = "Health check endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "plans", description = "Plan management endpoints"),
        (name = "coupons", description = "Coupon management endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "subscribe", description = "Subscription management endpoints"),
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}
