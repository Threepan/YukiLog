use sea_orm::{ConnectionTrait, DatabaseConnection, Order, QueryOrder, Statement};
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
pub async fn get_theme_by_slug(db: &DatabaseConnection, slug: &str) -> ServiceResult<Theme> {
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
    use crate::entities::themes::{Column, Entity};
    use sea_orm::EntityTrait;

    let sort = sort_by.unwrap_or(ThemeSortBy::PostCount);

    let query = match sort {
        ThemeSortBy::PostCount => Entity::find().order_by(Column::PostCount, Order::Desc),
        ThemeSortBy::ViewCount => Entity::find().order_by(Column::ViewCount, Order::Desc),
        ThemeSortBy::CreatedAt => Entity::find().order_by(Column::CreatedAt, Order::Desc),
    };

    let models = query.all(db).await?;

    let themes = models
        .into_iter()
        .map(|model| Theme::from(repo_themes::ThemeDto::from(model)))
        .collect();

    Ok(themes)
}

/// 4. 更新主题信息（管理后台，允许修改 slug）
pub async fn update_theme(
    db: &DatabaseConnection,
    current_slug: &str,
    input: UpdateThemeInput,
) -> ServiceResult<Theme> {
    // 如果要修改 slug，先校验新 slug 格式
    if let Some(ref new_slug) = input.slug {
        validate_slug(new_slug)?;
    }

    // 先通过 current_slug 获取主题 id
    let current = repo_themes::get_theme_by_slug(db, current_slug)
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

    let dto = repo_themes::update_theme(db, current.id, repo_patch).await?;
    Ok(Theme::from(dto))
}

/// 5. 删除主题（管理后台）
pub async fn delete_theme(db: &DatabaseConnection, slug: &str) -> ServiceResult<()> {
    let theme = repo_themes::get_theme_by_slug(db, slug)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    repo_themes::delete_theme(db, theme.id)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    Ok(())
}

/// 6. 增加浏览计数（前台访问主题页时调用）
pub async fn increment_view_count(
    db: &DatabaseConnection,
    theme_id: i64,
) -> ServiceResult<()> {
    let sql = "UPDATE themes SET view_count = view_count + 1 WHERE id = $1";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![theme_id.into()],
    );

    db.execute(stmt).await?;
    Ok(())
}

/// 7. 调整文章计数（给 post service 调用）
/// delta: +1 表示新文章绑定此主题，-1 表示文章解绑/删除
pub async fn adjust_post_count(
    db: &DatabaseConnection,
    theme_id: i64,
    delta: i32,
) -> ServiceResult<()> {
    let sql = "UPDATE themes SET post_count = post_count + $1 WHERE id = $2";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![delta.into(), theme_id.into()],
    );

    db.execute(stmt).await?;
    Ok(())
}
