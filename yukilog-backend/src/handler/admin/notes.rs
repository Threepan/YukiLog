use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;

use crate::domain::status::NoteStatus;
use crate::handler::{
    auth::Claims,
    response::{no_content, ok, paged, ApiResponse, PagedData},
    state::AppState,
};
use crate::service::{
    self,
    error::ServiceError,
    notes::{CreateNoteInput, Note, UpdateNoteInput},
};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Deserialize)]
pub struct AdminListNotesQuery {
    /// 分页：页码（从 1 开始）
    pub page: Option<u64>,
    /// 分页：每页数量（默认 10）
    pub page_size: Option<u64>,
    /// 筛选：状态（published / draft / private）
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    /// Markdown 内容
    pub content: String,
    /// 心情标记（可选）
    pub mood: Option<String>,
    /// 状态（可选，默认 published）
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    /// Markdown 内容
    pub content: Option<String>,
    /// 心情标记（None=不改, null=清空, "happy"=设置）
    pub mood: Option<Option<String>>,
    /// 状态
    pub status: Option<String>,
}

// ================================
// 辅助函数
// ================================

fn parse_note_status(s: &str) -> Result<NoteStatus, ServiceError> {
    NoteStatus::try_from(s).map_err(|_| {
        ServiceError::InvalidInput(format!(
            "invalid status: '{}', expected: published / draft / private",
            s
        ))
    })
}

// ================================
// Handler 实现
// ================================

/// GET /api/admin/notes
///
/// 获取随记列表（管理端，含 draft/private）
///
/// # 查询参数
///
/// - `page`: 页码（从 1 开始，默认 1）
/// - `page_size`: 每页数量（默认 10）
/// - `status`: 状态筛选（published / draft / private，不传则全部）
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "items": [...],
///     "total": 42,
///     "page": 1,
///     "page_size": 10,
///     "total_pages": 5
///   }
/// }
/// ```
pub async fn list_notes(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<AdminListNotesQuery>,
) -> Result<Json<ApiResponse<PagedData<Note>>>, ServiceError> {
    tracing::debug!("Admin {} listing notes", claims.sub);

    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(10).max(1);

    let status = params
        .status
        .as_deref()
        .map(parse_note_status)
        .transpose()?;

    let (notes, total) = service::notes::list_all_notes(&state.db, status, page, page_size).await?;
    Ok(paged(notes, total, page, page_size))
}

/// POST /api/admin/notes
///
/// 创建随记
///
/// # 请求体
///
/// ```json
/// {
///   "content": "今天学了 Rust 的 lifetime...",
///   "mood": "thinking",
///   "status": "published"
/// }
/// ```
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "id": 1,
///     "content": "今天学了 Rust 的 lifetime...",
///     "mood": "thinking",
///     "status": "published",
///     "created_at": "2026-03-10T14:30:00+08:00",
///     "updated_at": "2026-03-10T14:30:00+08:00"
///   }
/// }
/// ```
pub async fn create_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateNoteRequest>,
) -> Result<Json<ApiResponse<Note>>, ServiceError> {
    tracing::info!("Admin {} creating note", claims.sub);

    let status = req.status.as_deref().map(parse_note_status).transpose()?;

    let input = CreateNoteInput {
        content: req.content,
        mood: req.mood,
        status,
    };

    let note = service::notes::create_note(&state.db, input).await?;
    Ok(ok(note))
}

/// PUT /api/admin/notes/:id
///
/// 更新随记
///
/// # 路径参数
///
/// - `id`: 随记 ID
///
/// # 请求体
///
/// ```json
/// {
///   "content": "修改后的内容",
///   "mood": "happy",
///   "status": "draft"
/// }
/// ```
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "id": 1,
///     "content": "修改后的内容",
///     "mood": "happy",
///     "status": "draft",
///     ...
///   }
/// }
/// ```
pub async fn update_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateNoteRequest>,
) -> Result<Json<ApiResponse<Note>>, ServiceError> {
    tracing::info!("Admin {} updating note: {}", claims.sub, id);

    let status = req.status.as_deref().map(parse_note_status).transpose()?;

    let input = UpdateNoteInput {
        content: req.content,
        mood: req.mood,
        status,
    };

    let note = service::notes::update_note(&state.db, id, input).await?;
    Ok(ok(note))
}

/// DELETE /api/admin/notes/:id
///
/// 删除随记
///
/// # 路径参数
///
/// - `id`: 随记 ID
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": null
/// }
/// ```
pub async fn delete_note(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError> {
    tracing::info!("Admin {} deleting note: {}", claims.sub, id);

    service::notes::delete_note(&state.db, id).await?;
    Ok(no_content())
}
