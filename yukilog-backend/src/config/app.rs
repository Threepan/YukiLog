//! 应用配置
//!
//! 本模块定义应用的所有配置项，从环境变量加载

use super::database::DatabaseConfig;
use serde::Deserialize;

/// 应用总配置
///
/// # 使用方式
/// ```rust,ignore
/// use crate::config::AppConfig;
///
/// let config = AppConfig::from_env()?;
/// println!("Server listening on {}:{}", config.server.host, config.server.port);
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// 服务器配置
    pub server: ServerConfig,

    /// 数据库配置
    pub database: DatabaseConfig,

    /// JWT 配置
    pub jwt: JwtConfig,
}

impl AppConfig {
    /// 从环境变量加载配置
    ///
    /// # 环境变量命名规则
    /// 使用双下划线 `__` 分隔层级，例如：
    /// - `SERVER__HOST` 对应 `config.server.host`
    /// - `DATABASE__URL` 对应 `config.database.url`
    /// - `JWT__SECRET` 对应 `config.jwt.secret`
    ///
    /// # 错误
    /// - 必需的环境变量未设置时返回错误
    /// - 环境变量格式不正确时返回错误
    ///
    /// # 示例
    /// ```rust,ignore
    /// dotenvy::dotenv().ok(); // 加载 .env 文件
    /// let config = AppConfig::from_env()?;
    /// ```
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            // 从环境变量加载，使用 __ 作为层级分隔符
            .add_source(config::Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

/// 服务器配置
///
/// # 环境变量
/// - `SERVER__HOST`: 监听地址（可选，默认 127.0.0.1）
/// - `SERVER__PORT`: 监听端口（可选，默认 3000）
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// 服务器监听地址
    ///
    /// # 默认值
    /// `127.0.0.1` (仅本地访问)
    ///
    /// # 生产环境
    /// 通常设置为 `0.0.0.0` (允许外部访问)
    #[serde(default = "default_host")]
    pub host: String,

    /// 服务器监听端口
    ///
    /// # 默认值
    /// `3000`
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

/// JWT 配置
///
/// # 环境变量
/// - `JWT__SECRET`: JWT 签名密钥（必需）
/// - `JWT__ACCESS_TOKEN_EXPIRES`: Access Token 有效期秒数（可选，默认 900）
/// - `JWT__REFRESH_TOKEN_EXPIRES`: Refresh Token 有效期秒数（可选，默认 604800）
///
/// # 安全提示
/// - `JWT__SECRET` 必须使用强随机字符串
/// - 生产环境务必设置，不要使用默认值
/// - 定期更换密钥以提高安全性
#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    /// JWT 签名密钥
    ///
    /// # 要求
    /// - 长度至少 32 个字符
    /// - 使用随机字符串，不要使用可预测的值
    ///
    /// # 生成方式
    /// ```bash
    /// openssl rand -base64 32
    /// ```
    pub secret: String,

    /// Access Token 有效期（秒）
    ///
    /// # 默认值
    /// 900 秒（15 分钟）
    ///
    /// # 说明
    /// - Access Token 用于日常 API 请求
    /// - 有效期较短，提高安全性
    /// - 过期后需要用 Refresh Token 刷新
    #[serde(default = "default_access_token_expires")]
    pub access_token_expires: i64,

    /// Refresh Token 有效期（秒）
    ///
    /// # 默认值
    /// 604800 秒（7 天）
    ///
    /// # 说明
    /// - Refresh Token 用于刷新 Access Token
    /// - 有效期较长，减少用户重复登录
    /// - 过期后需要重新登录
    #[serde(default = "default_refresh_token_expires")]
    pub refresh_token_expires: i64,
}

fn default_access_token_expires() -> i64 {
    900 // 15 分钟
}

fn default_refresh_token_expires() -> i64 {
    604800 // 7 天
}
