use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::Deserialize;

use crate::handler::{
    auth::Claims,
    response::{ok, no_content, ApiResponse},
    state::AppState,
};
use crate::service::{self, error::ServiceError, tags::{CreateTagInput, Tag, UpdateTagInput}};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    /// 标签名称
    pub name: String,
    /// 标签 slug
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagRequest {
    /// 标签名称
    pub name: Option<String>,
    /// 标签 slug
    pub slug: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MergeTagsRequest {
    /// 目标标签 ID（保留此标签）
    pub target_id: i64,
    /// 源标签 ID 列表（这些标签将被合并到目标标签，然后删除）
    pub source_ids: Vec<i64>,
}

// ================================
// Handler 实现
// ================================

/// POST /api/admin/tags
///
/// 创建标签
///
/// # 请求体
///
/// ```json
/// {
///   "name": "Rust",
///   "slug": "rust"
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
///     "name": "Rust",
///     "slug": "rust",
///     "post_count": 0,
///     "view_count": 0,
///     "created_at": "2024-01-01T00:00:00Z"
///   }
/// }
/// ```
pub async fn create_tag(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, ServiceError> {
    tracing::info!("Admin {} creating tag: {}", claims.sub, req.slug);

    let input = CreateTagInput {
        name: req.name,
        slug: req.slug,
    };

    let tag = service::tags::create_tag(&state.db, input).await?;
    Ok(ok(tag))
}

/// PUT /api/admin/tags/:id
///
/// 更新标签
///
/// # 路径参数
///
/// - `id`: 标签 ID
///
/// # 请求体
///
/// ```json
/// {
///   "name": "Rust Lang"
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
///     "name": "Rust Lang",
///     "slug": "rust",
///     "post_count": 5,
///     "view_count": 500,
///     "created_at": "2024-01-01T00:00:00Z"
///   }
/// }
/// ```
pub async fn update_tag(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, ServiceError> {
    tracing::info!("Admin {} updating tag: {}", claims.sub, id);

    let input = UpdateTagInput {
        name: req.name,
        slug: req.slug,
    };

    let tag = service::tags::update_tag(&state.db, id, input).await?;
    Ok(ok(tag))
}

/// DELETE /api/admin/tags/:id
///
/// 删除标签
///
/// # 路径参数
///
/// - `id`: 标签 ID
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": null
/// }
/// ```
///
/// # 说明
///
/// - 删除标签会级联删除 post_tags 关联（数据库约束）
pub async fn delete_tag(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError> {
    tracing::info!("Admin {} deleting tag: {}", claims.sub, id);

    service::tags::delete_tag(&state.db, id).await?;
    Ok(no_content())
}

/// POST /api/admin/tags/merge
///
/// 合并标签
///
/// # 请求体
///
/// ```json
/// {
///   "target_id": 1,
///   "source_ids": [2, 3, 4]
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
///     "name": "Rust",
///     "slug": "rust",
///     "post_count": 15,
///     "view_count": 1500,
///     "created_at": "2024-01-01T00:00:00Z"
///   }
/// }
/// ```
///
/// # 说明
///
/// - 将 source_ids 中的所有标签的文章关联转移到 target_id
/// - 删除 source_ids 中的标签
/// - 返回更新后的目标标签
pub async fn merge_tags(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<MergeTagsRequest>,
) -> Result<Json<ApiResponse<Tag>>, ServiceError> {
    tracing::info!(
        "Admin {} merging tags {:?} into {}",
        claims.sub,
        req.source_ids,
        req.target_id
    );

    let tag = service::tags::merge_tags(&state.db, req.target_id, &req.source_ids).await?;
    Ok(ok(tag))
}
