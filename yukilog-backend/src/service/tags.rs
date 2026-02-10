use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, Order, QueryOrder, Statement,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};

use crate::repo::{
    error::RepoError,
    tags::{self as repo_tags, CreateTag as RepoCreateTag, UpdateTag as RepoUpdateTag},
};

use super::error::{ServiceError, ServiceResult};

// ============================================
// DTO 定义
// ============================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TagSortBy {
    PostCount,
    ViewCount,
    CreatedAt,
    Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagInput {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateTagInput {
    pub name: Option<String>,
    pub slug: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub post_count: i32,
    pub view_count: i64,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}

impl From<repo_tags::TagDto> for Tag {
    fn from(dto: repo_tags::TagDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            slug: dto.slug,
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

/// 1. 创建标签（管理后台）
pub async fn create_tag(db: &DatabaseConnection, input: CreateTagInput) -> ServiceResult<Tag> {
    validate_slug(&input.slug)?;

    let repo_input = RepoCreateTag {
        name: input.name,
        slug: input.slug,
    };

    let dto = repo_tags::create_tag(db, repo_input).await?;
    Ok(Tag::from(dto))
}

/// 2. 获取或创建标签（发布文章时调用）
/// 如果 slug 已存在则返回现有标签，否则创建新标签
/// 注意：如果标签已存在，不会覆盖现有 name
pub async fn get_or_create_tag(
    db: &DatabaseConnection,
    name: &str,
    slug: &str,
) -> ServiceResult<Tag> {
    // 先尝试查询
    match repo_tags::get_tag_by_slug(db, slug).await {
        Ok(dto) => Ok(Tag::from(dto)),
        Err(RepoError::NotFound) => {
            // 不存在则创建
            validate_slug(slug)?;
            let input = CreateTagInput {
                name: name.to_string(),
                slug: slug.to_string(),
            };
            create_tag(db, input).await
        }
        Err(e) => Err(ServiceError::Repo(e)),
    }
}

/// 3. 获取标签详情
pub async fn get_tag_by_slug(db: &DatabaseConnection, slug: &str) -> ServiceResult<Tag> {
    let dto = repo_tags::get_tag_by_slug(db, slug)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    Ok(Tag::from(dto))
}

/// 4. 列出所有标签（前台标签云/后台列表）
/// count: 每页数量，page: 页码（从 1 开始）
/// 当 count 或 page 为 None 时，返回全部标签
pub async fn list_all_tags(
    db: &DatabaseConnection,
    sort_by: Option<TagSortBy>,
    count: Option<u64>,
    page: Option<u64>,
) -> ServiceResult<Vec<Tag>> {
    use crate::entities::tags::{Column, Entity};
    use sea_orm::{EntityTrait, PaginatorTrait};

    let sort = sort_by.unwrap_or(TagSortBy::PostCount);

    let query = match sort {
        TagSortBy::PostCount => Entity::find().order_by(Column::PostCount, Order::Desc),
        TagSortBy::ViewCount => Entity::find().order_by(Column::ViewCount, Order::Desc),
        TagSortBy::CreatedAt => Entity::find().order_by(Column::CreatedAt, Order::Desc),
        TagSortBy::Name => Entity::find().order_by(Column::Name, Order::Asc),
    };

    // 分页处理
    let models = match (count, page) {
        (Some(per_page), Some(page_num)) if page_num > 0 => {
            let paginator = query.paginate(db, per_page);
            paginator.fetch_page(page_num - 1).await?
        }
        _ => query.all(db).await?,
    };

    let tags = models
        .into_iter()
        .map(|model| Tag::from(repo_tags::TagDto::from(model)))
        .collect();

    Ok(tags)
}

/// 5. 更新标签信息（管理后台，允许修改 slug）
pub async fn update_tag(
    db: &DatabaseConnection,
    current_slug: &str,
    input: UpdateTagInput,
) -> ServiceResult<Tag> {
    // 如果要修改 slug，先校验新 slug 格式
    if let Some(ref new_slug) = input.slug {
        validate_slug(new_slug)?;
    }

    // 先通过 current_slug 获取标签 id
    let current = repo_tags::get_tag_by_slug(db, current_slug)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    let repo_patch = RepoUpdateTag {
        name: input.name,
        slug: input.slug,
    };

    let dto = repo_tags::update_tag(db, current.id, repo_patch).await?;
    Ok(Tag::from(dto))
}

/// 6. 删除标签（管理后台）
pub async fn delete_tag(db: &DatabaseConnection, slug: &str) -> ServiceResult<()> {
    let tag = repo_tags::get_tag_by_slug(db, slug)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    repo_tags::delete_tag(db, tag.id)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    Ok(())
}

/// 7. 合并标签（管理后台）
/// 将 from_slug 的所有文章转移到 to_slug，然后删除 from_slug
pub async fn merge_tags(
    db: &DatabaseConnection,
    from_slug: &str,
    to_slug: &str,
) -> ServiceResult<()> {
    // 获取源标签和目标标签
    let from_tag = repo_tags::get_tag_by_slug(db, from_slug)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    let to_tag = repo_tags::get_tag_by_slug(db, to_slug)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    if from_tag.id == to_tag.id {
        return Err(ServiceError::InvalidInput(
            "cannot merge a tag into itself".to_string(),
        ));
    }

    let txn: DatabaseTransaction = db.begin().await?;

    // 1. 更新 post_tags：将 from_tag_id 的关联改为 to_tag_id
    //    忽略重复关联（如果某篇文章同时有这两个标签）
    let update_sql = r#"
        UPDATE post_tags 
        SET tag_id = $1 
        WHERE tag_id = $2 
        AND post_id NOT IN (
            SELECT post_id FROM post_tags WHERE tag_id = $1
        )
    "#;

    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        update_sql,
        vec![to_tag.id.into(), from_tag.id.into()],
    );

    txn.execute(stmt).await?;

    // 2. 删除剩余的 from_tag 关联（重复的部分）
    let delete_sql = "DELETE FROM post_tags WHERE tag_id = $1";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        delete_sql,
        vec![from_tag.id.into()],
    );

    txn.execute(stmt).await?;

    // 3. 同步目标标签的文章计数
    let count_sql = "SELECT COUNT(*) FROM post_tags WHERE tag_id = $1";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        count_sql,
        vec![to_tag.id.into()],
    );

    use sea_orm::QueryResult;
    let result: Option<QueryResult> = txn.query_one(stmt).await?;
    let new_count: i64 = result
        .ok_or_else(|| ServiceError::InvalidInput("failed to count posts".to_string()))?
        .try_get("", "count")?;

    let update_count_sql = "UPDATE tags SET post_count = $1 WHERE id = $2";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        update_count_sql,
        vec![(new_count as i32).into(), to_tag.id.into()],
    );

    txn.execute(stmt).await?;

    // 4. 删除源标签
    repo_tags::delete_tag(&txn, from_tag.id).await?;

    txn.commit().await?;

    Ok(())
}

/// 8. 增加浏览计数（前台访问标签页时调用）
pub async fn increment_view_count(db: &DatabaseConnection, tag_id: i64) -> ServiceResult<()> {
    let sql = "UPDATE tags SET view_count = view_count + 1 WHERE id = $1";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![tag_id.into()],
    );

    db.execute(stmt).await?;
    Ok(())
}

/// 9. 调整文章计数（给 post service 调用）
/// delta: +1 表示文章新增此标签，-1 表示文章移除此标签
pub async fn adjust_post_count(
    db: &DatabaseConnection,
    tag_id: i64,
    delta: i32,
) -> ServiceResult<()> {
    let sql = "UPDATE tags SET post_count = post_count + $1 WHERE id = $2";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![delta.into(), tag_id.into()],
    );

    db.execute(stmt).await?;
    Ok(())
}
