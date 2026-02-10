use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::config::AppConfig;
use crate::handler::auth::{validate_token, Claims};

/// JWT 认证中间件
///
/// 用于保护管理后台接口，验证请求中的 JWT 令牌
///
/// # 工作流程
///
/// 1. 从 `Authorization` header 中提取 token
/// 2. 验证 token 格式（必须是 `Bearer <token>`）
/// 3. 使用 JWT_SECRET 验证 token 的有效性和签名
/// 4. 检查 token 是否过期
/// 5. 将解析出的 `Claims` 注入到 request extensions
/// 6. 放行请求到下一个中间件或 handler
///
/// # 错误处理
///
/// 如果认证失败，返回 401 Unauthorized：
/// - 缺少 Authorization header
/// - Token 格式错误（不是 Bearer 格式）
/// - Token 签名无效
/// - Token 已过期
///
/// # 安全性
///
/// - 所有认证失败都会记录日志（用于安全审计）
/// - 不暴露具体失败原因给客户端（防止信息泄露）
/// - 使用常量时间比较防止时序攻击（jsonwebtoken 库已实现）
///
/// # 使用方式
///
/// ```rust
/// use axum::middleware::from_fn_with_state;
///
/// // 应用到路由组
/// let admin_routes = Router::new()
///     .route("/posts", post(create_post))
///     .route("/comments", get(list_comments))
///     .layer(from_fn_with_state(config.clone(), jwt_auth));
/// ```
///
/// # Handler 中获取用户信息
///
/// ```rust
/// pub async fn create_post(
///     Extension(claims): Extension<Claims>,  // ← 自动注入
///     // ... 其他参数
/// ) -> Result<Json<ApiResponse<Post>>, ServiceError> {
///     let username = claims.sub;  // 获取用户名
///     // ... 业务逻辑
/// }
/// ```
pub async fn jwt_auth(
    State(config): State<AppConfig>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. 提取 token
    let token = match extract_bearer_token(&req) {
        Ok(t) => t,
        Err(e) => {
            tracing::warn!("JWT auth failed: missing or invalid Authorization header");
            return Err(e);
        }
    };

    // 2. 验证 token 并解析 Claims
    let claims = match validate_token(token, &config.jwt_secret) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("JWT validation failed: {:?}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    tracing::debug!("JWT auth successful for user: {}", claims.sub);

    // 3. 将 Claims 注入到 request extensions
    // 后续的 handler 可以通过 Extension<Claims> 访问
    req.extensions_mut().insert(claims);

    // 4. 放行请求到下一层
    Ok(next.run(req).await)
}

/// 从请求中提取 Bearer Token
///
/// # 参数
///
/// * `req` - HTTP 请求
///
/// # 返回
///
/// * `Ok(&str)` - 提取出的 JWT token 字符串
/// * `Err(StatusCode)` - 401 Unauthorized（缺少或格式错误）
///
/// # 示例
///
/// ```text
/// Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
/// ```
fn extract_bearer_token<B>(req: &Request<B>) -> Result<&str, StatusCode> {
    // 获取 Authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 检查 "Bearer " 前缀
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 确保 token 不为空
    if token.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderValue, Request};

    #[test]
    fn test_extract_bearer_token_success() {
        let mut req = Request::builder().body(()).unwrap();
        req.headers_mut().insert(
            AUTHORIZATION,
            HeaderValue::from_static("Bearer test-token-123"),
        );

        let token = extract_bearer_token(&req).unwrap();
        assert_eq!(token, "test-token-123");
    }

    #[test]
    fn test_extract_bearer_token_missing_header() {
        let req = Request::builder().body(()).unwrap();
        let result = extract_bearer_token(&req);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_extract_bearer_token_wrong_format() {
        let mut req = Request::builder().body(()).unwrap();
        req.headers_mut()
            .insert(AUTHORIZATION, HeaderValue::from_static("Basic username:password"));

        let result = extract_bearer_token(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_bearer_token_empty_token() {
        let mut req = Request::builder().body(()).unwrap();
        req.headers_mut()
            .insert(AUTHORIZATION, HeaderValue::from_static("Bearer "));

        let result = extract_bearer_token(&req);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_bearer_token_no_space_after_bearer() {
        let mut req = Request::builder().body(()).unwrap();
        req.headers_mut()
            .insert(AUTHORIZATION, HeaderValue::from_static("Bearertoken123"));

        let result = extract_bearer_token(&req);
        assert!(result.is_err());
    }
}
