use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::{
    response::UserResponse,
    user::{CreateUserRequest, UpdateUserRequest},
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
    components(
        schemas(
            User,
            CreateUserRequest,
            UpdateUserRequest,
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
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
