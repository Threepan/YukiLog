use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, Set,
};

use crate::{
    domain::status::CommentStatus,
    entities::comments,
    repo::error::{RepoError, RepoResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentDto {
    pub id: i64,
    pub post_id: Option<i64>,
    pub content: String,
    pub guest_nick: String,
    pub guest_email: Option<String>,
    pub guest_website: Option<String>,
    pub parent_id: Option<i64>,
    pub root_id: Option<i64>,
    pub status: Option<CommentStatus>,
    pub ip: Option<String>,
    pub ua: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl TryFrom<comments::Model> for CommentDto {
    type Error = RepoError;

    fn try_from(model: comments::Model) -> Result<Self, Self::Error> {
        let status = match model.status.as_deref() {
            None => None,
            Some(s) => Some(CommentStatus::try_from(s)?),
        };

        Ok(Self {
            id: model.id,
            post_id: model.post_id,
            content: model.content,
            guest_nick: model.guest_nick,
            guest_email: model.guest_email,
            guest_website: model.guest_website,
            parent_id: model.parent_id,
            root_id: model.root_id,
            status,
            ip: model.ip,
            ua: model.ua,
            created_at: model.created_at,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateComment {
    pub post_id: Option<i64>,
    pub content: String,
    pub guest_nick: String,
    pub guest_email: Option<String>,
    pub guest_website: Option<String>,
    pub parent_id: Option<i64>,
    pub root_id: Option<i64>,
    pub ip: Option<String>,
    pub ua: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateComment {
    pub content: Option<String>,
    pub guest_nick: Option<String>,
    pub guest_email: Option<Option<String>>,
    pub guest_website: Option<Option<String>>,
    pub parent_id: Option<Option<i64>>,
    pub root_id: Option<Option<i64>>,
    pub status: Option<Option<CommentStatus>>,
    pub ip: Option<Option<String>>,
    pub ua: Option<Option<String>>,
}

pub async fn create_comment<C>(db: &C, input: CreateComment) -> RepoResult<CommentDto>
where
    C: ConnectionTrait,
{
    let active = comments::ActiveModel {
        post_id: Set(input.post_id),
        content: Set(input.content),
        guest_nick: Set(input.guest_nick),
        guest_email: Set(input.guest_email),
        guest_website: Set(input.guest_website),
        parent_id: Set(input.parent_id),
        root_id: Set(input.root_id),
        ip: Set(input.ip),
        ua: Set(input.ua),
        ..Default::default()
    };

    let model = active.insert(db).await?;
    CommentDto::try_from(model)
}

pub async fn get_comment_by_id<C>(db: &C, id: i64) -> RepoResult<CommentDto>
where
    C: ConnectionTrait,
{
    let model = comments::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    CommentDto::try_from(model)
}

pub async fn list_comments<C>(db: &C) -> RepoResult<Vec<CommentDto>>
where
    C: ConnectionTrait,
{
    let models = comments::Entity::find().all(db).await?;

    models
        .into_iter()
        .map(CommentDto::try_from)
        .collect::<Result<Vec<_>, _>>()
}

pub async fn list_comments_by_post_id<C>(db: &C, post_id: i64) -> RepoResult<Vec<CommentDto>>
where
    C: ConnectionTrait,
{
    let models = comments::Entity::find()
        .filter(comments::Column::PostId.eq(post_id))
        .all(db)
        .await?;

    models
        .into_iter()
        .map(CommentDto::try_from)
        .collect::<Result<Vec<_>, _>>()
}

/// 按条件统计评论数量（SELECT COUNT(*)）
pub async fn count_comments<C>(
    db: &C,
    post_id: Option<i64>,
    status: Option<&str>,
) -> RepoResult<u64>
where
    C: ConnectionTrait,
{
    let mut query = comments::Entity::find();

    if let Some(pid) = post_id {
        query = query.filter(comments::Column::PostId.eq(pid));
    }
    if let Some(s) = status {
        query = query.filter(comments::Column::Status.eq(s));
    }

    let count = query.count(db).await?;
    Ok(count)
}

pub async fn update_comment<C>(db: &C, id: i64, patch: UpdateComment) -> RepoResult<CommentDto>
where
    C: ConnectionTrait,
{
    let model = comments::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    let mut active = model.into_active_model();

    if let Some(v) = patch.content {
        active.content = Set(v);
    }
    if let Some(v) = patch.guest_nick {
        active.guest_nick = Set(v);
    }
    if let Some(v) = patch.guest_email {
        active.guest_email = Set(v);
    }
    if let Some(v) = patch.guest_website {
        active.guest_website = Set(v);
    }
    if let Some(v) = patch.parent_id {
        active.parent_id = Set(v);
    }
    if let Some(v) = patch.root_id {
        active.root_id = Set(v);
    }
    if let Some(v) = patch.status {
        active.status = Set(v.map(|s| s.as_str().to_string()));
    }
    if let Some(v) = patch.ip {
        active.ip = Set(v);
    }
    if let Some(v) = patch.ua {
        active.ua = Set(v);
    }

    let updated = active.update(db).await?;
    CommentDto::try_from(updated)
}

pub async fn delete_comment<C>(db: &C, id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    let res = comments::Entity::delete_by_id(id).exec(db).await?;
    if res.rows_affected == 0 {
        return Err(RepoError::NotFound);
    }
    Ok(())
}
