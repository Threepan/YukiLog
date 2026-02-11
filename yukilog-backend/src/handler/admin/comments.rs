use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;

use crate::handler::{
    auth::Claims,
    response::{ok, no_content, paged, ApiResponse, PagedData},
    state::AppState,
};
use crate::service::{
    self,
    comments::{AdminCommentFilter, Comment, CommentSortBy, UpdateCommentInput},
    error::ServiceError,
};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Deserialize)]
pub struct ListCommentsQuery {
    /// 分页：页码
    pub page: Option<u64>,
    /// 分页：每页数量
    pub page_size: Option<u64>,
    /// 排序方式
    pub sort: Option<CommentSortBy>,
    /// 筛选：文章 slug
    pub post_slug: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    /// 评论内容
    pub content: Option<String>,
    /// 评论者昵称
    pub guest_nick: Option<String>,
    /// 评论者邮箱
    pub guest_email: Option<Option<String>>,
    /// 评论者网站
    pub guest_website: Option<Option<String>>,
}

// ================================
// Handler 实现
// ================================

/// GET /api/admin/comments
///
/// 获取所有评论（分页）
///
/// # 查询参数
///
/// - `page`: 页码（默认 1）
/// - `page_size`: 每页数量（默认 10，最大 100）
/// - `sort`: 排序方式
/// - `post_slug`: 文章 slug 筛选
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "items": [...],
///     "total": 100,
///     "page": 1,
///     "page_size": 10,
///     "total_pages": 10
///   }
/// }
/// ```
pub async fn list_comments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<ListCommentsQuery>,
) -> Result<Json<ApiResponse<PagedData<Comment>>>, ServiceError> {
    tracing::debug!("Admin {} listing comments", claims.sub);

    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(10).min(100).max(1);

    let post_id = match params.post_slug.as_deref() {
        Some(slug) => Some(service::posts::get_post_by_slug(&state.db, slug).await?.id),
        None => None,
    };

    let filter = AdminCommentFilter {
        post_id,
        status: None, // 管理员可以看到所有状态
        sort_by: params.sort,
        count: Some(page_size),
        page: Some(page),
    };

    // 获取评论列表
    let comments = service::comments::list_all_comments(&state.db, filter.clone()).await?;

    // 获取总数（SELECT COUNT(*)）
    let count_filter = AdminCommentFilter {
        count: None,
        page: None,
        ..filter
    };
    let total = service::comments::count_all_comments(&state.db, count_filter).await?;

    Ok(paged(comments, total, page, page_size))
}

/// GET /api/admin/comments/pending
///
/// 获取待审核评论
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "id": 1,
///       "post_id": 1,
///       "content": "评论内容",
///       "guest_nick": "张三",
///       "status": "Pending",
///       ...
///     }
///   ]
/// }
/// ```
pub async fn list_pending_comments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Comment>>>, ServiceError> {
    tracing::debug!("Admin {} listing pending comments", claims.sub);

    let filter = AdminCommentFilter {
        post_id: None,
        status: Some(crate::domain::status::CommentStatus::Pending),
        sort_by: Some(CommentSortBy::CreatedAtAsc),
        count: None,
        page: None,
    };

    let comments = service::comments::list_all_comments(&state.db, filter).await?;
    Ok(ok(comments))
}

/// PUT /api/admin/comments/:id/approve
///
/// 审核通过评论
///
/// # 路径参数
///
/// - `id`: 评论 ID
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "id": 1,
///     "status": "Approved",
///     ...
///   }
/// }
/// ```
pub async fn approve_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Comment>>, ServiceError> {
    tracing::info!("Admin {} approving comment: {}", claims.sub, id);

    let comment = service::comments::approve_comment(&state.db, id).await?;
    Ok(ok(comment))
}

/// PUT /api/admin/comments/:id/reject
///
/// 审核拒绝评论
///
/// # 路径参数
///
/// - `id`: 评论 ID
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "id": 1,
///     "status": "Rejected",
///     ...
///   }
/// }
/// ```
pub async fn reject_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Comment>>, ServiceError> {
    tracing::info!("Admin {} rejecting comment: {}", claims.sub, id);

    let comment = service::comments::reject_comment(&state.db, id).await?;
    Ok(ok(comment))
}

/// PUT /api/admin/comments/:id
///
/// 更新评论内容
///
/// # 路径参数
///
/// - `id`: 评论 ID
///
/// # 请求体
///
/// ```json
/// {
///   "content": "修改后的评论内容",
///   "guest_nick": "新昵称"
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
///     "content": "修改后的评论内容",
///     ...
///   }
/// }
/// ```
pub async fn update_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateCommentRequest>,
) -> Result<Json<ApiResponse<Comment>>, ServiceError> {
    tracing::info!("Admin {} updating comment: {}", claims.sub, id);

    let input = UpdateCommentInput {
        content: req.content,
        guest_nick: req.guest_nick,
        guest_email: req.guest_email,
        guest_website: req.guest_website,
    };

    let comment = service::comments::update_comment(&state.db, id, input).await?;
    Ok(ok(comment))
}

/// DELETE /api/admin/comments/:id
///
/// 删除评论
///
/// # 路径参数
///
/// - `id`: 评论 ID
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": null
/// }
/// ```
pub async fn delete_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError> {
    tracing::info!("Admin {} deleting comment: {}", claims.sub, id);

    service::comments::delete_comment(&state.db, id).await?;
    Ok(no_content())
}
