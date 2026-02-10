use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::{
    entities::themes,
    repo::error::{RepoError, RepoResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeDto {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub post_count: Option<i32>,
    pub view_count: Option<i64>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl From<themes::Model> for ThemeDto {
    fn from(model: themes::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            description: model.description,
            post_count: model.post_count,
            view_count: model.view_count,
            created_at: model.created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateTheme {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateTheme {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<Option<String>>,
}

pub async fn create_theme<C>(db: &C, input: CreateTheme) -> RepoResult<ThemeDto>
where
    C: ConnectionTrait,
{
    let active = themes::ActiveModel {
        name: Set(input.name),
        slug: Set(input.slug),
        description: Set(input.description),
        ..Default::default()
    };

    let model = active.insert(db).await?;
    Ok(ThemeDto::from(model))
}

pub async fn get_theme_by_id<C>(db: &C, id: i64) -> RepoResult<ThemeDto>
where
    C: ConnectionTrait,
{
    let model = themes::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    Ok(ThemeDto::from(model))
}

pub async fn get_theme_by_slug<C>(db: &C, slug: &str) -> RepoResult<ThemeDto>
where
    C: ConnectionTrait,
{
    let model = themes::Entity::find()
        .filter(themes::Column::Slug.eq(slug))
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    Ok(ThemeDto::from(model))
}

pub async fn list_themes<C>(db: &C) -> RepoResult<Vec<ThemeDto>>
where
    C: ConnectionTrait,
{
    let models = themes::Entity::find().all(db).await?;
    Ok(models.into_iter().map(ThemeDto::from).collect())
}

pub async fn update_theme<C>(db: &C, id: i64, patch: UpdateTheme) -> RepoResult<ThemeDto>
where
    C: ConnectionTrait,
{
    let model = themes::Entity::find_by_id(id)
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
    if let Some(v) = patch.description {
        active.description = Set(v);
    }

    let updated = active.update(db).await?;
    Ok(ThemeDto::from(updated))
}

pub async fn delete_theme<C>(db: &C, id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    let res = themes::Entity::delete_by_id(id).exec(db).await?;
    if res.rows_affected == 0 {
        return Err(RepoError::NotFound);
    }
    Ok(())
}
