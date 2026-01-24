//! 业务错误定义
//!
//! 本模块定义应用中所有可能出现的业务错误类型。
//!
//! # 设计原则
//! - 使用 `thiserror` 自动实现 `std::error::Error` trait
//! - 实现 `IntoResponse` 自动转换为 HTTP JSON 响应
//! - 错误信息对用户友好，敏感信息不暴露
//! - 错误码规范化，便于前端统一处理

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// 应用业务错误类型
///
/// # 错误码规范
/// - `0`: 成功（不会出现在错误响应中）
/// - `40xxx`: 客户端错误（400 系列）
///   - `40000`: 请求参数错误
///   - `40001`: 数据校验失败
///   - `40002`: 业务逻辑错误
///   - `40100`: 未认证（需要登录）
///   - `40300`: 无权限（已登录但角色不足）
///   - `40400`: 资源不存在
/// - `50xxx`: 服务端错误（500 系列）
///   - `50000`: 数据库错误
///   - `50001`: 内部错误
///
/// # 使用示例
/// ```rust
/// // Service 层抛出错误
/// if user.is_none() {
///     return Err(AppError::NotFound("用户不存在".into()));
/// }
///
/// // Handler 层自动转换为 HTTP 响应
/// async fn get_user(id: i64) -> Result<Json<User>, AppError> {
///     let user = service.find_user(id).await?;
///     Ok(Json(user))
/// }
/// ```
#[derive(Debug, Error)]
pub enum AppError {
    // ==================== 客户端错误 (40xxx) ====================
    /// 未认证错误 (401)
    ///
    /// # 触发场景
    /// - 访问需要登录的接口但未提供 Token
    /// - Token 已过期
    /// - Token 格式错误或签名验证失败
    ///
    /// # 前端处理
    /// - 跳转到登录页
    /// - 提示用户重新登录
    ///
    /// # 示例
    /// ```rust
    /// return Err(AppError::Unauthorized("Token 已过期，请重新登录".into()));
    /// ```
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// 无权限错误 (403)
    ///
    /// # 触发场景
    /// - 已登录但角色不足（如普通用户访问管理员接口）
    /// - 尝试访问不属于自己的资源
    ///
    /// # 前端处理
    /// - 提示 "您没有权限执行此操作"
    /// - 不跳转登录页（因为已登录）
    ///
    /// # 示例
    /// ```rust
    /// if user.role != "admin" {
    ///     return Err(AppError::Forbidden("需要管理员权限".into()));
    /// }
    /// ```
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 资源不存在错误 (404)
    ///
    /// # 触发场景
    /// - 根据 ID 查询数据库，但记录不存在
    /// - 访问不存在的文章、用户、评论等
    ///
    /// # 前端处理
    /// - 显示 404 页面
    /// - 提示资源已被删除或不存在
    ///
    /// # 示例
    /// ```rust
    /// let post = Posts::find_by_id(id).one(&db).await?;
    /// post.ok_or_else(|| AppError::NotFound(format!("文章 {} 不存在", id)))?
    /// ```
    #[error("Not found: {0}")]
    NotFound(String),

    /// 请求参数错误 (400)
    ///
    /// # 触发场景
    /// - JSON 格式错误
    /// - 缺少必需参数
    /// - 参数类型不匹配
    ///
    /// # 前端处理
    /// - 提示用户检查输入
    /// - 高亮错误字段
    ///
    /// # 示例
    /// ```rust
    /// if title.is_empty() {
    ///     return Err(AppError::BadRequest("标题不能为空".into()));
    /// }
    /// ```
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// 数据校验失败 (400)
    ///
    /// # 触发场景
    /// - 邮箱格式不正确
    /// - 密码长度不符合要求
    /// - 用户名包含非法字符
    ///
    /// # 与 BadRequest 的区别
    /// - BadRequest: 参数结构问题（缺少、类型错误）
    /// - Validation: 参数值不符合业务规则
    ///
    /// # 示例
    /// ```rust
    /// if !email.contains('@') {
    ///     return Err(AppError::Validation("邮箱格式不正确".into()));
    /// }
    /// ```
    #[error("Validation error: {0}")]
    Validation(String),

