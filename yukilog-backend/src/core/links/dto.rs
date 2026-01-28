use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

// URL简单校验: http/https开头
static URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://.*").unwrap());

// ===== Request DTOs =====

/// 申请友链请求
#[derive(Debug, Deserialize, Validate)]
pub struct ApplyLinkRequest {
    /// 站点标题 (1-50字符)
    #[validate(length(min = 1, max = 50, message = "站点标题长度必须在1-50个字符之间"))]
    pub link_title: String,

    /// 站点URL (必须以http/https开头)
    #[validate(
        length(min = 1, max = 255, message = "URL长度必须在1-255个字符之间"),
        regex(path = "*URL_REGEX", message = "URL必须以http://或https://开头")
    )]
    pub link_url: String,

    /// 站点头像 (可选, URL格式)
    #[validate(regex(path = "*URL_REGEX", message = "头像URL必须以http://或https://开头"))]
    pub link_avatar: Option<String>,

    /// 站点描述 (可选, 最大100字符)
    #[validate(length(max = 100, message = "站点描述最多100个字符"))]
    pub link_desc: Option<String>,
}

/// 更新友链信息请求
///
/// # 注意
/// - 当前版本: 仅Admin可更新
/// - 未来扩展: 可通过申请人email/token验证身份后允许自主更新
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLinkRequest {
    /// 新站点标题 (可选, 1-50字符)
    #[validate(length(min = 1, max = 50, message = "站点标题长度必须在1-50个字符之间"))]
    pub link_title: Option<String>,

    /// 新站点头像 (可选, URL格式)
    #[validate(regex(path = "*URL_REGEX", message = "头像URL必须以http://或https://开头"))]
    pub link_avatar: Option<String>,

    /// 新站点描述 (可选, 最大100字符)
    #[validate(length(max = 100, message = "站点描述最多100个字符"))]
    pub link_desc: Option<String>,
}

/// 更新单个友链状态请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStatusRequest {
    /// 新状态: active, pending, broken
    #[validate(custom(function = "validate_link_status"))]
    pub status: String,
}

/// 批量更新友链状态请求
#[derive(Debug, Deserialize, Validate)]
pub struct BatchUpdateStatusRequest {
    /// 友链ID列表
    #[validate(length(min = 1, message = "ID列表不能为空"))]
    pub ids: Vec<i64>,

    /// 新状态: active, pending, broken
    #[validate(custom(function = "validate_link_status"))]
    pub status: String,
}

// ===== Response DTOs =====

/// 友链响应
#[derive(Debug, Serialize)]
pub struct LinkResponse {
    /// 友链ID
    pub id: i64,

    /// 站点标题
    pub link_title: String,

    /// 站点URL
    pub link_url: String,

    /// 站点头像
    pub link_avatar: Option<String>,

    /// 站点描述
    pub link_desc: Option<String>,

    /// 友链状态: active, pending, broken
    pub link_status: String,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 分页友链列表响应
#[derive(Debug, Serialize)]
pub struct LinkListResponse {
    /// 友链列表
    pub links: Vec<LinkResponse>,

    /// 总数
    pub total: u64,
}

// ===== Validators =====

/// 校验友链状态
fn validate_link_status(status: &str) -> Result<(), validator::ValidationError> {
    match status {
        "active" | "pending" | "broken" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_status")),
    }
}
