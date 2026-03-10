use sea_orm::DatabaseConnection;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::domain::status::{NoteMood, NoteStatus};
use crate::repo;
use crate::repo::notes::{CreateNote as RepoCreateNote, UpdateNote as RepoUpdateNote};
use crate::service::error::{ServiceError, ServiceResult};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub content: String,
    pub mood: Option<NoteMood>,
    pub status: NoteStatus,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl From<repo::notes::NoteDto> for Note {
    fn from(dto: repo::notes::NoteDto) -> Self {
        Self {
            id: dto.id,
            content: dto.content,
            mood: dto.mood,
            status: dto.status.unwrap_or(NoteStatus::Published),
            created_at: dto.created_at.unwrap_or_else(|| chrono::Utc::now().into()),
            updated_at: dto.updated_at.unwrap_or_else(|| chrono::Utc::now().into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoteInput {
    pub content: String,
    pub mood: Option<NoteMood>,
    pub status: Option<NoteStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateNoteInput {
    pub content: Option<String>,
    pub mood: Option<Option<NoteMood>>,
    pub status: Option<NoteStatus>,
}

// ================================
// 业务逻辑
// ================================

/// 1. 创建随记（管理后台）
///
/// 逻辑：
/// - 校验 content 不为空白
/// - status 不传则走 DB 默认 published
pub async fn create_note(
    db: &DatabaseConnection,
    input: CreateNoteInput,
) -> ServiceResult<Note> {
    if input.content.trim().is_empty() {
        return Err(ServiceError::InvalidInput(
            "content must not be empty".to_string(),
        ));
    }

    let create_input = RepoCreateNote {
        content: input.content,
        mood: input.mood,
        status: input.status,
    };
    let dto = repo::notes::create_note(db, create_input).await?;
    Ok(dto.into())
}

/// 2. 获取已发布随记详情（前台）
///
/// 校验 status = published，否则返回 NotFound
pub async fn get_published_note(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Note> {
    let dto = repo::notes::get_note_by_id(db, id).await?;
    let note: Note = dto.into();
    if note.status != NoteStatus::Published {
        return Err(ServiceError::NotFound);
    }
    Ok(note)
}

/// 3. 获取随记详情（管理后台，不限状态）
pub async fn get_note(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Note> {
    let dto = repo::notes::get_note_by_id(db, id).await?;
    Ok(dto.into())
}

/// 4. 公开端分页列表（仅 published，时间倒序）
///
/// 返回 (数据, 总数) 元组，用于分页
pub async fn list_public_notes(
    db: &DatabaseConnection,
    page: u64,
    page_size: u64,
) -> ServiceResult<(Vec<Note>, u64)> {
    let status = NoteStatus::Published.as_str();
    let total = repo::notes::count_notes(db, Some(status)).await?;
    let dtos = repo::notes::list_notes_filtered(db, Some(status), page_size, page).await?;
    let notes = dtos.into_iter().map(Into::into).collect();
    Ok((notes, total))
}

/// 5. 管理端分页列表（可按状态筛选）
///
/// 返回 (数据, 总数) 元组，用于分页
pub async fn list_all_notes(
    db: &DatabaseConnection,
    status: Option<NoteStatus>,
    page: u64,
    page_size: u64,
) -> ServiceResult<(Vec<Note>, u64)> {
    let status_str = status.as_ref().map(|s| s.as_str());
    let total = repo::notes::count_notes(db, status_str).await?;
    let dtos = repo::notes::list_notes_filtered(db, status_str, page_size, page).await?;
    let notes = dtos.into_iter().map(Into::into).collect();
    Ok((notes, total))
}

/// 6. 更新随记（管理后台）
///
/// 如果传了 content，校验不为空白
pub async fn update_note(
    db: &DatabaseConnection,
    id: i64,
    input: UpdateNoteInput,
) -> ServiceResult<Note> {
    if let Some(ref content) = input.content {
        if content.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "content must not be empty".to_string(),
            ));
        }
    }

    let update_input = RepoUpdateNote {
        content: input.content,
        mood: input.mood,
        status: input.status,
    };
    let dto = repo::notes::update_note(db, id, update_input).await?;
    Ok(dto.into())
}

/// 7. 删除随记（管理后台）
pub async fn delete_note(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<()> {
    repo::notes::delete_note(db, id).await?;
    Ok(())
}

/// 8. 统计公开随记总数
pub async fn count_public_notes(
    db: &DatabaseConnection,
) -> ServiceResult<u64> {
    let count = repo::notes::count_notes(db, Some(NoteStatus::Published.as_str())).await?;
    Ok(count)
}

/// 9. 统计随记总数（管理端，可按状态筛选）
pub async fn count_all_notes(
    db: &DatabaseConnection,
    status: Option<NoteStatus>,
) -> ServiceResult<u64> {
    let status_str = status.as_ref().map(|s| s.as_str());
    let count = repo::notes::count_notes(db, status_str).await?;
    Ok(count)
}
