use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

// 邮箱简单校验
static EMAIL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());

// URL简单校验
static URL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^https?://.*").unwrap());

// ===== Request DTOs =====

/// 发表评论请求
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    /// 文章ID
    pub post_id: i64,

    /// 评论内容 (1-1000字符)
    #[validate(length(min = 1, max = 1000, message = "评论内容长度必须在1-1000个字符之间"))]
    pub content: String,

    /// 父评论ID (可选, 用于回复评论)
    pub parent_id: Option<i64>,

    /// 游客昵称 (游客必填, 1-50字符)
    #[validate(length(min = 1, max = 50, message = "昵称长度必须在1-50个字符之间"))]
    pub guest_nickname: Option<String>,

    /// 游客邮箱 (游客必填, email格式)
    #[validate(regex(path = "*EMAIL_REGEX", message = "邮箱格式不正确"))]
    pub guest_email: Option<String>,

    /// 游客网站 (可选, URL格式)
    #[validate(regex(path = "*URL_REGEX", message = "网站URL必须以http://或https://开头"))]
    pub guest_website: Option<String>,
}

/// 更新评论请求 (仅Admin)
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    /// 新评论内容 (可选, 1-1000字符)
    #[validate(length(min = 1, max = 1000, message = "评论内容长度必须在1-1000个字符之间"))]
    pub content: Option<String>,

    /// 审核状态 (可选)
    pub is_reviewed: Option<bool>,
}

/// 审核评论请求
#[derive(Debug, Deserialize, Validate)]
pub struct ReviewCommentRequest {
    /// 审核结果: true=通过, false=拒绝
    pub is_approved: bool,
}

/// 批量审核请求
#[derive(Debug, Deserialize, Validate)]
pub struct BatchReviewRequest {
    /// 评论ID列表
    #[validate(length(min = 1, message = "ID列表不能为空"))]
    pub ids: Vec<i64>,

    /// 审核结果: true=通过, false=拒绝
    pub is_approved: bool,
}

// ===== Response DTOs =====

/// 评论响应 (扁平结构)
#[derive(Debug, Clone, Serialize)]
pub struct CommentResponse {
    /// 评论ID
    pub id: i64,

    /// 文章ID
    pub post_id: i64,

    /// 评论内容
    pub content: String,

    /// 父评论ID
    pub parent_id: Option<i64>,

    /// 作者信息
    pub author: CommentAuthor,

    /// 是否已审核
    pub is_reviewed: bool,

    /// UserAgent
    pub ua: Option<String>,

    /// IP地址 (管理员可见)
    pub ip: Option<String>,

    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// 评论树节点 (递归结构)
#[derive(Debug, Serialize)]
pub struct CommentNode {
    /// 评论信息
    #[serde(flatten)]
    pub comment: CommentResponse,

    /// 子评论列表
    pub children: Vec<CommentNode>,
}

/// 分页评论列表响应
#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    /// 评论列表
    pub comments: Vec<CommentResponse>,

    /// 总数
    pub total: u64,
}

// ===== 嵌套结构 =====

/// 评论作者信息
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum CommentAuthor {
    /// 登录用户
    #[serde(rename = "user")]
    User {
        id: i64,
        username: String,
        nickname: Option<String>,
        avatar_url: Option<String>,
    },
    /// 游客
    #[serde(rename = "guest")]
    Guest {
        nickname: String,
        email: String,
        website: Option<String>,
    },
}
