use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

use crate::{
    entities::post_tags,
    repo::error::{RepoError, RepoResult},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostTagDto {
    pub post_id: i64,
    pub tag_id: i64,
}

impl From<post_tags::Model> for PostTagDto {
    fn from(model: post_tags::Model) -> Self {
        Self {
            post_id: model.post_id,
            tag_id: model.tag_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CreatePostTag {
    pub post_id: i64,
    pub tag_id: i64,
}

pub async fn create_post_tag<C>(db: &C, input: CreatePostTag) -> RepoResult<PostTagDto>
where
    C: ConnectionTrait,
{
    let active = post_tags::ActiveModel {
        post_id: sea_orm::ActiveValue::Set(input.post_id),
        tag_id: sea_orm::ActiveValue::Set(input.tag_id),
    };

    let model = active.insert(db).await?;
    Ok(PostTagDto::from(model))
}

#[allow(dead_code)]
pub async fn get_post_tag<C>(db: &C, post_id: i64, tag_id: i64) -> RepoResult<PostTagDto>
where
    C: ConnectionTrait,
{
    let model = post_tags::Entity::find_by_id((post_id, tag_id))
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    Ok(PostTagDto::from(model))
}

#[allow(dead_code)]
pub async fn list_post_tags<C>(db: &C) -> RepoResult<Vec<PostTagDto>>
where
    C: ConnectionTrait,
{
    let models = post_tags::Entity::find().all(db).await?;
    Ok(models.into_iter().map(PostTagDto::from).collect())
}

#[allow(dead_code)]
pub async fn list_post_tags_by_post_id<C>(db: &C, post_id: i64) -> RepoResult<Vec<PostTagDto>>
where
    C: ConnectionTrait,
{
    let models = post_tags::Entity::find()
        .filter(post_tags::Column::PostId.eq(post_id))
        .all(db)
        .await?;

    Ok(models.into_iter().map(PostTagDto::from).collect())
}

#[allow(dead_code)]
pub async fn list_post_tags_by_tag_id<C>(db: &C, tag_id: i64) -> RepoResult<Vec<PostTagDto>>
where
    C: ConnectionTrait,
{
    let models = post_tags::Entity::find()
        .filter(post_tags::Column::TagId.eq(tag_id))
        .all(db)
        .await?;

    Ok(models.into_iter().map(PostTagDto::from).collect())
}

pub async fn delete_post_tag<C>(db: &C, post_id: i64, tag_id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    let res = post_tags::Entity::delete_many()
        .filter(post_tags::Column::PostId.eq(post_id))
        .filter(post_tags::Column::TagId.eq(tag_id))
        .exec(db)
        .await?;

    if res.rows_affected == 0 {
        return Err(RepoError::NotFound);
    }

    Ok(())
}

pub async fn get_tags_by_post_id<C>(db: &C, post_id: i64) -> RepoResult<Vec<crate::repo::tags::TagDto>>
where
    C: ConnectionTrait,
{
    use crate::entities::prelude::Tags;

    let tags_models = Tags::find()
        .inner_join(post_tags::Entity)
        .filter(post_tags::Column::PostId.eq(post_id))
        .all(db)
        .await?;

    Ok(tags_models.into_iter().map(crate::repo::tags::TagDto::from).collect())
}

/// 将 source tag 的所有文章关联迁移到 target tag（主键冲突时忽略）
pub async fn migrate_post_tags<C>(
    db: &C,
    source_tag_id: i64,
    target_tag_id: i64,
) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    use sea_orm::Statement;
    let sql = r#"
        INSERT INTO post_tags (post_id, tag_id)
        SELECT post_id, $1
        FROM post_tags
        WHERE tag_id = $2
        ON CONFLICT DO NOTHING
    "#;
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![target_tag_id.into(), source_tag_id.into()],
    );
    db.execute(stmt).await?;
    Ok(())
}

/// 删除指定标签的所有关联记录
pub async fn delete_post_tags_by_tag<C>(db: &C, tag_id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    post_tags::Entity::delete_many()
        .filter(post_tags::Column::TagId.eq(tag_id))
        .exec(db)
        .await?;
    Ok(())
}
