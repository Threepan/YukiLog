<div align="center">

## YukiLog 预留内部 API 索引

当前未被 handler 层调用，但作为标准库存保留，供后续扩展使用

</div>

---

## 目录

| 层级 | 说明 |
| --- | --- |
| Repo | [基础 CRUD 查询](#repo)（被 `_filtered` / `_sorted` 版本替代，但保留为原子接口） |
| Service | [业务辅助函数](#service)（handler 尚未接入的能力） |
| Handler | [响应工具函数](#handler)（已有的 helper，待使用场景出现） |

---

<a id="repo"></a>
## Repo 层 — 基础 CRUD 查询

> 这些函数是标准的「全量查询 / 按 ID 查询」原子接口。  
> 当前业务由 `_filtered` / `_sorted` 版本承担，基础版本保留供测试、导出、定时任务等场景使用。

### Themes

```rust
// src/repo/themes.rs

/// 获取所有主题（无排序、无筛选）
pub async fn list_themes<C: ConnectionTrait>(db: &C) -> RepoResult<Vec<ThemeDto>>
```

### Tags

```rust
// src/repo/tags.rs

/// 获取所有标签（无排序、无分页）
pub async fn list_tags<C: ConnectionTrait>(db: &C) -> RepoResult<Vec<TagDto>>
```

### Posts

```rust
// src/repo/posts.rs

/// 获取所有文章（无排序、无筛选、无分页）
pub async fn list_posts<C: ConnectionTrait>(db: &C) -> RepoResult<Vec<PostDto>>

/// 按 ID 获取单篇文章
pub async fn get_post_by_id<C: ConnectionTrait>(db: &C, id: i64) -> RepoResult<PostDto>
```

### Comments

```rust
// src/repo/comments.rs

/// 获取所有评论（无排序、无筛选）
pub async fn list_comments<C: ConnectionTrait>(db: &C) -> RepoResult<Vec<CommentDto>>

/// 获取指定文章下的所有评论
pub async fn list_comments_by_post_id<C: ConnectionTrait>(db: &C, post_id: i64) -> RepoResult<Vec<CommentDto>>
```

### Links

```rust
// src/repo/links.rs

/// 获取所有友链（无排序、无筛选）
pub async fn list_links<C: ConnectionTrait>(db: &C) -> RepoResult<Vec<LinkDto>>

/// 按 ID 获取单个友链
pub async fn get_link_by_id<C: ConnectionTrait>(db: &C, id: i64) -> RepoResult<LinkDto>
```

### PostTags

```rust
// src/repo/post_tags.rs

/// 按 (post_id, tag_id) 获取单条关联
pub async fn get_post_tag<C: ConnectionTrait>(db: &C, post_id: i64, tag_id: i64) -> RepoResult<PostTagDto>

/// 获取所有关联记录
pub async fn list_post_tags<C: ConnectionTrait>(db: &C) -> RepoResult<Vec<PostTagDto>>

/// 获取指定文章的所有关联
pub async fn list_post_tags_by_post_id<C: ConnectionTrait>(db: &C, post_id: i64) -> RepoResult<Vec<PostTagDto>>

/// 获取指定标签的所有关联
pub async fn list_post_tags_by_tag_id<C: ConnectionTrait>(db: &C, tag_id: i64) -> RepoResult<Vec<PostTagDto>>
```

### Notes

```rust
// src/repo/notes.rs

/// 按 ID 获取单条随记
pub async fn get_note_by_id<C: ConnectionTrait>(db: &C, id: i64) -> RepoResult<NoteDto>

/// 获取随记总数（可按状态筛选）
pub async fn count_notes<C: ConnectionTrait>(db: &C, status: Option<&str>) -> RepoResult<u64>
```

---

<a id="service"></a>
## Service 层 — 业务辅助函数

> handler 层尚未接入，但属于合理的公共业务能力，后续可直接对接 handler。

### Themes

```rust
// src/service/themes.rs

/// 通过 ID 获取主题详情
pub async fn get_theme_by_id<C: ConnectionTrait>(db: &C, theme_id: i64) -> ServiceResult<Theme>
```

**用途**：当需要根据 theme_id 反查主题信息时使用（例如文章详情附带主题数据）。

### Posts

```rust
// src/service/posts.rs

/// 文章及其关联数据的聚合结构
#[derive(Debug, Clone)]
pub struct PostWithRelations {
    pub post: Post,
    pub theme: Option<Theme>,
    pub tags: Vec<Tag>,
}

/// 获取文章及其关联数据（主题 + 标签），一次查询返回完整视图
pub async fn get_post_with_relations(
    db: &DatabaseConnection,
    slug: &str,
    include_draft: bool,
) -> ServiceResult<PostWithRelations>
```

**用途**：文章详情页需要同时展示主题和标签时，可直接返回完整聚合对象，避免前端多次请求。

### Comments

```rust
// src/service/comments.rs

/// 获取评论详情（按 ID）
pub async fn get_comment_by_id(db: &DatabaseConnection, id: i64) -> ServiceResult<Comment>
```

**用途**：管理端需要查看单条评论详情、或评论编辑前预加载数据时使用。

### Links

```rust
// src/service/links.rs

/// 获取友链详情（按 ID）
pub async fn get_link_by_id(db: &DatabaseConnection, id: i64) -> ServiceResult<Link>
```

**用途**：管理端需要查看单条友链详情、或友链编辑前预加载数据时使用。

### Notes

```rust
// src/service/notes.rs

/// 获取随记详情（按 ID，管理端不限状态）
pub async fn get_note(db: &DatabaseConnection, id: i64) -> ServiceResult<Note>

/// 获取已发布随记数量
pub async fn count_public_notes(db: &DatabaseConnection) -> ServiceResult<u64>

/// 获取所有随记数量（含草稿/私密）
pub async fn count_all_notes(db: &DatabaseConnection) -> ServiceResult<u64>
```

**用途**：
- `get_note`：管理端查看单条随记详情（不限状态），或作为更新前的预加载
- `count_public_notes`：站点统计可扩展为包含随记数量
- `count_all_notes`：管理面板仪表盘展示总随记数

---

<a id="handler"></a>
## Handler 层 — 响应工具函数

> `handler::response` 模块预留的响应构造器，当前由其他快捷方式覆盖，但保留备用。

```rust
// src/handler/response.rs

/// 返回成功响应（带数据 + 自定义消息）
pub fn ok_with_message<T: Serialize>(data: T, message: impl Into<String>) -> Json<ApiResponse<T>>

/// 返回错误响应（独立函数版本）
pub fn error(message: impl Into<String>) -> Json<ApiResponse<()>>

/// ApiResponse::error（方法版本）
impl<T> ApiResponse<T> {
    pub fn error(message: impl Into<String>) -> Self
}
```

**用途**：
- `ok_with_message`：需要在成功响应中附带提示信息时（如 "操作成功，但有 N 条记录跳过"）
- `error`：在不通过 `AppError` 机制、直接构造错误 JSON 时使用
