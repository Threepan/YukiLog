use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
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
