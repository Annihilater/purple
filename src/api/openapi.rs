use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{
    response::UserResponse,
    user::{CreateUserRequest, GetUsersQuery, UpdateUserRequest, UpdateUserStatusRequest},
};
use crate::models::{
    auth::{Claims, LoginRequest, RegisterRequest, TokenResponse},
    coupon::{
        Coupon, CouponListResponse, CouponResponse, CreateCouponRequest, UpdateCouponRequest,
        ValidateCouponResponse,
    },
    plan::{CreatePlanRequest, Plan, PlanListResponse, PlanResponse, UpdatePlanRequest},
    user::{User, UserResponse as UserModel},
};

#[derive(OpenApi)]
#[openapi(
    paths(
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
        crate::api::coupon::create_coupon,
        crate::api::coupon::list_coupons,
        crate::api::coupon::get_coupon,
        crate::api::coupon::update_coupon,
        crate::api::coupon::delete_coupon,
        crate::api::coupon::verify_coupon,
    ),
    components(
        schemas(
            User,
            CreateUserRequest,
            UpdateUserRequest,
            UpdateUserStatusRequest,
            GetUsersQuery,
            UserResponse,
            UserModel,
            Plan,
            CreatePlanRequest,
            UpdatePlanRequest,
            PlanResponse,
            PlanListResponse,
            Coupon,
            CreateCouponRequest,
            UpdateCouponRequest,
            CouponResponse,
            CouponListResponse,
            ValidateCouponResponse,
            RegisterRequest,
            LoginRequest,
            TokenResponse,
            Claims,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "plans", description = "Plan management endpoints"),
        (name = "coupons", description = "Coupon management endpoints"),
        (name = "auth", description = "Authentication endpoints"),
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}
