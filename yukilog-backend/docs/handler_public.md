<div align="center">

# YukiLog Handler 层文档

这个文档啊~我们来讲公共接口, 也就是前台接口

</div>

## 技术架构

---

#### Redis 限流系统

使用 Redis 实现 IP 限流和访问控制, 防止恶意刷访问量和评论灌水

源码: [yukilog-backend/src/handler/utils.rs]

#### IP 提取

```rust
pub fn get_client_ip(headers: &HeaderMap, addr: SocketAddr) -> String
```

**优先级:**
1. `X-Forwarded-For` header (Nginx/Cloudflare 反向代理)
2. `X-Real-IP` header (Nginx)
3. 连接 IP (直连)

#### 限流检查

```rust
pub async fn check_rate_limit(
    redis: &redis::Client,
    cache_key: &str,
    ttl: u64,
) -> Result<bool, redis::RedisError>
```

**返回值:**
- `Ok(true)` - 允许访问
- `Ok(false)` - 限流中

---

#### Gravatar 生成

```rust
pub fn generate_gravatar_url(email: &str) -> String
```

**特性:**
- MD5 哈希计算
- 大小写不敏感
- 默认头像: `d=identicon` (几何图案)
- 尺寸: 80x80 像素

**其他默认头像选项:**
- `monsterid` - 小怪物
- `wavatar` - 卡通脸
- `retro` - 8位像素
- `robohash` - 机器人

---

## 前台接口

顾名思义, 就是前端渲染博客页面和处理公共逻辑使用的接口

源码: 

* [yukilog-backend/src/handler/public/themes.rs](../src/handler/public/themes.rs)
* [yukilog-backend/src/handler/public/tags.rs](../src/handler/public/tags.rs)
* [yukilog-backend/src/handler/public/posts.rs](../src/handler/public/posts.rs)
* [yukilog-backend/src/handler/public/comments.rs](../src/handler/public/comments.rs)
* [yukilog-backend/src/handler/public/links.rs](../src/handler/public/links.rs)

---

## Themes 主题 - 3 个接口

```bash
GET     /api/public/themes              - 主题列表
GET     /api/public/themes/:slug        - 主题详情
POST    /api/public/themes/:slug/view   - 浏览记数 (IP限流 10分钟)
```

#### 接口定义

```rust
/// 获取所有主题列表
pub async fn list_themes(
    State(state): State<AppState>,
    Query(params): Query<ListThemesQuery>,
) -> Result<Json<ApiResponse<Vec<Theme>>>, ServiceError> 

/// 获取主题详情
pub async fn get_theme(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Theme>>, ServiceError>

/// 增加主题浏览计数
pub async fn increment_theme_view(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<()>>, ServiceError>
```

#### 接口 DTO

```rust
pub struct ListThemesQuery {
    /// 排序方式
    pub sort: Option<ThemeSortBy>,
}
```

## Tags 标签 - 3 个接口

```bash
GET    /api/public/tags             - 标签列表/云
GET    /api/public/tags/:slug       - 标签详情  
POST   /api/public/tags/:slug/view  - 浏览计数 (IP限流 10分钟)
```

#### 接口定义

```rust
/// 获取所有标签列表
pub async fn list_tags(
    State(state): State<AppState>,
    Query(params): Query<ListTagsQuery>,
) -> Result<Json<ApiResponse<Vec<Tag>>>, ServiceError>

/// 获取标签详情
pub async fn get_tag(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Tag>>, ServiceError>

/// 增加标签浏览计数
pub async fn increment_tag_view(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<()>>, ServiceError>
```

#### 接口 DTO

```rust
pub struct ListTagsQuery {
    /// 排序方式
    pub sort: Option<TagSortBy>,
}
```

## Posts 文章 - 3 个接口

```bash
GET    /api/public/posts            - 文章列表 (分页+过滤)
GET    /api/public/posts/:slug      - 文章详情
POST   /api/public/posts/:slug/view - 浏览计数 (IP限流 10分钟)
```

#### 接口定义

```rust
/// 获取文章列表（分页）
pub async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<ListPostsQuery>,
) -> Result<Json<ApiResponse<PagedData<PostWithRelations>>>, ServiceError>

/// 获取文章详情
pub async fn get_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<PostWithRelations>>, ServiceError>

/// 增加文章浏览计数
pub async fn increment_post_view(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
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
    /// 筛选：主题 slug（多个用逗号分隔）
    pub theme_slugs: Option<String>,
    /// 筛选：标签 slug（多个标签用逗号分隔，AND 关系）
    pub tag_slugs: Option<String>,
}
```

## Comments 评论 - 3个接口

```bash
GET    /api/public/posts/:slug/comments        - 评论树
GET    /api/public/posts/:slug/comments/:id   - 懒加载回复
POST   /api/public/posts/:slug/comments        - 发表评论 (频率限制 10秒 + Gravatar)
```

#### 接口定义

```rust
/// 获取文章评论树（仅返回顶层评论和直接回复）
pub async fn get_post_comments(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<Vec<CommentNode>>>, ServiceError>

/// 获取评论的所有回复（延迟加载）
pub async fn get_comment_replies(
    State(state): State<AppState>,
    Path((_slug, id)): Path<(String, i64)>,
) -> Result<Json<ApiResponse<Vec<Comment>>>, ServiceError>

/// 创建评论
pub async fn create_comment(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Path(slug): Path<String>,
    Json(req): Json<CreateCommentRequest>,
) -> Result<Json<ApiResponse<CreateCommentResponse>>, ServiceError>
```

#### 接口 DTO

```rust
pub struct CreateCommentRequest {
    /// 评论者昵称
    pub nickname: String,
    /// 评论者邮箱
    pub email: String,
    /// 评论内容
    pub content: String,
    /// 父评论 ID（顶层评论为 null）
    pub parent_id: Option<i64>,
    /// 网站 URL（可选）
    pub website: Option<String>,
}

pub struct CreateCommentResponse {
    /// 评论 ID
    pub id: i64,
    /// Gravatar URL
    pub avatar_url: String,
    /// 创建时间
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
}
```

## Links 友链 - 2 个接口

```bash
GET    /api/public/links            - 友链列表 (仅 active)
POST   /api/public/links/submit       - 申请友链
```

#### 接口定义

```rust
/// 获取活跃友链列表
pub async fn list_links(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Link>>>, ServiceError>

/// 提交友链申请
pub async fn submit_link(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(req): Json<SubmitLinkRequest>,
) -> Result<Json<ApiResponse<SubmitLinkResponse>>, ServiceError>
```

#### 接口 DTO

```rust
pub struct SubmitLinkRequest {
    /// 网站名称
    pub title: String,
    /// 网站 URL
    pub url: String,
    /// 网站头像 URL（可选）
    pub avatar: Option<String>,
    /// 网站描述
    pub description: Option<String>,
}

pub struct SubmitLinkResponse {
    /// 友链 ID
    pub id: i64,
    /// 提示信息
    pub message: String,
}
```
