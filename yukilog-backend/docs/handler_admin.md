<div align="center">

# YukiLog Handler 层文档

这个文档啊~我们来讲管理接口, 也就是后台接口

</div>

---

## 后台接口

顾名思义, 就是后台操作博客数据和处理前台请求使用的接口

源码: 

* [yukilog-backend/src/handler/admin/themes.rs](../src/handler/admin/themes.rs)
* [yukilog-backend/src/handler/admin/tags.rs](../src/handler/admin/tags.rs)
* [yukilog-backend/src/handler/admin/posts.rs](../src/handler/admin/posts.rs)
* [yukilog-backend/src/handler/admin/comments.rs](../src/handler/admin/comments.rs)
* [yukilog-backend/src/handler/admin/links.rs](../src/handler/admin/links.rs)

**⚠️ 权限要求:**  
所有管理接口都需要 JWT 认证, 通过 `Extension<Claims>` 获取管理员身份

---

## Themes 主题 - 3 个接口

```bash
POST    /api/admin/themes       - 创建主题
PUT     /api/admin/themes/:id   - 更新主题
DELETE  /api/admin/themes/:id   - 删除主题
```

#### 接口定义

```rust
/// 创建主题
pub async fn create_theme(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateThemeRequest>,
) -> Result<Json<ApiResponse<Theme>>, ServiceError>

/// 更新主题
pub async fn update_theme(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateThemeRequest>,
) -> Result<Json<ApiResponse<Theme>>, ServiceError>

/// 删除主题
pub async fn delete_theme(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError>
```

#### 接口 DTO

```rust
pub struct CreateThemeRequest {
    /// 主题名称
    pub name: String,
    /// 主题 slug
    pub slug: String,
    /// 描述
    pub description: Option<String>,
}

pub struct UpdateThemeRequest {
    /// 主题名称
    pub name: Option<String>,
    /// 主题 slug
    pub slug: Option<String>,
    /// 描述
    pub description: Option<Option<String>>,
}
```

---

## Tags 标签 - 4 个接口

```bash
POST    /api/admin/tags         - 创建标签
PUT     /api/admin/tags/:id     - 更新标签
DELETE  /api/admin/tags/:id     - 删除标签
POST    /api/admin/tags/merge   - 合并标签
```

#### 接口定义

```rust
/// 创建标签
pub async fn create_tag(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, ServiceError>

/// 更新标签
pub async fn update_tag(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, ServiceError>

/// 删除标签
pub async fn delete_tag(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError>

/// 合并标签 (将多个标签合并到一个目标标签)
pub async fn merge_tags(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<MergeTagsRequest>,
) -> Result<Json<ApiResponse<Tag>>, ServiceError>
```

#### 接口 DTO

```rust
pub struct CreateTagRequest {
    /// 标签名称
    pub name: String,
    /// 标签 slug
    pub slug: String,
}

pub struct UpdateTagRequest {
    /// 标签名称
    pub name: Option<String>,
    /// 标签 slug
    pub slug: Option<String>,
}

pub struct MergeTagsRequest {
    /// 目标标签 ID（保留此标签）
    pub target_id: i64,
    /// 源标签 ID 列表（这些标签将被合并到目标标签，然后删除）
    pub source_ids: Vec<i64>,
}
```

---

## Posts 文章 - 5 个接口

```bash
GET     /api/admin/posts        - 文章列表 (含草稿, 分页+过滤)
POST    /api/admin/posts        - 创建文章
GET     /api/admin/posts/:slug  - 获取单篇文章 (含草稿)
PUT     /api/admin/posts/:slug  - 更新文章
DELETE  /api/admin/posts/:slug  - 删除文章
```

#### 接口定义

```rust
/// 获取所有文章（含草稿、关联数据）
pub async fn list_posts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<ListPostsQuery>,
) -> Result<Json<ApiResponse<PagedData<PostWithRelations>>>, ServiceError>

/// 获取单篇文章（含草稿、关联数据）
pub async fn get_post(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<PostWithRelations>>, ServiceError>

/// 创建文章
pub async fn create_post(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<Post>>, ServiceError>

/// 更新文章
pub async fn update_post(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<Post>>, ServiceError>

/// 删除文章
pub async fn delete_post(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<()>>, ServiceError>
```

#### 接口 DTO

```rust
pub struct ListPostsQuery {
    /// 分页：页码（从 1 开始）
    pub page: Option<u64>,
    /// 分页：每页数量（默认 10，最大 100）
    pub page_size: Option<u64>,
    /// 排序方式
    pub sort: Option<PostSortBy>,
    /// 筛选：状态 (管理员可查看所有状态)
    pub status: Option<PostStatus>,
    /// 筛选：主题 slug（多个用逗号分隔）
    pub theme_slugs: Option<String>,
    /// 筛选：标签 slug（多个用逗号分隔）
    pub tag_slugs: Option<String>,
}

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
    /// 状态 (Draft/Published)
    pub status: PostStatus,
    /// 主题 slug
    pub theme_slug: Option<String>,
    /// 标签 slug 列表
    pub tag_slugs: Vec<String>,
}

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
```

---

## Comments 评论 - 6 个接口

```bash
GET     /api/admin/comments              - 评论列表 (分页)
GET     /api/admin/comments/pending      - 待审核评论
PUT     /api/admin/comments/:id/approve  - 审核通过
PUT     /api/admin/comments/:id/reject   - 审核拒绝
PUT     /api/admin/comments/:id          - 更新评论
DELETE  /api/admin/comments/:id          - 删除评论
```

#### 接口定义

```rust
/// 获取所有评论（分页）
pub async fn list_comments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<ListCommentsQuery>,
) -> Result<Json<ApiResponse<PagedData<Comment>>>, ServiceError>

/// 获取待审核评论
pub async fn list_pending_comments(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Comment>>>, ServiceError>

/// 审核通过评论
pub async fn approve_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Comment>>, ServiceError>

/// 审核拒绝评论
pub async fn reject_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Comment>>, ServiceError>

/// 更新评论内容
pub async fn update_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateCommentRequest>,
) -> Result<Json<ApiResponse<Comment>>, ServiceError>

/// 删除评论
pub async fn delete_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError>
```

#### 接口 DTO

```rust
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
```

---

## Links 友链 - 6 个接口

```bash
GET     /api/admin/links             - 友链列表 (所有状态)
GET     /api/admin/links/pending     - 待审核友链
PUT     /api/admin/links/:id/approve - 审核通过
PUT     /api/admin/links/:id/broken  - 标记失效
PUT     /api/admin/links/:id         - 更新友链
DELETE  /api/admin/links/:id         - 删除友链
```

#### 接口定义

```rust
/// 获取所有友链
pub async fn list_links(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<ListLinksQuery>,
) -> Result<Json<ApiResponse<Vec<Link>>>, ServiceError>

/// 获取待审核友链
pub async fn list_pending_links(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<Vec<Link>>>, ServiceError>

/// 审核通过友链
pub async fn approve_link(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Link>>, ServiceError>

/// 标记友链失效
pub async fn mark_link_broken(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<Link>>, ServiceError>

/// 更新友链信息
pub async fn update_link(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateLinkRequest>,
) -> Result<Json<ApiResponse<Link>>, ServiceError>

/// 删除友链
pub async fn delete_link(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ServiceError>
```

#### 接口 DTO

```rust
pub struct ListLinksQuery {
    /// 排序方式
    pub sort: Option<LinkSortBy>,
}

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
```

---
