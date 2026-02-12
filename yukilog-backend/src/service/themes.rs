use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::repo::{
    error::RepoError,
    themes::{self as repo_themes, CreateTheme as RepoCreateTheme, UpdateTheme as RepoUpdateTheme},
};

use super::error::{ServiceError, ServiceResult};

// ============================================
// DTO 定义
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemeSortBy {
    PostCount,
    ViewCount,
    CreatedAt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateThemeInput {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateThemeInput {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Theme {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub post_count: i32,
    pub view_count: i64,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl From<repo_themes::ThemeDto> for Theme {
    fn from(dto: repo_themes::ThemeDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            slug: dto.slug,
            description: dto.description,
            post_count: dto.post_count.unwrap_or(0),
            view_count: dto.view_count.unwrap_or(0),
            created_at: dto
                .created_at
                .unwrap_or_else(|| chrono::Utc::now().into()),
        }
    }
}

// ============================================
// 辅助函数
// ============================================

/// 校验 slug 格式：仅允许字母、数字、下划线、连字符
fn validate_slug(slug: &str) -> ServiceResult<()> {
    if slug.is_empty() {
        return Err(ServiceError::InvalidInput(
            "slug cannot be empty".to_string(),
        ));
    }

    if !slug
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(ServiceError::InvalidInput(
            "slug can only contain letters, numbers, underscores and hyphens".to_string(),
        ));
    }

    Ok(())
}

// ============================================
// Service 接口实现
// ============================================

/// 1. 创建主题（管理后台）
pub async fn create_theme(
    db: &DatabaseConnection,
    input: CreateThemeInput,
) -> ServiceResult<Theme> {
    validate_slug(&input.slug)?;

    let repo_input = RepoCreateTheme {
        name: input.name,
        slug: input.slug,
        description: input.description,
    };

    let dto = repo_themes::create_theme(db, repo_input).await?;
    Ok(Theme::from(dto))
}

/// 2. 获取主题详情（前台/后台）
pub async fn get_theme_by_slug<C: ConnectionTrait>(db: &C, slug: &str) -> ServiceResult<Theme> {
    let dto = repo_themes::get_theme_by_slug(db, slug)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    Ok(Theme::from(dto))
}

/// 3. 列出所有主题（前台导航/后台列表）
/// sort_by: None 时默认按 PostCount 倒序
pub async fn list_all_themes(
    db: &DatabaseConnection,
    sort_by: Option<ThemeSortBy>,
) -> ServiceResult<Vec<Theme>> {
    let sort = sort_by.unwrap_or(ThemeSortBy::PostCount);
    let (sort_column, order_desc) = match sort {
        ThemeSortBy::PostCount => ("post_count", true),
        ThemeSortBy::ViewCount => ("view_count", true),
        ThemeSortBy::CreatedAt => ("created_at", true),
    };

    let dtos = repo_themes::list_themes_sorted(db, sort_column, order_desc).await?;
    Ok(dtos.into_iter().map(Theme::from).collect())
}

/// 4. 更新主题信息（管理后台，允许修改 slug）
pub async fn update_theme(
    db: &DatabaseConnection,
    id: i64,
    input: UpdateThemeInput,
) -> ServiceResult<Theme> {
    // 如果要修改 slug，先校验新 slug 格式
    if let Some(ref new_slug) = input.slug {
        validate_slug(new_slug)?;
    }

    // 确保主题存在
    repo_themes::get_theme_by_id(db, id)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    let repo_patch = RepoUpdateTheme {
        name: input.name,
        slug: input.slug,
        description: input.description,
    };

    let dto = repo_themes::update_theme(db, id, repo_patch).await?;
    Ok(Theme::from(dto))
}

/// 5. 删除主题（管理后台）
pub async fn delete_theme(db: &DatabaseConnection, id: i64) -> ServiceResult<()> {
    repo_themes::delete_theme(db, id)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    Ok(())
}

/// 6. 增加浏览计数（前台访问主题页时调用）
pub async fn increment_view_count<C: ConnectionTrait>(
    db: &C,
    theme_id: i64,
) -> ServiceResult<()> {
    repo_themes::increment_view_count(db, theme_id).await?;
    Ok(())
}

/// 7. 调整文章计数（给 post service 调用）
/// delta: +1 表示新文章绑定此主题，-1 表示文章解绑/删除
pub async fn adjust_post_count<C: ConnectionTrait>(
    db: &C,
    theme_id: i64,
    delta: i32,
) -> ServiceResult<()> {
    repo_themes::adjust_post_count(db, theme_id, delta).await?;
    Ok(())
}

// ================================
// 辅助函数（给其他 service 调用）
// ================================

/// 通过 ID 获取主题
pub async fn get_theme_by_id<C: ConnectionTrait>(
    db: &C,
    theme_id: i64,
) -> ServiceResult<Theme> {
    let dto = repo_themes::get_theme_by_id(db, theme_id).await?;
    Ok(dto.into())
}

/// 批量按ID获取主题（返回完整 Theme 对象）
pub async fn get_themes_by_ids<C: ConnectionTrait>(
    db: &C,
    ids: &[i64],
) -> ServiceResult<Vec<Theme>> {
    let themes = repo_themes::get_themes_by_ids(db, ids).await?;
    Ok(themes.into_iter().map(Into::into).collect())
}

/// 通过多个 slug 批量获取主题 ID
pub async fn get_theme_ids_by_slugs<C: ConnectionTrait>(
    db: &C,
    slugs: &[String],
) -> ServiceResult<Vec<i64>> {
    let ids = repo_themes::get_theme_ids_by_slugs(db, slugs).await?;
    Ok(ids)
}
