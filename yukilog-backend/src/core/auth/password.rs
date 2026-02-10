//! 密码哈希工具
//!
//! 使用 Argon2id 算法进行密码哈希和验证
//!
//! # 安全性
//! - **Argon2id** 是目前推荐的密码哈希算法，结合了 Argon2i 和 Argon2d 的优点
//! - 具有内存硬性，有效抵抗 GPU/ASIC 暴力破解
//! - 参数可调，可根据服务器性能调整安全级别
//!
//! # 示例
//! ```rust,ignore
//! use crate::core::auth::password;
//!
//! // 注册时：哈希密码
//! let hash = password::hash_password("user_password")?;
//!
//! // 登录时：验证密码
//! let is_valid = password::verify_password("user_password", &hash)?;
//! ```

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::core::error::AppError;

/// 对密码进行哈希
///
/// # 参数
/// - `password`: 明文密码
///
/// # 返回
/// - 成功：Argon2id 哈希字符串（包含算法参数和盐值）
/// - 失败：返回 `AppError::Internal`
///
/// # 哈希格式
/// 返回的哈希字符串格式为 PHC 格式：
/// ```text
/// $argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>
/// ```
///
/// # 安全说明
/// - 每次调用都会生成新的随机盐值
/// - 相同密码每次哈希结果不同（这是正确的行为）
/// - 使用 Argon2id 变体，推荐用于密码哈希
pub fn hash_password(password: &str) -> Result<String, AppError> {
    // 生成随机盐值
    let salt = SaltString::generate(&mut OsRng);

    // 使用默认参数创建 Argon2 实例
    // 默认参数：Argon2id, 内存 19456 KiB, 迭代 2 次, 并行度 1
    let argon2 = Argon2::default();

    // 执行哈希
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("密码哈希失败: {}", e)))?;

    Ok(password_hash.to_string())
}

/// 验证密码是否匹配
///
/// # 参数
/// - `password`: 用户输入的明文密码
/// - `hash`: 数据库中存储的哈希值
///
/// # 返回
/// - `Ok(true)`: 密码匹配
/// - `Ok(false)`: 密码不匹配
/// - `Err(AppError)`: 哈希格式无效
///
/// # 安全说明
/// - 验证过程是常量时间的，防止计时攻击
/// - 盐值从哈希字符串中自动提取
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    // 解析存储的哈希字符串
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("密码哈希格式无效: {}", e)))?;

    // 验证密码
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);

    Ok(result.is_ok())
}
