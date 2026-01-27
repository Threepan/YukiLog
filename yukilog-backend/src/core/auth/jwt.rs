//! JWT Token 工具
//!
//! 实现 Access Token + Refresh Token 双重 Token 策略
//!
//! # Token 策略
//!
//! | Token 类型 | 用途 | 有效期 | 包含信息 |
//! |-----------|------|--------|---------|
//! | Access Token | API 请求认证 | 15 分钟 | 用户完整信息 |
//! | Refresh Token | 刷新 Access Token | 7 天 | 仅 user_id |
//!
//! # 安全设计
//!
//! - Access Token 有效期短，即使泄露影响有限
//! - Refresh Token 有效期长，但不携带敏感信息
//! - 两种 Token 使用相同密钥但不同的 `token_type` 字段区分

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::app::JwtConfig;
use crate::core::error::AppError;

/// Token 类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    /// Access Token - 用于 API 请求认证
    Access,
    /// Refresh Token - 用于刷新 Access Token
    Refresh,
}

/// JWT Claims 结构
///
/// # Access Token vs Refresh Token
/// - **Access Token**: 包含 username 和 role，用于权限验证
/// - **Refresh Token**: 仅包含 sub (user_id)，信息最小化
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject - 用户 ID
    pub sub: i64,

    /// 用户名（仅 Access Token 包含）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// 用户角色（仅 Access Token 包含）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Token 类型
    pub token_type: TokenType,

    /// 过期时间 (Unix timestamp)
    pub exp: i64,

    /// 签发时间 (Unix timestamp)
    pub iat: i64,
}

impl Claims {
    /// 获取用户 ID
    pub fn user_id(&self) -> i64 {
        self.sub
    }

    /// 检查是否为 Access Token
    pub fn is_access_token(&self) -> bool {
        self.token_type == TokenType::Access
    }

    /// 检查是否为 Refresh Token
    pub fn is_refresh_token(&self) -> bool {
        self.token_type == TokenType::Refresh
    }

    /// 检查是否为管理员
    pub fn is_admin(&self) -> bool {
        self.role.as_deref() == Some("admin")
    }
}

/// JWT 工具结构体
#[derive(Clone)]
pub struct JwtUtils {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_expires: i64,
    refresh_expires: i64,
}

impl JwtUtils {
    /// 创建 JwtUtils 实例
    pub fn new(config: &JwtConfig) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(config.secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(config.secret.as_bytes()),
            access_expires: config.access_token_expires,
            refresh_expires: config.refresh_token_expires,
        }
    }

    /// 生成 Access Token + Refresh Token 对
    pub fn generate_token_pair(
        &self,
        user_id: i64,
        username: &str,
        role: &str,
    ) -> Result<(String, String), AppError> {
        let access_token = self.generate_access_token(user_id, username, role)?;
        let refresh_token = self.generate_refresh_token(user_id)?;
        Ok((access_token, refresh_token))
    }

    /// 生成 Access Token
    pub fn generate_access_token(
        &self,
        user_id: i64,
        username: &str,
        role: &str,
    ) -> Result<String, AppError> {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(self.access_expires);

        let claims = Claims {
            sub: user_id,
            username: Some(username.to_string()),
            role: Some(role.to_string()),
            token_type: TokenType::Access,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("生成 Access Token 失败: {}", e)))
    }

    /// 生成 Refresh Token（仅包含 user_id）
    pub fn generate_refresh_token(&self, user_id: i64) -> Result<String, AppError> {
        let now = Utc::now();
        let expires_at = now + Duration::seconds(self.refresh_expires);

        let claims = Claims {
            sub: user_id,
            username: None,
            role: None,
            token_type: TokenType::Refresh,
            exp: expires_at.timestamp(),
            iat: now.timestamp(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(format!("生成 Refresh Token 失败: {}", e)))
    }

    /// 验证 Access Token
    pub fn verify_access_token(&self, token: &str) -> Result<Claims, AppError> {
        let claims = self.decode_token(token)?;
        if !claims.is_access_token() {
            return Err(AppError::Unauthorized("Token 类型错误".to_string()));
        }
        Ok(claims)
    }

    /// 验证 Refresh Token
    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims, AppError> {
        let claims = self.decode_token(token)?;
        if !claims.is_refresh_token() {
            return Err(AppError::Unauthorized("Token 类型错误".to_string()));
        }
        Ok(claims)
    }

    /// 解码 Token（内部方法）
    fn decode_token(&self, token: &str) -> Result<Claims, AppError> {
        let validation = Validation::default();

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::Unauthorized("Token 已过期".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    AppError::Unauthorized("Token 格式无效".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                    AppError::Unauthorized("Token 签名无效".to_string())
                }
                _ => AppError::Unauthorized(format!("Token 验证失败: {}", e)),
            })
    }

    /// 获取 Access Token 有效期（秒）
    pub fn access_expires(&self) -> i64 {
        self.access_expires
    }

    /// 获取 Refresh Token 有效期（秒）
    pub fn refresh_expires(&self) -> i64 {
        self.refresh_expires
    }
}
