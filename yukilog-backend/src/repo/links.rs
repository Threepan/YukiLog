use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, QueryFilter,
    QueryOrder, Set,
};

use crate::{
    domain::status::LinkStatus,
    entities::links,
    repo::error::{RepoError, RepoResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkDto {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub status: Option<LinkStatus>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl TryFrom<links::Model> for LinkDto {
    type Error = RepoError;

    fn try_from(model: links::Model) -> Result<Self, Self::Error> {
        let status = match model.status.as_deref() {
            None => None,
            Some(s) => Some(LinkStatus::try_from(s)?),
        };

        Ok(Self {
            id: model.id,
            title: model.title,
            url: model.url,
            avatar: model.avatar,
            description: model.description,
            status,
            created_at: model.created_at,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateLink {
    pub title: String,
    pub url: String,
    pub avatar: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateLink {
    pub title: Option<String>,
    pub url: Option<String>,
    pub avatar: Option<Option<String>>,
    pub description: Option<Option<String>>,
    pub status: Option<Option<LinkStatus>>,
}

pub async fn create_link<C>(db: &C, input: CreateLink) -> RepoResult<LinkDto>
where
    C: ConnectionTrait,
{
    let active = links::ActiveModel {
        title: Set(input.title),
        url: Set(input.url),
        avatar: Set(input.avatar),
        description: Set(input.description),
        ..Default::default()
    };

    let model = active.insert(db).await?;
    LinkDto::try_from(model)
}

pub async fn get_link_by_id<C>(db: &C, id: i64) -> RepoResult<LinkDto>
where
    C: ConnectionTrait,
{
    let model = links::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    LinkDto::try_from(model)
}

pub async fn get_link_by_url<C>(db: &C, url: &str) -> RepoResult<LinkDto>
where
    C: ConnectionTrait,
{
    let model = links::Entity::find()
        .filter(links::Column::Url.eq(url))
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    LinkDto::try_from(model)
}

pub async fn list_links<C>(db: &C) -> RepoResult<Vec<LinkDto>>
where
    C: ConnectionTrait,
{
    let models = links::Entity::find().all(db).await?;

    models
        .into_iter()
        .map(LinkDto::try_from)
        .collect::<Result<Vec<_>, _>>()
}

pub async fn update_link<C>(db: &C, id: i64, patch: UpdateLink) -> RepoResult<LinkDto>
where
    C: ConnectionTrait,
{
    let model = links::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    let mut active = model.into_active_model();

    if let Some(v) = patch.title {
        active.title = Set(v);
    }
    if let Some(v) = patch.url {
        active.url = Set(v);
    }
    if let Some(v) = patch.avatar {
        active.avatar = Set(v);
    }
    if let Some(v) = patch.description {
        active.description = Set(v);
    }
    if let Some(v) = patch.status {
        active.status = Set(v.map(|s| s.as_str().to_string()));
    }

    let updated = active.update(db).await?;
    LinkDto::try_from(updated)
}

pub async fn delete_link<C>(db: &C, id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    let res = links::Entity::delete_by_id(id).exec(db).await?;
    if res.rows_affected == 0 {
        return Err(RepoError::NotFound);
    }
    Ok(())
}

/// 按条件筛选友链列表（支持按状态筛选和排序）
pub async fn list_links_filtered<C>(
    db: &C,
    status: Option<&str>,
    sort_asc: bool,
) -> RepoResult<Vec<LinkDto>>
where
    C: ConnectionTrait,
{
    let mut query = links::Entity::find();

    if let Some(s) = status {
        query = query.filter(links::Column::Status.eq(s));
    }

    if sort_asc {
        query = query.order_by_asc(links::Column::CreatedAt);
    } else {
        query = query.order_by_desc(links::Column::CreatedAt);
    }

    let models = query.all(db).await?;
    models
        .into_iter()
        .map(LinkDto::try_from)
        .collect::<Result<Vec<_>, _>>()
}
