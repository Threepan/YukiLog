use sea_orm::DatabaseConnection;

use crate::config::AppConfig;

/// 应用程序状态，包含数据库连接和 Redis 客户端
///
/// 在 Axum handler 中通过 `State<AppState>` 注入
#[derive(Clone)]
pub struct AppState {
    /// 应用配置
    pub config: AppConfig,
    /// SeaORM 数据库连接
    pub db: DatabaseConnection,
    /// Redis 客户端（用于限流和缓存）
    pub redis: redis::Client,
}
