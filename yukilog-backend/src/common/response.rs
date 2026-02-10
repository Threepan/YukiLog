//! 统一 API 响应格式
//!
//! 本模块提供标准的 HTTP API 响应结构，确保前后端约定一致。
//!
//! # 设计规范
//! - 所有 API 返回统一格式：`{ code, data, message }`
//! - `code = 0` 表示成功，非 0 表示业务错误
//! - `data` 成功时有值，错误时为 `null`
//! - `message` 用于前端提示用户

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// 统一 API 响应结构
///
/// # 泛型参数
/// - `T`: 响应数据类型，必须实现 `Serialize`
///
/// # 示例
/// ```rust,ignore
/// // 成功响应
/// let response = ApiResponse::success(user);
/// // 返回: { "code": 0, "data": { ... }, "message": "success" }
///
/// // 错误响应
/// let response = ApiResponse::<()>::error(40400, "User not found");
/// // 返回: { "code": 40400, "data": null, "message": "User not found" }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 业务状态码
    ///
    /// - `0`: 请求成功
    /// - `40xxx`: 客户端错误（参数错误、未认证等）
    /// - `50xxx`: 服务端错误（数据库错误、内部异常等）
    pub code: i32,

    /// 响应数据
    ///
    /// - 成功时包含实际业务数据
    /// - 错误时为 `null`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// 提示信息
    ///
    /// - 成功时通常为 "success"
    /// - 错误时为具体的错误描述，供前端显示
    pub message: String,
}

impl<T: Serialize> ApiResponse<T> {
    /// 创建成功响应（带数据）
    ///
    /// # 参数
    /// - `data`: 要返回的业务数据
    ///
    /// # 示例
    /// ```rust,ignore
    /// let user = User { id: 1, name: "Alice" };
    /// ApiResponse::success(user)
    /// ```
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            data: Some(data),
            message: "success".to_string(),
        }
    }

    /// 创建成功响应（自定义消息）
    ///
    /// # 参数
    /// - `data`: 要返回的业务数据
    /// - `message`: 自定义成功消息
    ///
    /// # 使用场景
    /// - 创建/更新/删除操作需要友好提示
    ///
    /// # 示例
    /// ```rust,ignore
    /// ApiResponse::success_with_message(user, "用户创建成功")
    /// ```
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            code: 0,
            data: Some(data),
            message: message.into(),
        }
    }
}

impl ApiResponse<()> {
    /// 创建成功响应（无数据）
    ///
    /// # 使用场景
    /// - 删除操作
    /// - 更新操作（不需要返回数据）
    ///
    /// # 示例
    /// ```rust,ignore
    /// ApiResponse::ok("删除成功")
    /// ```
    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            code: 0,
            data: None,
            message: message.into(),
        }
    }

    /// 创建错误响应
    ///
    /// # 参数
    /// - `code`: 业务错误码（见 `core/error.rs` 定义）
    /// - `message`: 错误描述信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// ApiResponse::<()>::error(40400, "用户不存在")
    /// ```
    pub fn error(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            data: None,
            message: message.into(),
        }
    }
}

// 实现 IntoResponse，让 ApiResponse 可以直接作为 Handler 返回值
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let response = ApiResponse::success("Hello");
        assert_eq!(response.code, 0);
        assert_eq!(response.data, Some("Hello"));
        assert_eq!(response.message, "success");
    }

    #[test]
    fn test_error_response() {
        let response = ApiResponse::<()>::error(40400, "Not found");
        assert_eq!(response.code, 40400);
        assert_eq!(response.data, None);
        assert_eq!(response.message, "Not found");
    }
}
