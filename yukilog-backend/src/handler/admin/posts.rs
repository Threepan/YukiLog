use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;

use crate::domain::status::PostStatus;
use crate::handler::{
    auth::Claims,
    response::{ok, no_content, paged, ApiResponse, PagedData},
    state::AppState,
};
use crate::service::{
    self,
    error::ServiceError,
    posts::{CreatePostInput, Post, PostFilter, PostSortBy, UpdatePostInput},
};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    /// 分页：页码
    pub page: Option<u64>,
    /// 分页：每页数量
    pub page_size: Option<u64>,
    /// 排序方式
    pub sort: Option<PostSortBy>,
    /// 筛选：状态
    pub status: Option<PostStatus>,
    /// 筛选：主题 slug（多个用逗号分隔）
    pub theme_slugs: Option<String>,
    /// 筛选：标签 slug（多个用逗号分隔）
    pub tag_slugs: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    /// 文章标题
    pub title: String,
    /// 文章 slug
    pub slug: String,
    /// 文章内容
    pub content: String,
    /// 摘要
    pub summary: Option<String>,
    /// 封面图
    pub cover_image: Option<String>,
    /// 状态
    pub status: PostStatus,
    /// 主题 slug
    pub theme_slug: Option<String>,
    /// 标签 slug 列表
    pub tag_slugs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    /// 文章标题
    pub title: Option<String>,
    /// 文章 slug
    pub slug: Option<String>,
    /// 文章内容
    pub content: Option<String>,
    /// 摘要
    pub summary: Option<Option<String>>,
    /// 封面图
    pub cover_image: Option<Option<String>>,
    /// 状态
    pub status: Option<PostStatus>,
    /// 主题 slug
    pub theme_slug: Option<Option<String>>,
    /// 标签 slug 列表
    pub tag_slugs: Option<Vec<String>>,
}

// ================================
// Handler 实现
// ================================

/// GET /api/admin/posts
///
/// 获取所有文章（含草稿）
///
/// # 查询参数
///
/// - `page`: 页码（默认 1）
/// - `page_size`: 每页数量（默认 10，最大 100）
/// - `sort`: 排序方式
/// - `status`: 状态筛选
/// - `theme_slugs`: 主题筛选
/// - `tag_slugs`: 标签筛选
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
pub async fn list_posts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<ListPostsQuery>,
) -> Result<Json<ApiResponse<PagedData<Post>>>, ServiceError> {
    tracing::debug!("Admin {} listing posts", claims.sub);

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

    // 构造过滤器（管理员可以看到所有状态）
    let filter = PostFilter {
        theme_slugs,
        tag_slugs,
        status: params.status,
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

/// POST /api/admin/posts
///
/// 创建文章
///
/// # 请求体
///
/// ```json
/// {
///   "title": "文章标题",
///   "slug": "article-slug",
///   "content": "文章内容",
///   "summary": "文章摘要",
///   "cover_image": "https://example.com/cover.jpg",
///   "status": "Draft",
///   "theme_slug": "tech",
///   "tag_slugs": ["rust", "backend"]
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
///     "title": "文章标题",
///     "slug": "article-slug",
///     ...
///   }
/// }
/// ```
pub async fn create_post(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<Post>>, ServiceError> {
    tracing::info!("Admin {} creating post: {}", claims.sub, req.slug);

    let input = CreatePostInput {
        title: req.title,
        slug: req.slug,
        content: req.content,
        summary: req.summary,
        cover_image: req.cover_image,
        status: req.status,
        theme_slug: req.theme_slug,
        tag_slugs: req.tag_slugs,
    };

    let post = service::posts::create_post(&state.db, input).await?;
    Ok(ok(post))
}

/// PUT /api/admin/posts/:slug
///
/// 更新文章
///
/// # 路径参数
///
/// - `slug`: 文章 slug
///
/// # 请求体
///
/// ```json
/// {
///   "title": "新标题",
///   "status": "Published",
///   "tag_slugs": ["rust", "web"]
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
///     "title": "新标题",
///     ...
///   }
/// }
/// ```
pub async fn update_post(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<Post>>, ServiceError> {
    tracing::info!("Admin {} updating post: {}", claims.sub, slug);

    let input = UpdatePostInput {
        title: req.title,
        slug: req.slug,
        content: req.content,
        summary: req.summary,
        cover_image: req.cover_image,
        status: req.status,
        theme_slug: req.theme_slug,
        tag_slugs: req.tag_slugs,
    };

    let post = service::posts::update_post(&state.db, &slug, input).await?;
    Ok(ok(post))
}

/// DELETE /api/admin/posts/:slug
///
/// 删除文章
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
pub async fn delete_post(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<()>>, ServiceError> {
    tracing::info!("Admin {} deleting post: {}", claims.sub, slug);

    service::posts::delete_post(&state.db, &slug).await?;
    Ok(no_content())
}
