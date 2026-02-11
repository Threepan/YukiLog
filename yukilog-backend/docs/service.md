<div align="center">

# YukiLog Service 层文档

这一层就要做实际的业务逻辑啦~ 比如创建一篇文章, 审批一条友链~

</div>

---

## | 需求分析

哎, 既然都扯到 "业务" 这个词了, 那就不得不开始做 "需求分析" ...呜 ...

总而言之就是, 把 "我需要什么功能" 翻译为 "后端要做什么接口"

---

## 错误定义

源码: [yukilog-backend/src/service/error.rs](../src/service/error.rs)

这部分简直太简单了, 所以直接给个示例代码吧

```rust
pub enum ServiceError {
    #[error("repository error: {0}")]
    Repo(#[from] RepoError),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("not found")]
    NotFound,
}
```

---

#### theme - 主题

源码: [yukilog-backend/src/service/themes.rs](../src/service/themes.rs)

###### 接口定义

```rust
// src/service/themes.rs

/// 1. 创建主题（管理后台）
pub async fn create_theme(
    db: &DatabaseConnection,
    input: CreateThemeInput,
) -> ServiceResult<Theme>

/// 2. 获取主题详情（前台/后台）
pub async fn get_theme_by_slug<C: ConnectionTrait>(
    db: &C,
    slug: &str,
) -> ServiceResult<Theme>

/// 3. 列出所有主题（前台导航/后台列表）
pub async fn list_all_themes(
    db: &DatabaseConnection,
    sort_by: Option<ThemeSortBy>,
) -> ServiceResult<Vec<Theme>>

/// 4. 更新主题信息（管理后台，允许修改 slug）
pub async fn update_theme(
    db: &DatabaseConnection,
    id: i64,
    input: UpdateThemeInput,
) -> ServiceResult<Theme>

/// 5. 删除主题（管理后台）
/// 数据库已设 ON DELETE SET NULL，直接删即可
pub async fn delete_theme(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<()>

/// 6. 增加浏览计数（前台访问主题页时调用）
/// IP 去重逻辑在 handler 层处理（如 10 分钟内同一 IP 只触发一次）
pub async fn increment_view_count<C: ConnectionTrait>(
    db: &C,
    theme_id: i64,
) -> ServiceResult<()>

/// 7. 调整文章计数（给 post service 调用）
/// delta: +1 表示新文章绑定此主题，-1 表示文章解绑/删除
pub async fn adjust_post_count<C: ConnectionTrait>(
    db: &C,
    theme_id: i64,
    delta: i32,
) -> ServiceResult<()>

// ================================
// 辅助函数（给其他 service 调用）
// ================================

/// 通过 ID 获取主题
pub async fn get_theme_by_id<C: ConnectionTrait>(
    db: &C,
    theme_id: i64,
) -> ServiceResult<Theme>

/// 通过多个 slug 批量获取主题 ID
pub async fn get_theme_ids_by_slugs<C: ConnectionTrait>(
    db: &C,
    slugs: &[String],
) -> ServiceResult<Vec<i64>>
```

###### DTO定义

```rust
// 排序
pub enum ThemeSortBy {
    PostCount,   // 按文章数倒序（最热门）
    ViewCount,   // 按浏览量倒序（最受欢迎）
    CreatedAt,   // 按创建时间倒序（最新）
}

// 输入
pub struct CreateThemeInput {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

pub struct UpdateThemeInput {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<Option<String>>,
}

// 输出
pub struct Theme {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub post_count: i32,
    pub view_count: i64,
    pub created_at: DateTime<FixedOffset>,
}
```

###### 业务逻辑

| 接口名 | 职责 |
|-|-|
| `create_theme` | slug 格式校验: 字母数字-下划线-连字符 <br> 唯一性由 DB 约束保证 |
| `get_theme_by_slug` | 直接查询 |
| `list_all_themes` | 按 `post_count` `view_count` `created_at` 排序 |
| `update_theme` | 校验新 slug 格式 <br> 唯一性冲突由 DB 返回错误 |
| `delete_theme` | 直接删除 <br> posts.theme_id 会被 DB 置 NULL |
| `increment_view_count` | `UPDATE themes SET view_count = view_count + 1 WHERE id = ?` |
| `adjust_post_count` | `UPDATE themes SET post_count = post_count + ? WHERE id = ?` |
| `get_theme_by_id` | 辅助函数：通过 ID 查询主题（给 post service 调用）|
| `get_theme_ids_by_slugs` | 辅助函数：批量获取主题 ID（给 post service 筛选使用）|

