use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::handler::{
    response::{ok, paged, ApiResponse, PagedData},
    state::AppState,
};
use crate::service::{self, error::ServiceError, notes::Note};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Deserialize)]
pub struct ListNotesQuery {
    /// 分页：页码（从 1 开始）
    pub page: Option<u64>,
    /// 分页：每页数量（默认 10，最大 20）
    pub page_size: Option<u64>,
}

// ================================
// Handler 实现
// ================================

/// GET /api/public/notes
///
/// 获取随记列表（分页，仅已发布）
///
/// # 查询参数
///
/// - `page`: 页码（从 1 开始，默认 1）
/// - `page_size`: 每页数量（默认 10，最大 20）
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "items": [
///       {
///         "id": 1,
///         "content": "今天学了 Rust 的 lifetime...",
///         "mood": "thinking",
///         "status": "published",
///         "created_at": "2026-03-10T14:30:00+08:00",
///         "updated_at": "2026-03-10T14:30:00+08:00"
///       }
///     ],
///     "total": 42,
///     "page": 1,
///     "page_size": 10,
///     "total_pages": 5
///   }
/// }
/// ```
pub async fn list_notes(
    State(state): State<AppState>,
    Query(params): Query<ListNotesQuery>,
) -> Result<Json<ApiResponse<PagedData<Note>>>, ServiceError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(10).min(20).max(1);

    let (notes, total) = service::notes::list_public_notes(&state.db, page, page_size).await?;
    Ok(paged(notes, total, page, page_size))
}

/// GET /api/public/notes/:id
///
/// 获取随记详情（仅已发布）
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
pub async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Note>>, ServiceError> {
    let note = service::notes::get_published_note(&state.db, id).await?;
    Ok(ok(note))
}
