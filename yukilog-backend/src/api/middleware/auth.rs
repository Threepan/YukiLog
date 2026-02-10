//! JWT 认证中间件
//!
//! 验证 HTTP 请求中的 JWT Access Token，并将用户信息注入到请求扩展中。
//!
//! # 工作流程
//! 1. 从 Authorization Header 提取 Bearer Token
//! 2. 调用 JwtUtils 验证 Token 签名和过期时间
//! 3. 提取 Claims（user_id, username, role）
//! 4. 将 Claims 注入到请求 Extension 中
//! 5. 如果验证失败，返回 401 Unauthorized
//!
//! # 使用示例
//! ```rust,ignore
//! use axum::{Router, routing::get, middleware};
//!
//! let protected_routes = Router::new()
//!     .route("/profile", get(get_profile))
//!     .layer(middleware::from_fn(auth_middleware));
//! ```

use axum::{body::Body, extract::Request, http::HeaderMap, middleware::Next, response::Response};

use crate::config::app::AppConfig;
use crate::core::auth::jwt::JwtUtils;
use crate::core::error::AppError;

/// JWT 认证中间件
///
/// # 功能
/// - 验证 JWT Access Token
/// - 提取用户信息（Claims）并注入到请求中
///
/// # 错误处理
/// - 401: Token 缺失或无效
pub async fn auth_middleware(
    headers: HeaderMap,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // 1. 从 Header 提取 Token
    let token = extract_bearer_token(&headers)
        .map_err(|e| AppError::Unauthorized(format!("认证失败: {}", e)))?;

    // 2. 验证 Token 并提取 Claims
    let config = AppConfig::from_env()
        .map_err(|e| AppError::Unauthorized(format!("配置加载失败: {}", e)))?;
    let jwt_utils = JwtUtils::new(&config.jwt);
    let claims = jwt_utils
        .verify_access_token(&token)
        .map_err(|e| AppError::Unauthorized(format!("Token 验证失败: {}", e)))?;

    // 3. 将 Claims 注入到请求扩展中
    req.extensions_mut().insert(claims);

    // 4. 继续执行后续中间件或 Handler
    Ok(next.run(req).await)
}

/// 从 HTTP Header 提取 Bearer Token
///
/// # 格式
/// ```text
/// Authorization: Bearer <token>
/// ```
///
/// # 错误
/// - Token 缺失
/// - 格式错误（不是 Bearer 格式）
fn extract_bearer_token(headers: &HeaderMap) -> Result<String, String> {
    // 获取 Authorization Header
    let auth_header = headers
        .get("authorization")
        .or_else(|| headers.get("Authorization"))
        .ok_or("缺少 Authorization Header")?;

    // 转换为字符串
    let auth_str = auth_header
        .to_str()
        .map_err(|_| "Authorization Header 格式无效")?;

    // 检查是否以 "Bearer " 开头
    if !auth_str.starts_with("Bearer ") {
        return Err("Authorization Header 必须以 'Bearer ' 开头".to_string());
    }

    // 提取 Token（去掉 "Bearer " 前缀）
    let token = auth_str[7..].trim().to_string();

    if token.is_empty() {
        return Err("Token 不能为空".to_string());
    }

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_extract_bearer_token_success() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Bearer test_token_12345"),
        );

        let result = extract_bearer_token(&headers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_token_12345");
    }

    #[test]
    fn test_extract_bearer_token_missing() {
        let headers = HeaderMap::new();
        let result = extract_bearer_token(&headers);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "缺少 Authorization Header");
    }

    #[test]
    fn test_extract_bearer_token_invalid_format() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_static("Basic dXNlcjpwYXNz"),
        );

        let result = extract_bearer_token(&headers);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Bearer"));
    }

    #[test]
    fn test_extract_bearer_token_empty() {
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_static("Bearer "));

        let result = extract_bearer_token(&headers);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Token 不能为空");
    }
}