---

#### tag - 标签

源码: [yukilog-backend/src/service/tags.rs](../src/service/tags.rs)

###### 接口定义

```rust
// src/service/tags.rs

/// 1. 创建标签（管理后台）
pub async fn create_tag<C: ConnectionTrait>(
    db: &C,
    input: CreateTagInput,
) -> ServiceResult<Tag>

/// 2. 获取或创建标签（发布文章时调用）
/// 如果 slug 已存在则返回现有标签，否则创建新标签
/// 注意：如果标签已存在，不会覆盖现有 name
pub async fn get_or_create_tag<C: ConnectionTrait>(
    db: &C,
    name: &str,
    slug: &str,
) -> ServiceResult<Tag>

/// 3. 获取标签详情
pub async fn get_tag_by_slug(
    db: &DatabaseConnection,
    slug: &str,
) -> ServiceResult<Tag>

/// 4. 列出所有标签（前台标签云/后台列表）
/// count: 每页数量，page: 页码（从 1 开始）
pub async fn list_all_tags(
    db: &DatabaseConnection,
    sort_by: Option<TagSortBy>,
    count: Option<u64>,
    page: Option<u64>,
) -> ServiceResult<Vec<Tag>>

/// 5. 更新标签信息（管理后台，允许修改 slug）
pub async fn update_tag(
    db: &DatabaseConnection,
    id: i64,
    input: UpdateTagInput,
) -> ServiceResult<Tag>

/// 6. 删除标签（管理后台）
/// 数据库已设 ON DELETE CASCADE，会自动删除 post_tags 关联
pub async fn delete_tag(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<()>

/// 7. 合并标签（管理后台）
/// 将 source_ids 的所有文章关联迁移到 target_id，然后删除 source_ids
pub async fn merge_tags(
    db: &DatabaseConnection,
    target_id: i64,
    source_ids: &[i64],
) -> ServiceResult<Tag>

/// 8. 增加浏览计数（前台访问标签页时调用）
pub async fn increment_view_count<C: ConnectionTrait>(
    db: &C,
    tag_id: i64,
) -> ServiceResult<()>

/// 9. 调整文章计数（给 post service 调用）
/// delta: +1 表示文章新增此标签，-1 表示文章移除此标签
pub async fn adjust_post_count<C: ConnectionTrait>(
    db: &C,
    tag_id: i64,
    delta: i32,
) -> ServiceResult<()>

// ================================
// 辅助函数（给其他 service 调用）
// ================================

/// 通过多个 slug 批量获取标签 ID
pub async fn get_tag_ids_by_slugs<C: ConnectionTrait>(
    db: &C,
    slugs: &[String],
) -> ServiceResult<Vec<i64>>
```

###### DTO定义

```rust
// 排序
pub enum TagSortBy {
    PostCount,   // 按文章数倒序（最热门标签）
    ViewCount,   // 按浏览量倒序
    CreatedAt,   // 按创建时间倒序（最新标签）
    Name,        // 按名称字母序（标签云常用）
}

// 输入
pub struct CreateTagInput {
    pub name: String,
    pub slug: String,
}

pub struct UpdateTagInput {
    pub name: Option<String>,
    pub slug: Option<String>,
}

// 输出
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub post_count: i32,
    pub view_count: i64,
    pub created_at: DateTime<FixedOffset>,
}
```

###### 业务逻辑

| 接口名 | 职责 |
|-|-|
| `create_tag` | slug 格式校验: 字母数字-下划线连字符 <br> 唯一性由 DB 约束保证 |
| `get_or_create_tag` | 先查询，不存在则创建 <br> 已存在时不覆盖 name |
| `get_tag_by_slug` | 直接查询 |
| `list_all_tags` | 支持 4 种排序 + 分页 <br> count/page 为 None 时返回全部 |
| `update_tag` | 校验新 slug 格式 <br> 唯一性冲突由 DB 返回错误 |
| `delete_tag` | 直接删除 <br> post_tags 会被 DB 级联删除 |
| `merge_tags` | 将 source_ids 的 post_tags 迁移到 target_id（冲突忽略）<br> 删除 source_ids 标签本体 <br> 重新计算并修正 target 的 post_count <br> 返回更新后的目标标签 |
| `increment_view_count` | `UPDATE tags SET view_count = view_count + 1 WHERE id = ?` |
| `adjust_post_count` | `UPDATE tags SET post_count = post_count + ? WHERE id = ?` |
| `get_tag_ids_by_slugs` | 辅助函数：批量获取标签 ID（给 post service 筛选使用）|

