//! 认证相关数据传输对象

use serde::{Deserialize, Serialize};

/// 登录请求
#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码（明文）
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    /// Access Token
    pub access_token: String,
    /// Refresh Token
    pub refresh_token: String,
    /// Token 类型，固定为 "Bearer"
    pub token_type: String,
    /// Access Token 有效期（秒）
    pub expires_in: i64,
    /// 当前登录用户信息
    pub user: UserInfo,
}

impl LoginResponse {
    pub fn new(
        access_token: String,
        refresh_token: String,
        expires_in: i64,
        user: UserInfo,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
            user,
        }
    }
}

/// 用户基本信息（不含敏感字段）
#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
}

impl UserInfo {
    /// 从 User Entity 创建
    pub fn from_entity(user: &crate::entities::users::Model) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            nickname: user.nickname.clone(),
            avatar_url: user.avatar_url.clone(),
            role: user.role.clone().unwrap_or_else(|| "user".to_string()),
        }
    }
}

/// Token 刷新请求
#[derive(Debug, Clone, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Token 刷新响应
#[derive(Debug, Clone, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

impl RefreshTokenResponse {
    pub fn new(
        access_token: String,
        refresh_token: String,
        expires_in: i64,
        user: UserInfo,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in,
            user,
        }
    }
}
