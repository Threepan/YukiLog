//! 认证授权服务
//!
//! 本模块提供用户认证相关功能，包括：
//! - 密码哈希和验证（Argon2id）
//! - JWT Token 生成和验证（Access + Refresh 双 Token 策略）
//! - 登录和 Token 刷新业务逻辑
//!
//! # 模块结构
//!
//! - [`password`] - 密码哈希工具
//! - [`jwt`] - JWT Token 工具
//! - [`dto`] - 数据传输对象
//! - [`service`] - 认证服务
//!
//! # 示例
//!
//! ```rust,ignore
//! use crate::core::auth::{AuthService, dto::LoginRequest};
//! use crate::core::auth::jwt::JwtUtils;
//!
//! // 创建服务
//! let jwt_utils = JwtUtils::new(&config.jwt);
//! let auth_service = AuthService::new(user_repo, jwt_utils);
//!
//! // 登录
//! let response = auth_service.login(LoginRequest {
//!     username: "alice".to_string(),
//!     password: "password".to_string(),
//! }).await?;
//!
//! // 刷新 Token
//! let new_tokens = auth_service.refresh_token(&response.refresh_token).await?;
//! ```

pub mod dto;
pub mod jwt;
pub mod password;
pub mod service;

// 重导出常用类型
pub use dto::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, UserInfo};
pub use jwt::{Claims, JwtUtils, TokenType};
pub use service::AuthService;
