/// 统一响应格式
pub mod response;

/// 错误处理和转换
pub mod error;

/// JWT 认证和登录
pub mod auth;

/// 中间件（JWT 认证等）
pub mod middleware;

/// 工具函数（IP提取、限流、Gravatar）
pub mod utils;

/// 应用程序状态（数据库连接和 Redis 客户端）
pub mod state;

/// 公开接口（前台）
pub mod public;
