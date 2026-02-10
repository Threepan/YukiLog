<div align="center">

# YukiLog Repo 层文档

Repo 的目标：把 SeaORM 的实体层（弱类型字段、ActiveModel 操作细节）封装起来，给后续业务层提供稳定的 CRUD 原子接口。

</div>

---

## | 目录结构

* 实体层（codegen）：[yukilog-backend/src/entities](../src/entities)
* 领域类型（枚举封装）：[yukilog-backend/src/domain](../src/domain)
* Repo 层（本章重点）：[yukilog-backend/src/repo](../src/repo)

Repo 的模块入口：

* [yukilog-backend/src/repo/mod.rs](../src/repo/mod.rs)
* 统一错误类型：[yukilog-backend/src/repo/error.rs](../src/repo/error.rs)

---

## | 通用约定

#### Repo 的职责边界

* Repo 只做“数据访问”：CRUD、按索引字段查询、简单 list/filter
* Repo 不做业务编排：跨表事务（例如“创建文章并绑定标签”）请放在后续 service 层

#### 为什么要 DTO

实体层的字段类型由数据库决定：例如 `status VARCHAR(20)` 在实体里通常是 `Option<String>`。

Repo 返回 DTO（如 `PostDto`），把这些弱类型字段转换成强类型 enum（如 `Option<PostStatus>`），这样业务层就不会到处散落 `"draft"` 这种魔法字符串。

#### 错误与返回值

Repo 统一返回：

```rust
pub type RepoResult<T> = Result<T, RepoError>;

pub enum RepoError {
	Db(sea_orm::DbErr),
	NotFound,
	InvalidStatus(crate::domain::status::InvalidStatus),
}
```

对应实现见：[yukilog-backend/src/repo/error.rs](../src/repo/error.rs)

#### status 枚举封装

当前有 `status` 字段的表：`posts` / `comments` / `links`。

对应枚举类型定义在：[yukilog-backend/src/domain/status.rs](../src/domain/status.rs)

* `PostStatus`：`draft` / `published`
* `CommentStatus`：`approved` / `pending` / `spam`
* `LinkStatus`：`active` / `pending` / `broken`

Repo 会在 `Model -> Dto` 时执行 `TryFrom<&str>` 校验：

* 如果数据库里存在非法值（例如拼错），Repo 会返回 `RepoError::InvalidStatus`
* 写回数据库时会通过 `status.as_str()` 转回字符串

---

## | posts

源码：[yukilog-backend/src/repo/posts.rs](../src/repo/posts.rs)

##### CRUD 模板

* `create_post(db, CreatePost) -> RepoResult<PostDto>`: (创建一条 post 记录)
* `get_post_by_id(db, id) -> RepoResult<PostDto>`: (用 id 获取 post 记录)
* `get_post_by_slug(db, slug) -> RepoResult<PostDto>`: (用 slug 获取 post 记录)
* `list_posts(db) -> RepoResult<Vec<PostDto>>`: (获取所有 post 记录)
* `update_post(db, id, UpdatePost) -> RepoResult<PostDto>`: (更新一条 post 记录)
* `delete_post(db, id) -> RepoResult<()>`: (删除一条 post 记录)

##### 数据结构

* `PostDto`：所有 CUR 接口的标准返回结构
* `CreatePost`：创建输入
* `UpdatePost`：更新 patch（`Option<T>` 表示是否更新该字段；`Option<Option<T>>` 表示可更新成 NULL）

---

## | comments

源码：[yukilog-backend/src/repo/comments.rs](../src/repo/comments.rs)

##### CRUD 模板

* `create_comment(db, CreateComment) -> RepoResult<CommentDto>`: (创建一条 comment 记录)
* `get_comment_by_id(db, id) -> RepoResult<CommentDto>`: (用 id 获取 comment 记录)
* `list_comments(db) -> RepoResult<Vec<CommentDto>>`: (获取所有 comment 记录)
* `list_comments_by_post_id(db, post_id) -> RepoResult<Vec<CommentDto>>`: (获取某篇文章下的所有评论)
* `update_comment(db, id, UpdateComment) -> RepoResult<CommentDto>`: (更新一条 comment 记录)
* `delete_comment(db, id) -> RepoResult<()>`: (删除一条 comment 记录)