    /// 业务逻辑错误 (400)
    ///
    /// # 触发场景
    /// - 用户名已存在（注册时）
    /// - 文章已发布无法删除
    /// - 评论已审核无法修改
    ///
    /// # 说明
    /// 这类错误不是参数格式问题，而是业务规则限制
    ///
    /// # 示例
    /// ```rust
    /// if username_exists {
    ///     return Err(AppError::Business("用户名已被占用".into()));
    /// }
    /// ```
    #[error("Business error: {0}")]
    Business(String),

    // ==================== 服务端错误 (50xxx) ====================
    /// 数据库错误 (500)
    ///
    /// # 触发场景
    /// - 数据库连接失败
    /// - SQL 执行错误
    /// - 外键约束冲突
    /// - 数据库服务不可用
    ///
    /// # 安全考虑
    /// - 不向前端暴露具体的数据库错误信息（可能泄露表结构）
    /// - 日志中记录详细错误，便于排查
    ///
    /// # 自动转换
    /// 使用 `#[from]` 自动将 `sea_orm::DbErr` 转换为 `AppError::Database`
    ///
    /// # 示例
    /// ```rust
    /// // 自动转换
    /// let user = Users::find_by_id(id).one(&db).await?; // DbErr -> AppError
    ///
    /// // 手动转换
    /// Users::find().all(&db).await.map_err(|e| {
    ///     tracing::error!("Database error: {:?}", e);
    ///     AppError::Database(e)
    /// })?;
    /// ```
    #[error("Database error")]
    Database(#[from] sea_orm::DbErr),

    /// 内部错误 (500)
    ///
    /// # 触发场景
    /// - 未预期的异常情况
    /// - 第三方服务调用失败
    /// - 文件系统错误
    /// - 内存不足等系统错误
    ///
    /// # 处理建议
    /// - 记录详细的错误日志
    /// - 向前端返回通用的错误信息
    /// - 考虑是否需要告警通知
    ///
    /// # 示例
    /// ```rust
    /// let file = std::fs::read("config.toml")
    ///     .map_err(|e| AppError::Internal(format!("配置文件读取失败: {}", e)))?;
    /// ```
    #[error("Internal error: {0}")]
    Internal(String),
}

impl AppError {
    /// 获取 HTTP 状态码和业务错误码
    ///
    /// # 返回
    /// `(StatusCode, i32)` - HTTP 状态码和业务错误码的元组
    ///
    /// # 映射规则
    /// | AppError         | HTTP Status | Error Code |
    /// |------------------|-------------|------------|
    /// | Unauthorized     | 401         | 40100      |
    /// | Forbidden        | 403         | 40300      |
    /// | NotFound         | 404         | 40400      |
    /// | BadRequest       | 400         | 40000      |
    /// | Validation       | 400         | 40001      |
    /// | Business         | 400         | 40002      |
    /// | Database         | 500         | 50000      |
    /// | Internal         | 500         | 50001      |
    fn status_and_code(&self) -> (StatusCode, i32) {
        match self {
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, 40100),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, 40300),
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, 40400),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, 40000),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, 40001),
            AppError::Business(_) => (StatusCode::BAD_REQUEST, 40002),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50000),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
        }
    }
}

// 实现 IntoResponse，让 AppError 可以直接作为 Handler 返回值
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = self.status_and_code();

        // 获取错误信息
        // 对于数据库错误，不暴露详细信息
        let message = match &self {
            AppError::Database(_) => {
                // 记录详细错误到日志
                tracing::error!("Database error: {:?}", self);
                "数据库错误，请稍后重试".to_string()
            }
            AppError::Internal(_) => {
                // 记录详细错误到日志
                tracing::error!("Internal error: {:?}", self);
                "服务器内部错误，请稍后重试".to_string()
            }
            _ => self.to_string(),
        };

        // 构造符合 ApiResponse 格式的 JSON
        let body = json!({
            "code": code,
            "data": null,
            "message": message,
        });

        (status, Json(body)).into_response()
    }
}

/// Result 类型别名，简化函数签名
///
/// # 使用
/// ```rust
/// // 原始写法
/// async fn get_user(id: i64) -> std::result::Result<User, AppError> { }
///
/// // 使用别名
/// use crate::core::error::Result;
/// async fn get_user(id: i64) -> Result<User> { }
/// ```
pub type Result<T> = std::result::Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(
            AppError::Unauthorized("test".into()).status_and_code(),
            (StatusCode::UNAUTHORIZED, 40100)
        );
        assert_eq!(
            AppError::NotFound("test".into()).status_and_code(),
            (StatusCode::NOT_FOUND, 40400)
        );
    }
}
