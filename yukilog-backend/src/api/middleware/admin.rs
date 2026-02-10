//! Admin 权限检查中间件
//!
//! 检查当前用户是否拥有管理员权限。
//!
//! # 工作流程
//! 1. 从请求 Extension 中提取 Claims（由 auth_middleware 注入）
//! 2. 检查 claims.role == "admin"
//! 3. 如果不是管理员，返回 403 Forbidden
//! 4. 如果是管理员，继续执行 Handler
//!
//! # 使用示例
//! ```rust,ignore
//! use axum::{Router, routing::get, middleware};
//!
//! let admin_routes = Router::new()
//!     .route("/users", get(list_users))
//!     .layer(middleware::from_fn(admin_middleware))
//!     .layer(middleware::from_fn(auth_middleware));  // 必须先通过认证
//! ```
//!
//! # 注意事项
//! - **必须在 auth_middleware 之后使用**
//! - 如果单独使用会导致 Extension 中没有 Claims

use axum::{
    body::Body,
    extract::{Extension, Request},
    middleware::Next,
    response::Response,
};

use crate::core::auth::jwt::Claims;
use crate::core::error::AppError;

/// Admin 权限检查中间件
///
/// # 功能
/// - 检查用户角色是否为 "admin"
/// - 拒绝非管理员访问
///
/// # 前置条件
/// - 必须先通过 auth_middleware（需要 Claims 已注入）
///
/// # 错误处理
/// - 403: 非管理员用户
pub async fn admin_middleware(
    Extension(claims): Extension<Claims>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // 检查用户角色
    if claims.role.as_deref() != Some("admin") {
        return Err(AppError::Forbidden("此操作需要管理员权限".to_string()));
    }

    // 角色验证通过，继续执行
    Ok(next.run(req).await)
}
