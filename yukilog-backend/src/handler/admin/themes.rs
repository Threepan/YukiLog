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
use crate::service::{self, error::ServiceError, themes::{CreateThemeInput, Theme, UpdateThemeInput}};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Deserialize)]
pub struct CreateThemeRequest {
    /// 主题名称
    pub name: String,
    /// 主题 slug
    pub slug: String,
    /// 描述
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateThemeRequest {
    /// 主题名称
    pub name: Option<String>,
    /// 主题 slug
    pub slug: Option<String>,
    /// 描述
    pub description: Option<Option<String>>,
}

// ================================
// Handler 实现
// ================================

/// POST /api/admin/themes
///
/// 创建主题
///
/// # 请求体
///
/// ```json
/// {
///   "name": "技术",
///   "slug": "tech",
///   "description": "技术相关文章"
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
///     "name": "技术",
///     "slug": "tech",
///     "description": "技术相关文章",
///     "post_count": 0,
///     "view_count": 0,
///     "created_at": "2024-01-01T00:00:00Z"
///   }
/// }
/// ```
pub async fn create_theme(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateThemeRequest>,
) -> Result<Json<ApiResponse<Theme>>, ServiceError> {
    tracing::info!("Admin {} creating theme: {}", claims.sub, req.slug);

    let input = CreateThemeInput {
        name: req.name,
        slug: req.slug,
        description: req.description,
    };

    let theme = service::themes::create_theme(&state.db, input).await?;
    Ok(ok(theme))
}

/// PUT /api/admin/themes/:id
///
/// 更新主题
///
/// # 路径参数
///
/// - `id`: 主题 ID
///
/// # 请求体
///
/// ```json
/// {
///   "name": "技术分享",
///   "description": "更新后的描述"
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
///     "name": "技术分享",
///     "slug": "tech",
///     "description": "更新后的描述",
///     "post_count": 10,
///     "view_count": 1000,
///     "created_at": "2024-01-01T00:00:00Z"
///   }
/// }
/// ```
pub async fn update_theme(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateThemeRequest>,
) -> Result<Json<ApiResponse<Theme>>, ServiceError> {
    tracing::info!("Admin {} updating theme: {}", claims.sub, id);

    let input = UpdateThemeInput {
        name: req.name,
        slug: req.slug,
        description: req.description,
    };

    let theme = service::themes::update_theme(&state.db, id, input).await?;
    Ok(ok(theme))
}

/// DELETE /api/admin/themes/:id
///
/// 删除主题
///
/// # 路径参数
///
/// - `id`: 主题 ID
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
/// - 删除主题前会检查是否有关联的文章
/// - 如果有关联文章，将返回错误
pub async fn delete_theme(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError> {
    tracing::info!("Admin {} deleting theme: {}", claims.sub, id);

    service::themes::delete_theme(&state.db, id).await?;
    Ok(no_content())
}