---

#### post - 文章

源码: [yukilog-backend/src/service/posts.rs](../src/service/posts.rs)

###### 接口定义

```rust
// src/service/posts.rs

/// 1. 创建文章
pub async fn create_post(
    db: &DatabaseConnection,
    input: CreatePostInput,
) -> ServiceResult<Post>

/// 2. 获取已发布文章详情（前台）
pub async fn get_published_post_by_slug(
    db: &DatabaseConnection,
    slug: &str,
) -> ServiceResult<Post>

/// 3. 获取文章详情（后台，包括草稿）
pub async fn get_post_by_slug(
    db: &DatabaseConnection,
    slug: &str,
) -> ServiceResult<Post>

/// 4. 获取文章及其关联数据
pub async fn get_post_with_relations(
    db: &DatabaseConnection,
    slug: &str,
    include_draft: bool,
) -> ServiceResult<PostWithRelations>

/// 5. 列出文章
pub async fn list_posts(
    db: &DatabaseConnection,
    filter: PostFilter,
) -> ServiceResult<Vec<Post>>

/// 6. 更新文章（统一接口，包括 slug）
pub async fn update_post(
    db: &DatabaseConnection,
    current_slug: &str,
    input: UpdatePostInput,
) -> ServiceResult<Post>

/// 7. 删除文章
pub async fn delete_post(
    db: &DatabaseConnection,
    slug: &str,
) -> ServiceResult<()>

/// 8. 增加浏览计数
pub async fn increment_view_count(
    db: &DatabaseConnection,
    post_id: i64,
) -> ServiceResult<()>

/// 9. 获取文章的所有标签
pub async fn get_post_tags<C: ConnectionTrait>(
    db: &C,
    post_id: i64,
) -> ServiceResult<Vec<Tag>>

/// 10. 统计文章数量（SELECT COUNT(*)）
/// 使用与 list_posts 相同的筛选条件，通过 repo::posts::count_posts 执行
pub async fn count_posts(
    db: &DatabaseConnection,
    filter: PostFilter,
) -> ServiceResult<u64>
```

###### DTO定义

```rust
// 输入
pub struct CreatePostInput {
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub cover_image: Option<String>,
    pub theme_slug: Option<String>,
    pub tag_slugs: Vec<String>,  // 自动创建不存在的标签
    pub status: PostStatus,
}

pub struct UpdatePostInput {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<Option<String>>,
    pub content: Option<String>,
    pub cover_image: Option<Option<String>>,
    pub theme_slug: Option<Option<String>>,  // None=不改, Some(None)=移除, Some(Some(x))=设置
    pub tag_slugs: Option<Vec<String>>,
    pub status: Option<PostStatus>,
}

pub struct PostFilter {
    pub theme_slugs: Option<Vec<String>>,
    pub tag_slugs: Option<Vec<String>>,  // AND 逻辑
    pub status: Option<PostStatus>,
    pub sort_by: Option<PostSortBy>,
    pub count: Option<u64>,
    pub page: Option<u64>,
}

pub enum PostSortBy {
    CreatedAt,   // 按创建时间倒序（最新文章）
    UpdatedAt,   // 按更新时间倒序（最近更新）
    ViewCount,   // 按浏览量倒序（最热门）
}

// 输出
pub struct Post {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub content: String,
    pub cover_image: Option<String>,
    pub status: PostStatus,
    pub theme_id: Option<i64>,
    pub view_count: i64,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

pub struct PostWithRelations {
    pub post: Post,
    pub theme: Option<Theme>,
    pub tags: Vec<Tag>,
}
```

###### 业务逻辑

| 接口名 | 职责 |
|-|-|
| `create_post` | slug 格式校验 <br> 自动创建不存在的标签（调用 `get_or_create_tag`）<br> 获取主题 ID <br> 创建文章和标签关联 <br> 如果 status=published：同步 theme/tags 计数 |
| `get_published_post_by_slug` | 查询文章并验证 status=published <br> 前台访问专用 |
| `get_post_by_slug` | 直接查询文章 <br> 后台访问专用，包括草稿 |
| `get_post_with_relations` | 获取文章 + 主题 + 所有标签 <br> 根据 include_draft 控制草稿访问 |
| `list_posts` | 支持按 theme/tag/status 筛选 <br> 标签筛选用 AND 逻辑 <br> 支持 3 种排序 + 分页 |
| `update_post` | 统一更新接口（包括 slug） <br> 处理主题变化：旧主题 -1，新主题 +1 <br> 处理标签变化：diff 计算，增删关联 <br> 处理状态变化：draft↔published 同步计数 <br> 仅在 published 状态同步计数 |
| `delete_post` | 如果 status=published：同步 theme/tags 计数 -1 <br> 删除文章（post_tags 由 DB CASCADE 删除）|
| `increment_view_count` | `UPDATE posts SET view_count = view_count + 1 WHERE id = ?` |
| `get_post_tags` | 通过 post_tags JOIN tags 查询标签列表 |
| `count_posts` | 使用与 list_posts 相同的筛选条件 <br> 通过 repo::posts::count_posts 执行 SELECT COUNT(*) <br> 用于分页接口计算 total |

