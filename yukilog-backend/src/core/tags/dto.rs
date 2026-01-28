use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Slug正则: 仅小写字母、数字、连字符
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9-]+$").unwrap());

// ===== Request DTOs =====

/// 创建标签请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTagRequest {
    /// 标签名 (1-30字符)
    #[validate(length(min = 1, max = 30, message = "标签名长度必须在1-30个字符之间"))]
    pub name: String,

    /// URL友好的标识符 (1-30字符, 仅小写字母数字连字符)
    #[validate(
        length(min = 1, max = 30, message = "Slug长度必须在1-30个字符之间"),
        regex(path = "*SLUG_REGEX", message = "Slug只能包含小写字母、数字和连字符")
    )]
    pub slug: String,
}

/// 更新标签请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTagRequest {
    /// 新标签名 (可选, 1-30字符)
    #[validate(length(min = 1, max = 30, message = "标签名长度必须在1-30个字符之间"))]
    pub name: Option<String>,

    /// 新Slug (可选, 1-30字符, 仅小写字母数字连字符)
    #[validate(
        length(min = 1, max = 30, message = "Slug长度必须在1-30个字符之间"),
        regex(path = "*SLUG_REGEX", message = "Slug只能包含小写字母、数字和连字符")
    )]
    pub slug: Option<String>,
}

// ===== Response DTOs =====

/// 标签响应 (基础信息)
#[derive(Debug, Serialize)]
pub struct TagResponse {
    /// 标签ID
    pub id: i32,

    /// 标签名
    pub name: String,

    /// URL标识符
    pub slug: String,

    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 标签响应 (含文章数)
#[derive(Debug, Serialize)]
pub struct TagWithCountResponse {
    /// 标签ID
    pub id: i32,

    /// 标签名
    pub name: String,

    /// URL标识符
    pub slug: String,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 使用该标签的文章数量
    pub post_count: u64,
}

/// 批量查找或创建标签请求
#[derive(Debug, Deserialize, Validate)]
pub struct FindOrCreateBatchRequest {
    /// 标签列表 (每项包含name和slug)
    #[validate(length(min = 1, message = "标签列表不能为空"))]
    pub tags: Vec<CreateTagRequest>,
}

/// 批量查找或创建标签响应
#[derive(Debug, Serialize)]
pub struct FindOrCreateBatchResponse {
    /// 成功创建/查找的标签
    pub tags: Vec<TagResponse>,

    /// 失败的标签 (名称 + 错误原因)
    pub errors: Vec<TagCreationError>,
}

/// 单个标签创建失败信息
#[derive(Debug, Serialize)]
pub struct TagCreationError {
    /// 标签名
    pub name: String,

    /// Slug
    pub slug: String,

    /// 错误原因
    pub reason: String,
}
