use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::{
    domain::status::NoteStatus,
    entities::notes,
    repo::error::{RepoError, RepoResult},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteDto {
    pub id: i64,
    pub content: String,
    pub mood: Option<String>,
    pub status: Option<NoteStatus>,
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub updated_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}

impl TryFrom<notes::Model> for NoteDto {
    type Error = RepoError;

    fn try_from(model: notes::Model) -> Result<Self, Self::Error> {
        let status = match model.status.as_deref() {
            None => None,
            Some(s) => Some(NoteStatus::try_from(s)?),
        };

        Ok(Self {
            id: model.id,
            content: model.content,
            mood: model.mood,
            status,
            created_at: model.created_at,
            updated_at: model.updated_at,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CreateNote {
    pub content: String,
    pub mood: Option<String>,
    pub status: Option<NoteStatus>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateNote {
    pub content: Option<String>,
    pub mood: Option<Option<String>>,
    pub status: Option<NoteStatus>,
}

pub async fn create_note<C>(db: &C, input: CreateNote) -> RepoResult<NoteDto>
where
    C: ConnectionTrait,
{
    let mut active = notes::ActiveModel {
        content: Set(input.content),
        ..Default::default()
    };

    if let Some(mood) = input.mood {
        active.mood = Set(Some(mood));
    }
    if let Some(status) = input.status {
        active.status = Set(Some(status.as_str().to_string()));
    }

    let model = active.insert(db).await?;
    NoteDto::try_from(model)
}

pub async fn get_note_by_id<C>(db: &C, id: i64) -> RepoResult<NoteDto>
where
    C: ConnectionTrait,
{
    let model = notes::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    NoteDto::try_from(model)
}

pub async fn update_note<C>(db: &C, id: i64, patch: UpdateNote) -> RepoResult<NoteDto>
where
    C: ConnectionTrait,
{
    let model = notes::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(RepoError::NotFound)?;

    let mut active = model.into_active_model();

    if let Some(v) = patch.content {
        active.content = Set(v);
    }
    if let Some(v) = patch.mood {
        active.mood = Set(v);
    }
    if let Some(v) = patch.status {
        active.status = Set(Some(v.as_str().to_string()));
    }

    let updated = active.update(db).await?;
    NoteDto::try_from(updated)
}

pub async fn delete_note<C>(db: &C, id: i64) -> RepoResult<()>
where
    C: ConnectionTrait,
{
    let res = notes::Entity::delete_by_id(id).exec(db).await?;
    if res.rows_affected == 0 {
        return Err(RepoError::NotFound);
    }
    Ok(())
}

/// 按条件筛选随记列表（支持按状态筛选，时间倒序，分页）
pub async fn list_notes_filtered<C>(
    db: &C,
    status: Option<&str>,
    count: u64,
    page: u64,
) -> RepoResult<Vec<NoteDto>>
where
    C: ConnectionTrait,
{
    let mut query = notes::Entity::find();

    if let Some(s) = status {
        query = query.filter(notes::Column::Status.eq(s));
    }

    query = query.order_by_desc(notes::Column::CreatedAt);

    let offset = (page - 1) * count;
    query = query.limit(count).offset(offset);

    let models = query.all(db).await?;
    models
        .into_iter()
        .map(NoteDto::try_from)
        .collect::<Result<Vec<_>, _>>()
}

/// 按条件统计随记数量（SELECT COUNT(*)）
pub async fn count_notes<C>(db: &C, status: Option<&str>) -> RepoResult<u64>
where
    C: ConnectionTrait,
{
    let mut query = notes::Entity::find();

    if let Some(s) = status {
        query = query.filter(notes::Column::Status.eq(s));
    }

    let count = query.count(db).await?;
    Ok(count)
}
