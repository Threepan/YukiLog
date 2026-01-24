//! 配置管理模块
//!
//! 提供应用配置的加载和管理

pub mod app;
pub mod database;

// 重导出常用类型
pub use app::{AppConfig, JwtConfig, ServerConfig};
pub use database::DatabaseConfig;
