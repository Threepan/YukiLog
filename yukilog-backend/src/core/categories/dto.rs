//! 分类服务数据传输对象
//!
//! 定义分类管理相关的请求和响应结构

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

// 定义 Slug 格式校验正则：仅小写字母、数字、连字符
static SLUG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-z0-9-]+$").expect("Invalid regex"));

// ==================== 请求 DTO ====================

/// 创建分类请求
///
/// # 使用场景
/// - 管理员创建新分类
///
/// # 校验规则
/// - `name`: 1-50 字符，唯一
/// - `slug`: 1-50 字符，仅小写字母、数字、连字符，唯一
/// - `description`: 最多 500 字符
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    /// 分类名称（唯一）
    ///
    /// # 校验规则
    /// - 长度: 1-50 字符
    #[validate(length(min = 1, max = 50, message = "分类名称长度必须在 1-50 字符之间"))]
    pub name: String,

    /// URL slug（唯一）
    ///
    /// # 校验规则
    /// - 长度: 1-50 字符
    /// - 格式: 仅小写字母、数字、连字符
    ///
    /// # 示例
    /// - `rust-programming`
    /// - `web-development`
    /// - `2024-recap`
    #[validate(length(min = 1, max = 50, message = "Slug 长度必须在 1-50 字符之间"))]
    #[validate(regex(path = "*SLUG_REGEX", message = "Slug 只能包含小写字母、数字和连字符"))]
    pub slug: String,

    /// 分类描述（可选）
    ///
    /// # 校验规则
    /// - 最多 500 字符
    #[validate(length(max = 500, message = "描述最多 500 字符"))]
    pub description: Option<String>,
}

/// 更新分类请求
///
/// # 使用场景
/// - 管理员更新分类信息
///
/// # 说明
/// - 所有字段都是可选的（部分更新）
/// - 更新时需要检查 name/slug 的唯一性（排除自身）
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    /// 分类名称（可选）
    #[validate(length(min = 1, max = 50, message = "分类名称长度必须在 1-50 字符之间"))]
    pub name: Option<String>,

    /// URL slug（可选）
    #[validate(length(min = 1, max = 50, message = "Slug 长度必须在 1-50 字符之间"))]
    #[validate(regex(path = "*SLUG_REGEX", message = "Slug 只能包含小写字母、数字和连字符"))]
    pub slug: Option<String>,

    /// 分类描述（可选）
    #[validate(length(max = 500, message = "描述最多 500 字符"))]
    pub description: Option<String>,
}

// ==================== 响应 DTO ====================

/// 分类响应
///
/// # 使用场景
/// - 分类详情
/// - 分类列表
///
/// # 说明
/// - 包含该分类下的文章数（仅已发布文章）
#[derive(Debug, Clone, Serialize)]
pub struct CategoryResponse {
    /// 分类 ID
    pub id: i64,

    /// 分类名称
    pub name: String,

    /// URL slug
    pub slug: String,

    /// 分类描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// 该分类下的文章数（仅已发布）
    pub post_count: u64,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

impl CategoryResponse {
    /// 从 Category Entity 和文章数创建响应
    ///
    /// # 参数
    /// - `category`: categories::Model 实体
    /// - `post_count`: 文章数量
    pub fn from_entity_with_count(
        category: &crate::entities::categories::Model,
        post_count: u64,
    ) -> Self {
        Self {
            id: category.id,
            name: category.name.clone(),
            slug: category.slug.clone(),
            description: category.description.clone(),
            post_count,
            created_at: category
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
            updated_at: category
                .updated_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
        }
    }
}
