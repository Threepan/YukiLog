//! 业务逻辑层（Service 层）
//!
//! 本模块包含所有业务逻辑处理，是应用的核心。
//!
//! # 模块结构
//! - `error` - 业务错误定义
//! - `auth/` - 认证授权服务
//! - `users/` - 用户管理服务
//! - `posts/` - 文章管理服务
//! - `comments/` - 评论管理服务
//! - `categories/` - 分类管理服务
//! - `tags/` - 标签管理服务
//! - `links/` - 友链管理服务
//!
//! # 设计原则
//! - Service 层负责业务逻辑处理
//! - 调用 Repository 层访问数据
//! - 返回值使用 DTO（Data Transfer Object）
//! - 事务控制在 Service 层
//! - 错误统一使用 `AppError`

pub mod error;

// Service 模块（待实现）
pub mod auth;
pub mod categories;
pub mod comments;
pub mod links;
pub mod posts;
pub mod tags;
pub mod users;

// 重导出常用类型
pub use error::{AppError, Result};
