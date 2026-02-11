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
pub async fn create_tag<C: ConnectionTrait>(db: &C, input: CreateTagInput) -> ServiceResult<Tag> {
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
pub async fn get_or_create_tag<C: ConnectionTrait>(
    db: &C,
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
pub async fn update_tag(db: &DatabaseConnection, id: i64, input: UpdateTagInput) -> ServiceResult<Tag> {
    // 如果要修改 slug，先校验新 slug 格式
    if let Some(ref new_slug) = input.slug {
        validate_slug(new_slug)?;
    }

    // 确保标签存在
    repo_tags::get_tag_by_id(db, id)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    let repo_patch = RepoUpdateTag {
        name: input.name,
        slug: input.slug,
    };

    let dto = repo_tags::update_tag(db, id, repo_patch).await?;
    Ok(Tag::from(dto))
}

/// 6. 删除标签（管理后台）
pub async fn delete_tag(db: &DatabaseConnection, id: i64) -> ServiceResult<()> {
    repo_tags::delete_tag(db, id)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    Ok(())
}

/// 7. 合并标签（管理后台）
/// 将 source_ids 的所有文章关联迁移到 target_id，然后删除 source_ids
pub async fn merge_tags(
    db: &DatabaseConnection,
    target_id: i64,
    source_ids: &[i64],
) -> ServiceResult<Tag> {
    if source_ids.is_empty() {
        return Err(ServiceError::InvalidInput("source_ids cannot be empty".to_string()));
    }
    if source_ids.iter().any(|&id| id == target_id) {
        return Err(ServiceError::InvalidInput(
            "cannot merge a tag into itself".to_string(),
        ));
    }

    // 确保目标标签存在
    repo_tags::get_tag_by_id(db, target_id)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;

    // 事务：迁移关联 -> 删除源标签 -> 纠正 target 的 post_count
    let txn: DatabaseTransaction = db.begin().await?;

    for &source_id in source_ids {
        // 将 source_id 的关联复制到 target_id（主键冲突时忽略）
        let insert_sql = r#"
            INSERT INTO post_tags (post_id, tag_id)
            SELECT post_id, $1
            FROM post_tags
            WHERE tag_id = $2
            ON CONFLICT DO NOTHING
        "#;
        let insert_stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            insert_sql,
            vec![target_id.into(), source_id.into()],
        );
        txn.execute(insert_stmt).await?;

        // 删除源标签的关联
        let delete_rel_sql = r#"DELETE FROM post_tags WHERE tag_id = $1"#;
        let delete_rel_stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            delete_rel_sql,
            vec![source_id.into()],
        );
        txn.execute(delete_rel_stmt).await?;

        // 删除源标签本体
        repo_tags::delete_tag(&txn, source_id)
            .await
            .map_err(|e| match e {
                RepoError::NotFound => ServiceError::NotFound,
                other => ServiceError::Repo(other),
            })?;
    }

    // 重新计算 target 的 post_count
    let fix_count_sql = r#"
        UPDATE tags
        SET post_count = (
            SELECT COUNT(*) FROM post_tags WHERE tag_id = $1
        )
        WHERE id = $1
    "#;
    let fix_count_stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        fix_count_sql,
        vec![target_id.into()],
    );
    txn.execute(fix_count_stmt).await?;

    txn.commit().await?;

    let dto = repo_tags::get_tag_by_id(db, target_id)
        .await
        .map_err(|e| match e {
            RepoError::NotFound => ServiceError::NotFound,
            other => ServiceError::Repo(other),
        })?;
    Ok(Tag::from(dto))
}

/// 8. 增加浏览计数（前台访问标签页时调用）
pub async fn increment_view_count<C: ConnectionTrait>(db: &C, tag_id: i64) -> ServiceResult<()> {
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
pub async fn adjust_post_count<C: ConnectionTrait>(
    db: &C,
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

// ================================
// 辅助函数（给其他 service 调用）
// ================================

/// 通过多个 slug 批量获取标签 ID
pub async fn get_tag_ids_by_slugs<C: ConnectionTrait>(
    db: &C,
    slugs: &[String],
) -> ServiceResult<Vec<i64>> {
    use crate::entities::prelude::Tags;
    use crate::entities::tags::Column;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let tags = Tags::find()
        .filter(Column::Slug.is_in(slugs.iter().cloned()))
        .all(db)
        .await?;

    Ok(tags.into_iter().map(|t| t.id).collect())
}