##### 数据结构

* `CommentDto`：返回结构（包含 `status: Option<CommentStatus>`）
* `CreateComment`：创建输入（不包含 `status`，走 DB 默认 `pending`）
* `UpdateComment`：更新 patch（可更新 `status`）

---

## | links

源码：[yukilog-backend/src/repo/links.rs](../src/repo/links.rs)

##### CRUD 模板

* `create_link(db, CreateLink) -> RepoResult<LinkDto>`: (创建一条 link 记录)
* `get_link_by_id(db, id) -> RepoResult<LinkDto>`: (用 id 获取 link 记录)
* `get_link_by_url(db, url) -> RepoResult<LinkDto>`: (用 url 获取 link 记录)
* `list_links(db) -> RepoResult<Vec<LinkDto>>`: (获取所有 link 记录)
* `update_link(db, id, UpdateLink) -> RepoResult<LinkDto>`: (更新一条 link 记录)
* `delete_link(db, id) -> RepoResult<()>`: (删除一条 link 记录)

##### 数据结构

* `LinkDto`：返回结构（包含 `status: Option<LinkStatus>`）
* `CreateLink`：创建输入（不包含 `status`，走 DB 默认 `pending`）
* `UpdateLink`：更新 patch（可更新 `status`）

---

## | tags

源码：[yukilog-backend/src/repo/tags.rs](../src/repo/tags.rs)

##### CRUD 模板

* `create_tag(db, CreateTag) -> RepoResult<TagDto>`: (创建一条 tag 记录)
* `get_tag_by_id(db, id) -> RepoResult<TagDto>`: (用 id 获取 tag 记录)
* `get_tag_by_slug(db, slug) -> RepoResult<TagDto>`: (用 slug 获取 tag 记录)
* `list_tags(db) -> RepoResult<Vec<TagDto>>`: (获取所有 tag 记录)
* `update_tag(db, id, UpdateTag) -> RepoResult<TagDto>`: (更新一条 tag 记录)
* `delete_tag(db, id) -> RepoResult<()>`: (删除一条 tag 记录)

##### 数据结构

* `TagDto`
* `CreateTag`
* `UpdateTag`

---

## | themes

源码：[yukilog-backend/src/repo/themes.rs](../src/repo/themes.rs)

##### CRUD 模板

* `create_theme(db, CreateTheme) -> RepoResult<ThemeDto>`: (创建一条 theme 记录)
* `get_theme_by_id(db, id) -> RepoResult<ThemeDto>`: (用 id 获取 theme 记录)
* `get_theme_by_slug(db, slug) -> RepoResult<ThemeDto>`: (用 slug 获取 theme 记录)
* `list_themes(db) -> RepoResult<Vec<ThemeDto>>`: (获取所有 theme 记录)
* `update_theme(db, id, UpdateTheme) -> RepoResult<ThemeDto>`: (更新一条 theme 记录)
* `delete_theme(db, id) -> RepoResult<()>`: (删除一条 theme 记录)

##### 数据结构

* `ThemeDto`
* `CreateTheme`
* `UpdateTheme`

---

## | post_tags

`post_tags` 是文章与标签的多对多关联表（复合主键：`(post_id, tag_id)`）。

源码：[yukilog-backend/src/repo/post_tags.rs](../src/repo/post_tags.rs)

##### CRUD 模板

* `create_post_tag(db, CreatePostTag) -> RepoResult<PostTagDto>`: (创建一条 post_tags 关联记录)
* `get_post_tag(db, post_id, tag_id) -> RepoResult<PostTagDto>`: (用复合主键获取一条关联记录)
* `list_post_tags(db) -> RepoResult<Vec<PostTagDto>>`: (获取所有关联记录)
* `list_post_tags_by_post_id(db, post_id) -> RepoResult<Vec<PostTagDto>>`: (获取某篇文章绑定的所有 tag 关联)
* `list_post_tags_by_tag_id(db, tag_id) -> RepoResult<Vec<PostTagDto>>`: (获取某个 tag 关联的所有文章)
* `delete_post_tag(db, post_id, tag_id) -> RepoResult<()>`: (删除一条关联记录)

##### 数据结构

* `PostTagDto`
* `CreatePostTag`

---
