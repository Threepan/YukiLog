use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;

use crate::handler::{
    auth::Claims,
    response::{ok, no_content, ApiResponse},
    state::AppState,
};
use crate::service::{
    self,
    error::ServiceError,
    links::{Link, LinkFilter, LinkSortBy, UpdateLinkInput},
};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Deserialize)]
pub struct ListLinksQuery {
    /// 排序方式
    pub sort: Option<LinkSortBy>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLinkRequest {
    /// 网站名称
    pub title: Option<String>,
    /// 网站 URL
    pub url: Option<String>,
    /// 网站头像
    pub avatar: Option<Option<String>>,
    /// 网站描述
    pub description: Option<Option<String>>,
}

// ================================
// Handler 实现
// ================================

/// GET /api/admin/links
///
/// 获取所有友链
///
/// # 查询参数
///
/// - `sort`: 排序方式
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "id": 1,
///       "title": "友链网站",
///       "url": "https://example.com",
///       "avatar": "https://example.com/avatar.png",
///       "description": "网站描述",
///       "status": "Active",
///       "created_at": "2024-01-01T00:00:00Z"
///     }
///   ]
/// }
/// ```
pub async fn list_links(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<ListLinksQuery>,
) -> Result<Json<ApiResponse<Vec<Link>>>, ServiceError> {
    tracing::debug!("Admin {} listing links", claims.sub);

    let filter = LinkFilter {
        status: None, // 管理员可以看到所有状态
        sort_by: params.sort,
    };

    let links = service::links::list_all_links(&state.db, filter).await?;
    Ok(ok(links))
}

/// GET /api/admin/links/pending
///
/// 获取待审核友链
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": [
///     {
///       "id": 1,
///       "title": "友链网站",
///       "url": "https://example.com",
///       "status": "Pending",
///       ...
///     }
///   ]
/// }
/// ```
pub async fn list_pending_links(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Link>>>, ServiceError> {
    tracing::debug!("Admin {} listing pending links", claims.sub);

    let links = service::links::list_pending_links(&state.db).await?;
    Ok(ok(links))
}

/// PUT /api/admin/links/:id/approve
///
/// 审核通过友链
///
/// # 路径参数
///
/// - `id`: 友链 ID
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "id": 1,
///     "status": "Active",
///     ...
///   }
/// }
/// ```
pub async fn approve_link(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Link>>, ServiceError> {
    tracing::info!("Admin {} approving link: {}", claims.sub, id);

    let link = service::links::approve_link(&state.db, id).await?;
    Ok(ok(link))
}

/// PUT /api/admin/links/:id/broken
///
/// 标记友链失效
///
/// # 路径参数
///
/// - `id`: 友链 ID
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "id": 1,
///     "status": "Broken",
///     ...
///   }
/// }
/// ```
pub async fn mark_link_broken(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Link>>, ServiceError> {
    tracing::info!("Admin {} marking link as broken: {}", claims.sub, id);

    let link = service::links::mark_link_broken(&state.db, id).await?;
    Ok(ok(link))
}

/// PUT /api/admin/links/:id
///
/// 更新友链信息
///
/// # 路径参数
///
/// - `id`: 友链 ID
///
/// # 请求体
///
/// ```json
/// {
///   "title": "新名称",
///   "url": "https://new-url.com",
///   "description": "新描述"
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
///     "title": "新名称",
///     ...
///   }
/// }
/// ```
pub async fn update_link(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateLinkRequest>,
) -> Result<Json<ApiResponse<Link>>, ServiceError> {
    tracing::info!("Admin {} updating link: {}", claims.sub, id);

    let input = UpdateLinkInput {
        title: req.title,
        url: req.url,
        avatar: req.avatar,
        description: req.description,
    };

    let link = service::links::update_link(&state.db, id, input).await?;
    Ok(ok(link))
}

/// DELETE /api/admin/links/:id
///
/// 删除友链
///
/// # 路径参数
///
/// - `id`: 友链 ID
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": null
/// }
/// ```
pub async fn delete_link(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError> {
    tracing::info!("Admin {} deleting link: {}", claims.sub, id);

    service::links::delete_link(&state.db, id).await?;
    Ok(no_content())
}
