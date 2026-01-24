//! 数据库配置
//!
//! 本模块定义数据库连接相关的配置项

use serde::Deserialize;

/// 数据库配置
///
/// # 环境变量
/// - `DATABASE__URL`: 数据库连接字符串（必需）
/// - `DATABASE__MAX_CONNECTIONS`: 最大连接数（可选，默认 10）
/// - `DATABASE__MIN_CONNECTIONS`: 最小连接数（可选，默认 2）
/// - `DATABASE__CONNECT_TIMEOUT`: 连接超时秒数（可选，默认 30）
///
/// # 示例 .env
/// ```env
/// DATABASE__URL=postgres://user:password@localhost:5432/yukilog
/// DATABASE__MAX_CONNECTIONS=20
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库连接 URL
    ///
    /// # 格式
    /// `postgres://username:password@host:port/database`
    ///
    /// # 示例
    /// `postgres://yukilog:secret@localhost:5432/yukilog_db`
    pub url: String,

    /// 连接池最大连接数
    ///
    /// # 说明
    /// - 默认值: 10
    /// - 建议范围: 5-50
    /// - 过大会占用过多数据库资源
    /// - 过小会导致高并发时连接不足
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    /// 连接池最小连接数
    ///
    /// # 说明
    /// - 默认值: 2
    /// - 保持一定数量的空闲连接，提升响应速度
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,

    /// 连接超时时间（秒）
    ///
    /// # 说明
    /// - 默认值: 30 秒
    /// - 超过此时间未能建立连接将返回错误
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout: u64,

    /// 是否启用 SQL 日志
    ///
    /// # 说明
    /// - 默认值: false
    /// - 开发环境建议设为 true，便于调试
    /// - 生产环境建议设为 false，避免日志过多
    #[serde(default = "default_enable_logging")]
    pub enable_logging: bool,
}

fn default_max_connections() -> u32 {
    10
}

fn default_min_connections() -> u32 {
    2
}

fn default_connect_timeout() -> u64 {
    30
}

fn default_enable_logging() -> bool {
    false
}
