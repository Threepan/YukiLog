use axum::{
    extract::{ConnectInfo, Path, Query, State},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;
use std::net::SocketAddr;

use crate::domain::status::PostStatus;
use crate::handler::{
    response::{ok, paged, ApiResponse, PagedData},
    state::AppState,
    utils::{check_rate_limit, get_client_ip},
};
use crate::service::{
    self,
    error::ServiceError,
    posts::{Post, PostFilter, PostSortBy},
};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    /// 分页：页码（从 1 开始）
    pub page: Option<u64>,
    /// 分页：每页数量（默认 10，最大 100）
    pub page_size: Option<u64>,
    /// 排序方式
    pub sort: Option<PostSortBy>,
    /// 筛选：主题 slug（多个用逗号分隔）
    pub theme_slugs: Option<String>,
    /// 筛选：标签 slug（多个标签用逗号分隔，AND 关系）
    pub tag_slugs: Option<String>,
}

// ================================
// Handler 实现
// ================================

/// GET /api/public/posts
///
/// 获取文章列表（分页）
///
/// # 查询参数
///
/// - `page`: 页码（从 1 开始，默认 1）
/// - `page_size`: 每页数量（默认 10，最大 100）
/// - `sort`: 排序方式（view_count | created_at | updated_at）
/// - `theme_slugs`: 按主题筛选（多个用逗号分隔）
/// - `tag_slugs`: 按标签筛选（多个标签用逗号分隔，AND 关系）
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
///         "title": "文章标题",
///         "slug": "article-slug",
///         "summary": "文章摘要",
///         "content": "文章内容",
///         "status": "Published",
///         "theme_id": 1,
///         "view_count": 100,
///         "created_at": "2024-01-01T00:00:00Z",
///         "updated_at": "2024-01-01T00:00:00Z"
///       }
///     ],
///     "total": 100,
///     "page": 1,
///     "page_size": 10,
///     "total_pages": 10
///   }
/// }
/// ```
pub async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<ListPostsQuery>,
) -> Result<Json<ApiResponse<PagedData<Post>>>, ServiceError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(10).min(100).max(1);

    // 解析主题 slug 列表
    let theme_slugs = params
        .theme_slugs
        .as_ref()
        .map(|s| s.split(',').map(|t| t.trim().to_string()).collect::<Vec<_>>());

    // 解析标签 slug 列表
    let tag_slugs = params
        .tag_slugs
        .as_ref()
        .map(|s| s.split(',').map(|t| t.trim().to_string()).collect::<Vec<_>>());

    // 构造过滤器（仅返回已发布的文章）
    let filter = PostFilter {
        theme_slugs,
        tag_slugs,
        status: Some(PostStatus::Published),
        sort_by: params.sort,
        count: Some(page_size),
        page: Some(page),
    };

    // 获取文章列表
    let posts = service::posts::list_posts(&state.db, filter.clone()).await?;

    // 获取总数（SELECT COUNT(*)）
    let count_filter = PostFilter {
        count: None,
        page: None,
        ..filter
    };
    let total = service::posts::count_posts(&state.db, count_filter).await?;

    Ok(paged(posts, total, page, page_size))
}

/// GET /api/public/posts/:slug
///
/// 获取文章详情
///
/// # 路径参数
///
/// - `slug`: 文章 slug
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": {
///     "id": 1,
///     "title": "文章标题",
///     "slug": "article-slug",
///     "summary": "文章摘要",
///     "content": "文章内容",
///     "status": "Published",
///     "theme_id": 1,
///     "view_count": 100,
///     "created_at": "2024-01-01T00:00:00Z",
///     "updated_at": "2024-01-01T00:00:00Z"
///   }
/// }
/// ```
pub async fn get_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Post>>, ServiceError> {
    let post = service::posts::get_post_by_slug(&state.db, &slug).await?;

    // 仅返回已发布的文章
    if post.status != PostStatus::Published {
        return Err(ServiceError::NotFound);
    }

    Ok(ok(post))
}

/// POST /api/public/posts/:slug/view
///
/// 增加文章浏览计数
///
/// # 限流
///
/// - 同一 IP 10 分钟内只计数一次
/// - 静默处理：已访问过返回成功，但不增加计数
///
/// # 路径参数
///
/// - `slug`: 文章 slug
///
/// # 响应
///
/// ```json
/// {
///   "success": true,
///   "data": null
/// }
/// ```
pub async fn increment_post_view(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<()>>, ServiceError> {
    let ip = get_client_ip(&headers, addr);
    let cache_key = format!("view:post:{}:{}", slug, ip);

    // IP 限流检查（10 分钟）
    if !check_rate_limit(&state.redis, &cache_key, 600)
        .await
        .map_err(|e| {
            tracing::error!("Redis error in check_rate_limit: {:?}", e);
            ServiceError::InvalidInput("限流检查失败".to_string())
        })?
    {
        // 已访问过，静默返回成功
        return Ok(ok(()));
    }

    // 获取文章并增加计数
    let post = service::posts::get_post_by_slug(&state.db, &slug).await?;

    // 仅对已发布的文章计数
    if post.status != PostStatus::Published {
        return Err(ServiceError::NotFound);
    }

    service::posts::increment_view_count(&state.db, post.id).await?;

    tracing::debug!("Post {} view incremented for IP: {}", slug, ip);
    Ok(ok(()))
}
