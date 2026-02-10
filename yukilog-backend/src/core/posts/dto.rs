use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

// Slug正则: 仅小写字母、数字、连字符
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9-]+$").unwrap());

// URL简单校验
static URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://.*").unwrap());

// ===== Request DTOs =====

/// 创建文章请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    /// 文章标题 (1-255字符)
    #[validate(length(min = 1, max = 255, message = "标题长度必须在1-255个字符之间"))]
    pub title: String,

    /// 副标题 (可选, 1-255字符)
    #[validate(length(min = 1, max = 255, message = "副标题长度必须在1-255个字符之间"))]
    pub sub_title: Option<String>,

    /// URL友好的标识符 (1-255字符, 仅小写字母数字连字符)
    #[validate(
        length(min = 1, max = 255, message = "Slug长度必须在1-255个字符之间"),
        regex(path = "*SLUG_REGEX", message = "Slug只能包含小写字母、数字和连字符")
    )]
    pub slug: String,

    /// 文章摘要 (可选, 最大1000字符)
    #[validate(length(max = 1000, message = "摘要最多1000个字符"))]
    pub summary: Option<String>,

    /// 文章正文 (Markdown格式)
    #[validate(length(min = 1, message = "文章内容不能为空"))]
    pub content: String,

    /// 封面图URL (可选)
    #[validate(regex(path = "*URL_REGEX", message = "封面图URL必须以http://或https://开头"))]
    pub cover_image: Option<String>,

    /// 分类ID (可选)
    pub category_id: Option<i64>,

    /// 标签ID列表 (可选)
    pub tag_ids: Option<Vec<i64>>,

    /// 文章状态 (可选, 默认draft)
    #[validate(custom(function = "validate_post_status"))]
    pub status: Option<String>,
}

/// 更新文章请求
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePostRequest {
    /// 新标题 (可选, 1-255字符)
    #[validate(length(min = 1, max = 255, message = "标题长度必须在1-255个字符之间"))]
    pub title: Option<String>,

    /// 新副标题 (可选, 1-255字符)
    #[validate(length(min = 1, max = 255, message = "副标题长度必须在1-255个字符之间"))]
    pub sub_title: Option<String>,

    /// 新Slug (可选, 1-255字符, 仅小写字母数字连字符)
    #[validate(
        length(min = 1, max = 255, message = "Slug长度必须在1-255个字符之间"),
        regex(path = "*SLUG_REGEX", message = "Slug只能包含小写字母、数字和连字符")
    )]
    pub slug: Option<String>,

    /// 新摘要 (可选, 最大1000字符)
    #[validate(length(max = 1000, message = "摘要最多1000个字符"))]
    pub summary: Option<String>,

    /// 新正文 (可选)
    #[validate(length(min = 1, message = "文章内容不能为空"))]
    pub content: Option<String>,

    /// 新封面图 (可选)
    #[validate(regex(path = "*URL_REGEX", message = "封面图URL必须以http://或https://开头"))]
    pub cover_image: Option<String>,

    /// 新分类ID (可选)
    pub category_id: Option<i64>,

    /// 新标签ID列表 (可选)
    pub tag_ids: Option<Vec<i64>>,

    /// 新状态 (可选)
    #[validate(custom(function = "validate_post_status"))]
    pub status: Option<String>,
}

/// 发布文章请求
#[derive(Debug, Deserialize, Validate)]
pub struct PublishPostRequest {
    /// 发布时间 (可选, 默认当前时间)
    /// ISO 8601格式: 2024-01-28T10:00:00Z
    pub published_at: Option<DateTime<Utc>>,
}

/// 同步标签请求
#[derive(Debug, Deserialize, Validate)]
pub struct SyncTagsRequest {
    /// 标签ID列表
    pub tag_ids: Vec<i64>,
}

// ===== Response DTOs =====

/// 文章详情响应 (含完整内容)
#[derive(Debug, Serialize)]
pub struct PostDetailResponse {
    /// 文章ID
    pub id: i64,

    /// 标题
    pub title: String,

    /// 副标题
    pub sub_title: Option<String>,

    /// URL标识符
    pub slug: String,

    /// 摘要
    pub summary: Option<String>,

    /// 正文 (Markdown)
    pub content: String,

    /// 封面图
    pub cover_image: Option<String>,

    /// 状态: draft, published, archived
    pub status: String,

    /// 分类信息
    pub category: Option<CategoryInfo>,

    /// 标签列表
    pub tags: Vec<TagInfo>,

    /// 作者信息
    pub author: Option<AuthorInfo>,

    /// 浏览量
    pub view_count: i64,

    /// 是否置顶
    pub is_pinned: bool,

    /// 发布时间
    pub published_at: Option<DateTime<Utc>>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 文章列表项响应 (不含正文)
#[derive(Debug, Serialize)]
pub struct PostListItemResponse {
    /// 文章ID
    pub id: i64,

    /// 标题
    pub title: String,

    /// 副标题
    pub sub_title: Option<String>,

    /// URL标识符
    pub slug: String,

    /// 摘要
    pub summary: Option<String>,

    /// 封面图
    pub cover_image: Option<String>,

    /// 状态
    pub status: String,

    /// 分类信息
    pub category: Option<CategoryInfo>,

    /// 标签列表
    pub tags: Vec<TagInfo>,

    /// 作者信息
    pub author: Option<AuthorInfo>,

    /// 浏览量
    pub view_count: i64,

    /// 是否置顶
    pub is_pinned: bool,

    /// 发布时间
    pub published_at: Option<DateTime<Utc>>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 归档列表项 (简化版)
#[derive(Debug, Serialize)]
pub struct PostArchiveItem {
    /// 文章ID
    pub id: i64,

    /// 标题
    pub title: String,

    /// URL标识符
    pub slug: String,

    /// 发布时间
    pub published_at: DateTime<Utc>,
}

/// 归档分组 (按年月)
#[derive(Debug, Serialize)]
pub struct ArchiveGroup {
    /// 年月 (格式: 2024-01)
    pub year_month: String,

    /// 该月文章列表
    pub posts: Vec<PostArchiveItem>,
}

/// 分页文章列表响应
#[derive(Debug, Serialize)]
pub struct PostListResponse {
    /// 文章列表
    pub posts: Vec<PostListItemResponse>,

    /// 总数
    pub total: u64,
}

// ===== 嵌套结构 =====

/// 分类信息 (嵌套在文章响应中)
#[derive(Debug, Serialize, Clone)]
pub struct CategoryInfo {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

/// 标签信息 (嵌套在文章响应中)
#[derive(Debug, Serialize)]
pub struct TagInfo {
    pub id: i64,
    pub name: String,
    pub slug: String,
}

/// 作者信息 (嵌套在文章响应中)
#[derive(Debug, Serialize, Clone)]
pub struct AuthorInfo {
    pub id: i64,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
}

// ===== Validators =====

/// 校验文章状态
fn validate_post_status(status: &str) -> Result<(), validator::ValidationError> {
    match status {
        "draft" | "published" | "archived" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_status")),
    }
}
