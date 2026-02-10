use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::{
    domain::status::PostStatus,
    entities::posts,
    repo::error::{RepoError, RepoResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostDto {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub cover_image: Option<String>,
    pub status: Option<PostStatus>,
    pub theme_id: Option<i64>,
    pub view_count: Option<i64>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub updated_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl TryFrom<posts::Model> for PostDto {
    type Error = RepoError;

    fn try_from(model: posts::Model) -> Result<Self, Self::Error> {
        let status = match model.status.as_deref() {
            None => None,
            Some(s) => Some(PostStatus::try_from(s)?),
        };

        Ok(Self {
            id: model.id,
            title: model.title,
            slug: model.slug,
            summary: model.summary,
            content: model.content,
            cover_image: model.cover_image,
            status,
            theme_id: model.theme_id,
            view_count: model.view_count,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreatePost {
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub cover_image: Option<String>,
    pub status: Option<String>,
    pub theme_id: Option<i64>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<Option<String>>,
    pub content: Option<String>,
    pub cover_image: Option<Option<String>>,
    pub status: Option<Option<PostStatus>>,
    pub theme_id: Option<Option<i64>>,
}

pub async fn create_post<C>(db: &C, input: CreatePost) -> RepoResult<PostDto>
where
    C: ConnectionTrait,
{
    let active = posts::ActiveModel {
        title: Set(input.title),
        slug: Set(input.slug),
        summary: Set(input.summary),
        content: Set(input.content),
        cover_image: Set(input.cover_image),
        status: Set(input.status),
        theme_id: Set(input.theme_id),
        ..Default::default()
    };

    let model = active.insert(db).await?;
    PostDto::try_from(model)
}

pub async fn get_post_by_id<C>(db: &C, id: i64) -> RepoResult<PostDto>
where
    C: ConnectionTrait,
{
    let model = posts::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    PostDto::try_from(model)
}

pub async fn get_post_by_slug<C>(db: &C, slug: &str) -> RepoResult<PostDto>
where
    C: ConnectionTrait,
{
    let model = posts::Entity::find()
        .filter(posts::Column::Slug.eq(slug))
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    PostDto::try_from(model)
}

pub async fn list_posts<C>(db: &C) -> RepoResult<Vec<PostDto>>
where
    C: ConnectionTrait,
{
    let models = posts::Entity::find().all(db).await?;

    models
        .into_iter()
        .map(PostDto::try_from)
        .collect::<Result<Vec<_>, _>>()
}

pub async fn update_post<C>(db: &C, id: i64, patch: UpdatePost) -> RepoResult<PostDto>
where
    C: ConnectionTrait,
{
    let model = posts::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    let mut active = model.into_active_model();

    if let Some(v) = patch.title {
        active.title = Set(v);
    }
    if let Some(v) = patch.slug {
        active.slug = Set(v);
    }
    if let Some(v) = patch.summary {
        active.summary = Set(v);
    }
    if let Some(v) = patch.content {
        active.content = Set(v);
    }
    if let Some(v) = patch.cover_image {
        active.cover_image = Set(v);
    }
    if let Some(v) = patch.status {
        active.status = Set(v.map(|s| s.as_str().to_string()));
    }
    if let Some(v) = patch.theme_id {
        active.theme_id = Set(v);
    }

    let updated = active.update(db).await?;
    PostDto::try_from(updated)
}

pub async fn delete_post<C>(db: &C, id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    let res = posts::Entity::delete_by_id(id).exec(db).await?;
    if res.rows_affected == 0 {
        return Err(RepoError::NotFound);
    }
    Ok(())
}

pub async fn increment_view_count<C>(db: &C, id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    use sea_orm::Statement;
    let sql = "UPDATE posts SET view_count = view_count + 1 WHERE id = $1";
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Postgres,
        sql,
        vec![id.into()],
    );
    db.execute(stmt).await?;
    Ok(())
}
