use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::{
    entities::tags,
    repo::error::{RepoError, RepoResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagDto {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub post_count: Option<i32>,
    pub view_count: Option<i64>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl From<tags::Model> for TagDto {
    fn from(model: tags::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            post_count: model.post_count,
            view_count: model.view_count,
            created_at: model.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTag {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateTag {
    pub name: Option<String>,
    pub slug: Option<String>,
}

pub async fn create_tag<C>(db: &C, input: CreateTag) -> RepoResult<TagDto>
where
    C: ConnectionTrait,
{
    let active = tags::ActiveModel {
        name: Set(input.name),
        slug: Set(input.slug),
        ..Default::default()
    };

    let model = active.insert(db).await?;
    Ok(TagDto::from(model))
}

pub async fn get_tag_by_id<C>(db: &C, id: i64) -> RepoResult<TagDto>
where
    C: ConnectionTrait,
{
    let model = tags::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    Ok(TagDto::from(model))
}

pub async fn get_tag_by_slug<C>(db: &C, slug: &str) -> RepoResult<TagDto>
where
    C: ConnectionTrait,
{
    let model = tags::Entity::find()
        .filter(tags::Column::Slug.eq(slug))
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    Ok(TagDto::from(model))
}

pub async fn list_tags<C>(db: &C) -> RepoResult<Vec<TagDto>>
where
    C: ConnectionTrait,
{
    let models = tags::Entity::find().all(db).await?;
    Ok(models.into_iter().map(TagDto::from).collect())
}

pub async fn update_tag<C>(db: &C, id: i64, patch: UpdateTag) -> RepoResult<TagDto>
where
    C: ConnectionTrait,
{
    let model = tags::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    let mut active = model.into_active_model();

    if let Some(v) = patch.name {
        active.name = Set(v);
    }
    if let Some(v) = patch.slug {
        active.slug = Set(v);
    }

    let updated = active.update(db).await?;
    Ok(TagDto::from(updated))
}

pub async fn delete_tag<C>(db: &C, id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    let res = tags::Entity::delete_by_id(id).exec(db).await?;
    if res.rows_affected == 0 {
        return Err(RepoError::NotFound);
    }
    Ok(())
}

/// 按指定列排序获取标签列表（支持分页）
pub async fn list_tags_sorted<C>(
    db: &C,
    sort_by: &str,
    order_desc: bool,
    count: Option<u64>,
    page: Option<u64>,
) -> RepoResult<Vec<TagDto>>
where
    C: ConnectionTrait,
{
    let order = if order_desc {
        sea_orm::Order::Desc
    } else {
        sea_orm::Order::Asc
    };
    let column = match sort_by {
        "post_count" => tags::Column::PostCount,
        "view_count" => tags::Column::ViewCount,
        "name" => tags::Column::Name,
        _ => tags::Column::CreatedAt,
    };

    let query = tags::Entity::find().order_by(column, order);

    let models = match (count, page) {
        (Some(per_page), Some(page_num)) if page_num > 0 => {
            let paginator = query.paginate(db, per_page);
            paginator.fetch_page(page_num - 1).await?
        }
        _ => query.all(db).await?,
    };

    Ok(models.into_iter().map(TagDto::from).collect())
}

/// 将指定标签的 view_count + 1
pub async fn increment_view_count<C>(db: &C, id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    use sea_orm::Statement;
    let sql = "UPDATE tags SET view_count = view_count + 1 WHERE id = $1";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![id.into()],
    );
    db.execute(stmt).await?;
    Ok(())
}

/// 调整指定标签的 post_count（delta 可为正或负）
pub async fn adjust_post_count<C>(db: &C, id: i64, delta: i32) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    use sea_orm::Statement;
    let sql = "UPDATE tags SET post_count = post_count + $1 WHERE id = $2";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![delta.into(), id.into()],
    );
    db.execute(stmt).await?;
    Ok(())
}

/// 重新计算指定标签的 post_count
pub async fn recount_post_count<C>(db: &C, tag_id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    use sea_orm::Statement;
    let sql = r#"
        UPDATE tags
        SET post_count = (
            SELECT COUNT(*) FROM post_tags WHERE tag_id = $1
        )
        WHERE id = $1
    "#;
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![tag_id.into()],
    );
    db.execute(stmt).await?;
    Ok(())
}

/// 通过多个 slug 批量获取标签 ID
pub async fn get_tag_ids_by_slugs<C>(db: &C, slugs: &[String]) -> RepoResult<Vec<i64>>
where
    C: ConnectionTrait,
{
    let models = tags::Entity::find()
        .filter(tags::Column::Slug.is_in(slugs.iter().cloned()))
        .all(db)
        .await?;
    Ok(models.into_iter().map(|m| m.id).collect())
}
