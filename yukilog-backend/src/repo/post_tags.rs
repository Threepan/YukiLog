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

pub async fn list_post_tags<C>(db: &C) -> RepoResult<Vec<PostTagDto>>
where
    C: ConnectionTrait,
{
    let models = post_tags::Entity::find().all(db).await?;
    Ok(models.into_iter().map(PostTagDto::from).collect())
}

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