---

#### comment - 评论

源码: [yukilog-backend/src/service/comments](../src/service/comments.rs)

###### 接口定义

```rust
// src/service/comments.rs

/// 1. 创建评论（前台）
pub async fn create_comment(
    db: &DatabaseConnection,
    post_slug: &str,
    input: CreateCommentInput,
) -> ServiceResult<Comment>

/// 2. 获取文章评论列表（前台，扁平，仅已审核）
pub async fn list_post_comments(
    db: &DatabaseConnection,
    post_slug: &str,
    filter: CommentFilter,
) -> ServiceResult<Vec<Comment>>

/// 3. 获取文章评论树（前台，树形，仅已审核）
pub async fn get_post_comment_tree(
    db: &DatabaseConnection,
    post_slug: &str,
) -> ServiceResult<Vec<CommentNode>>

/// 4. 获取评论详情
pub async fn get_comment_by_id(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Comment>

/// 5. 列出所有评论（后台，支持筛选和分页）
pub async fn list_all_comments(
    db: &DatabaseConnection,
    filter: AdminCommentFilter,
) -> ServiceResult<Vec<Comment>>

/// 6. 审核评论：通过
pub async fn approve_comment(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Comment>

/// 7. 审核评论：拒绝（标记为垃圾）
pub async fn reject_comment(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Comment>

/// 8. 更新评论内容（后台）
pub async fn update_comment(
    db: &DatabaseConnection,
    id: i64,
    input: UpdateCommentInput,
) -> ServiceResult<Comment>

/// 9. 删除评论（后台）
pub async fn delete_comment(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<()>

/// 10. 获取评论的回复列表（用于懒加载）
pub async fn list_comment_replies(
    db: &DatabaseConnection,
    parent_id: i64,
) -> ServiceResult<Vec<Comment>>

/// 11. 统计评论数量（SELECT COUNT(*)）
/// 使用与 list_all_comments 相同的筛选条件，通过 repo::comments::count_comments 执行
pub async fn count_all_comments(
    db: &DatabaseConnection,
    filter: AdminCommentFilter,
) -> ServiceResult<u64>
```

###### DTO定义

```rust
// 输入
pub struct CreateCommentInput {
    pub content: String,
    pub guest_nick: String,
    pub guest_email: Option<String>,
    pub guest_website: Option<String>,
    pub parent_id: Option<i64>,     // 回复谁
    pub ip: Option<String>,         // handler 层提供
    pub ua: Option<String>,         // handler 层提供
}

pub struct UpdateCommentInput {
    pub content: Option<String>,
    pub guest_nick: Option<String>,
    pub guest_email: Option<Option<String>>,
    pub guest_website: Option<Option<String>>,
}

pub struct CommentFilter {
    pub status: Option<CommentStatus>,  // 前台默认 Approved
    pub sort_by: Option<CommentSortBy>, // CreatedAt 升序/降序
}

pub struct AdminCommentFilter {
    pub post_id: Option<i64>,
    pub status: Option<CommentStatus>,
    pub count: Option<u64>,
    pub page: Option<u64>,
}

pub enum CommentSortBy {
    CreatedAtAsc,   // 时间正序（评论常用）
    CreatedAtDesc,  // 时间倒序
}

// 输出
pub struct Comment {
    pub id: i64,
    pub post_id: i64,
    pub content: String,
    pub guest_nick: String,
    pub guest_email: Option<String>,
    pub guest_website: Option<String>,
    pub parent_id: Option<i64>,
    pub root_id: Option<i64>,
    pub status: CommentStatus,
    pub ip: Option<String>,
    pub ua: Option<String>,
    pub created_at: DateTime<FixedOffset>,
}

pub struct CommentNode {
    pub comment: Comment,
    pub children: Vec<CommentNode>,
}
```

