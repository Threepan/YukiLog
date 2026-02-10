use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;

/// 应用全局配置
#[derive(Clone, Debug)]
pub struct AppConfig {
    /// 数据库连接 URL
    pub database_url: String,
    
    /// Redis 连接 URL
    pub redis_url: String,
    
    /// 服务器监听地址
    pub server_host: String,
    
    /// 服务器监听端口
    pub server_port: u16,
    
    /// JWT 签名密钥
    pub jwt_secret: String,
    
    /// JWT 过期时间（秒）
    pub jwt_expires_in: i64,
    
    /// 管理员用户名
    pub admin_username: String,
    
    /// 管理员密码哈希（Argon2）
    pub admin_password_hash: String,
    
    /// CORS 允许的源（可选）
    pub cors_allowed_origins: Option<Vec<String>>,
}

impl AppConfig {
    /// 从环境变量加载配置
    /// 
    /// # 错误
    /// 
    /// 如果缺少必需的环境变量或格式不正确，返回错误
    pub fn from_env() -> Result<Self> {
        // 加载 .env 文件（如果存在）
        dotenv().ok();
        
        let config = Self {
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL 未设置")?,
            
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            
            server_host: env::var("SERVER_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .context("SERVER_PORT 必须是有效的端口号")?,
            
            jwt_secret: env::var("JWT_SECRET")
                .context("JWT_SECRET 未设置")?,
            
            jwt_expires_in: env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .context("JWT_EXPIRES_IN 必须是有效的秒数")?,
            
            admin_username: env::var("ADMIN_USERNAME")
                .context("ADMIN_USERNAME 未设置")?,
            
            admin_password_hash: env::var("ADMIN_PASSWORD_HASH")
                .context("ADMIN_PASSWORD_HASH 未设置")?,
            
            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .ok()
                .map(|s| s.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()),
        };
        
        // 验证配置
        config.validate()?;
        
        Ok(config)
    }
    
    /// 验证配置的有效性
    fn validate(&self) -> Result<()> {
        // 验证 JWT secret 长度（建议至少 32 字节）
        if self.jwt_secret.len() < 16 {
            anyhow::bail!("JWT_SECRET 长度至少应为 16 字符（建议 32+）");
        }
        
        // 验证 JWT 过期时间（应在合理范围内）
        if self.jwt_expires_in < 60 {
            anyhow::bail!("JWT_EXPIRES_IN 不应少于 60 秒");
        }
        if self.jwt_expires_in > 30 * 24 * 3600 {
            anyhow::bail!("JWT_EXPIRES_IN 不应超过 30 天");
        }
        
        // 验证管理员用户名
        if self.admin_username.is_empty() {
            anyhow::bail!("ADMIN_USERNAME 不能为空");
        }
        
        // 验证密码哈希格式（Argon2 格式）
        if !self.admin_password_hash.starts_with("$argon2") {
            anyhow::bail!("ADMIN_PASSWORD_HASH 必须是有效的 Argon2 哈希");
        }
        
        Ok(())
    }
    
    /// 获取服务器监听地址
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_server_addr() {
        let config = AppConfig {
            database_url: "".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            server_host: "0.0.0.0".to_string(),
            server_port: 8080,
            jwt_secret: "test-secret-key-at-least-16-chars".to_string(),
            jwt_expires_in: 3600,
            admin_username: "admin".to_string(),
            admin_password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test".to_string(),
            cors_allowed_origins: None,
        };
        
        assert_eq!(config.server_addr(), "0.0.0.0:8080");
    }
}
