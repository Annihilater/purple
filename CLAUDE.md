# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Building and Testing

```bash
# Build the project
cargo build

# Build for production
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test --test test_name

# Check code without building
cargo check
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run database migrations
psql -U username -d purple -f migrations/init.sql
```

### Running the Application

```bash
# Start development server
cargo run

# Start with custom environment
RUST_LOG=debug cargo run
```

### Environment Setup

Create `.env` file with:

```
# 数据库连接配置
DATABASE_URL=postgresql://purple:purple@localhost:5432/purple

# JWT 秘钥
JWT_SECRET=your-secret-key-here-please-change-in-production

# 服务器配置
SERVER_ADDR=127.0.0.1
SERVER_PORT=8080

# 日志配置
RUST_LOG=info
LOG_LEVEL=info
LOG_WITH_THREAD_IDS=true
LOG_WITH_LINE_NUMBER=true
LOG_WITH_FILE=true
LOG_WITH_TARGET=false
LOG_FILE_PATH=logs/app.log

```

## Architecture Overview

This is a Rust web API built with Actix-web following a layered architecture pattern:

### Core Architecture Layers

- **API Layer** (`src/api/`): HTTP request handlers and OpenAPI documentation
- **Service Layer** (`src/services/`): Business logic and JWT authentication
- **Repository Layer** (`src/repositories/`): Data access abstractions for PostgreSQL
- **Model Layer** (`src/models/`): Data structures and domain models

### Key Components

- **Application State** (`src/app_state.rs`): Dependency injection container holding repositories and services
- **Startup** (`src/startup.rs`): Application bootstrap process with configuration, logging, and server setup
- **Routes** (`src/routes.rs`): Centralized route configuration with OpenAPI/Swagger integration
- **Middleware System** (`src/middleware/`): Auth, CORS, and request logging middleware
- **Common Response System** (`src/common/`): Unified error handling and response formatting

### Response Architecture

All APIs return standardized responses with:

- `code`: Business error code (1000-6999 range)
- `status`: Status string representation
- `message`: Human-readable message (Chinese by default)
- `data`: Response payload (optional)
- `timestamp`: Unix timestamp

Error code ranges:

- 1000-1999: General errors
- 2000-2999: Authentication errors
- 3000-3999: User errors
- 4000-4999: Plan errors
- 5000-5999: Coupon errors
- 6000-6999: Order errors

### Database Architecture

- PostgreSQL with SQLx for async database operations
- Connection pooling (max 5 connections)
- Tables prefixed with `purple_` (user, plan, coupon, order)
- Migration scripts in `migrations/` directory

### Logging System

Dual-output logging:

- Console: Colored output for development
- File: Daily rotated logs in `logs/` directory

## Adding New Features

1. **Define Model**: Add data structures in `src/models/`
2. **Create Repository**: Implement data access in `src/repositories/`
3. **Add Service**: Implement business logic in `src/services/`
4. **Create API Handler**: Add HTTP handlers in `src/api/`
5. **Register Routes**: Update `src/routes.rs` with new endpoints
6. **Use Response System**: Leverage `common/response.rs` for consistent responses

## Testing

Tests are located in `tests/` directory:

- Repository tests use mock dependencies
- Use `tokio-test` for async testing
- `mockall` for mocking external dependencies

## API Documentation

- Swagger UI available at `/swagger-ui/`
- OpenAPI spec at `/api-docs/openapi.json`
- Health check at `/health`
- All business APIs under `/api/` prefix

## OpenAPI/Swagger Documentation Rules

### Critical Rules for OpenAPI Schema References

When working with OpenAPI annotations (`#[utoipa::path]`), **NEVER** use these problematic types in `body =` parameters:

#### ❌ FORBIDDEN Types

- **Generic types**: `ApiResponse<T>`, `PageResponse<T>`, etc.
  - Problem: `T` parameter creates "/components/schemas/T does not exist" errors
- **Type aliases**: `Response`, `Response<T>`
  - Problem: Aliases are not concrete schema types
- **Trait types**: `ResponseError`
  - Problem: Traits cannot be serialized as schemas

#### ✅ REQUIRED Types

Always use specific, concrete response types in OpenAPI annotations:

- `EmptyApiResponse` - For empty/error responses
- `UserApiResponse` - For single user responses  
- `UserPageApiResponse` - For paginated user responses
- `TokenApiResponse` - For authentication token responses
- `UserIdApiResponse` - For user ID responses
- `HealthApiResponse` - For health check responses

### OpenAPI Schema Registration

1. **All response types** used in `body =` must be registered in `src/api/openapi.rs`:

   ```rust
   #[derive(OpenApi)]
   #[openapi(
       components(schemas(
           // Register ALL response types here
           EmptyApiResponse,
           UserApiResponse,
           UserPageApiResponse,
           // ... etc
       ))
   )]
   ```

2. **Remove `#[derive(ToSchema)]`** from generic types:

   ```rust
   // ❌ Wrong - causes conflicts
   #[derive(Debug, Serialize, Deserialize, ToSchema)]
   pub struct ApiResponse<T> { ... }
   
   // ✅ Correct - no ToSchema for generics
   #[derive(Debug, Serialize, Deserialize)]
   pub struct ApiResponse<T> { ... }
   ```

### Creating New Response Types

When adding new APIs, create specific response types instead of using generics:

```rust
// In src/common/response.rs
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct YourNewApiResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<YourDataType>,
    pub timestamp: i64,
}
```

### Paginated Response Pattern

For paginated data, create specific data types:

```rust
// Create specific paginated data type
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct YourPageData {
    pub items: Vec<YourType>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
    pub has_next: bool,
    pub has_prev: bool,
}

// Use in specific response type
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct YourPageApiResponse {
    pub code: i32,
    pub status: String,
    pub message: String,
    pub data: Option<YourPageData>,
    pub timestamp: i64,
}
```

## Key Dependencies

- **actix-web**: Web framework
- **sqlx**: Database toolkit
- **utoipa**: OpenAPI documentation generation
- **jsonwebtoken**: JWT authentication
- **tracing**: Structured logging
- **validator**: Input validation
- **serde**: Serialization/deserialization