###### 业务逻辑

| 接口名 | 职责 |
|-|-|
| `create_comment` | 通过 post_slug 查询文章 <br> 验证文章已发布（draft 不允许评论）<br> 验证父评论存在且属于同一文章 <br> 计算 root_id：无 parent_id → None，有 parent_id → parent.root_id ?? parent_id <br> status 由 DB 默认为 pending |
| `list_post_comments` | 扁平列表，仅已审核 <br> 支持按创建时间正序/倒序 |
| `get_post_comment_tree` | 树形结构，仅已审核 <br> 递归构建评论树 |
| `get_comment_by_id` | 直接查询评论详情 |
| `list_all_comments` | 后台管理，支持按文章/状态筛选 <br> 支持分页 <br> 按创建时间倒序 |
| `approve_comment` | 修改 status 为 Approved |
| `reject_comment` | 修改 status 为 Spam |
| `update_comment` | 更新评论内容和游客信息 |
| `delete_comment` | 直接删除 <br> 子评论由 DB CASCADE 删除 |
| `list_comment_replies` | 获取指定评论的回复列表 <br> 用于懒加载子评论 |
| `count_all_comments` | 使用与 list_all_comments 相同的筛选条件 <br> 通过 repo::comments::count_comments 执行 SELECT COUNT(*) <br> 用于分页接口计算 total |

---

#### link - 友链

源码: [yukilog-backend/src/service/links.rs](../src/service/links.rs)

###### 接口定义

```rust
// src/service/links.rs

/// 1. 提交友链申请（前台调用）
pub async fn create_link_application(
    db: &DatabaseConnection,
    input: CreateLinkInput,
) -> ServiceResult<Link>

/// 2. 获取已通过的友链列表（前台展示）
pub async fn list_active_links(
    db: &DatabaseConnection,
) -> ServiceResult<Vec<Link>>

/// 3. 获取所有友链（后台分页列表）
pub async fn list_all_links(
    db: &DatabaseConnection,
    offset: u64,
    limit: u64,
) -> ServiceResult<(Vec<Link>, u64)>

/// 4. 获取待审核友链列表（后台待审核列表）
pub async fn list_pending_links(
    db: &DatabaseConnection,
) -> ServiceResult<Vec<Link>>

/// 5. 获取单个友链详情（后台编辑页）
pub async fn get_link_by_id(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Link>

/// 6. 审核通过友链（后台操作）
pub async fn approve_link(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Link>

/// 7. 标记友链失效（后台操作）
pub async fn mark_link_broken(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Link>

/// 8. 更新友链信息（后台编辑）
pub async fn update_link(
    db: &DatabaseConnection,
    id: i64,
    input: UpdateLinkInput,
) -> ServiceResult<Link>

/// 9. 删除友链（后台删除）
pub async fn delete_link(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<()>
```

###### DTO定义

```rust
// 排序
pub enum LinkSortBy {
    CreatedAtAsc,   // 按创建时间正序
    CreatedAtDesc,  // 按创建时间倒序（默认）
}

// 输入
pub struct CreateLinkInput {
    pub title: String,
    pub url: String,
    pub avatar: Option<String>,    // 外部链接
    pub description: Option<String>,
}

pub struct UpdateLinkInput {
    pub title: Option<String>,
    pub url: Option<String>,
    pub avatar: Option<Option<String>>,
    pub description: Option<Option<String>>,
}

// 输出
pub struct Link {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub status: LinkStatus,  // Active | Pending | Broken
    pub created_at: DateTime<FixedOffset>,
}
```

###### 业务逻辑

| 接口名 | 职责 |
|-|-|
| `create_link_application` | 1. URL 格式验证（http/https 前缀）<br> 2. 检查 URL 是否已存在 <br> 3. 若存在且为 broken 状态，更新信息并重置为 pending <br> 4. 若不存在，创建新友链（默认 pending） |
| `list_active_links` | 仅返回 status = 'active' 的友链 <br> 前台展示用 |
| `list_all_links` | 返回所有友链（分页）<br> 后台管理用 |
| `list_pending_links` | 仅返回 status = 'pending' 的友链 <br> 后台待审核列表 |
| `get_link_by_id` | 通过 ID 获取友链详情 |
| `approve_link` | 将状态设置为 'active' <br> 审核通过 |
| `mark_link_broken` | 将状态设置为 'broken' <br> 标记失效 |
| `update_link` | 更新友链的基本信息 |
| `delete_link` | 直接删除友链记录 |

---
