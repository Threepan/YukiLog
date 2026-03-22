<div align="center">

# YukiLog 数据库文档

数据库文件地址: [yukilog.sql](../../yukilog-database/db/yukilog.sql)

</div>

---

## | ORM

ORM 是用 `sea-orm-cli` 做的, 目录在 [entities](../src/entities/)

这个库生成的表/关系定义可以直接拿来做增删改查

不过嘛...我们数据库里的一些值用的是枚举, 所以我要再封装一层 `repo`

---

## | repo

我的 `repo` 放在 [yukilog-backend/src/repo](../src/repo)，它的目标是：

* 只负责“数据访问”（CRUD、简单查询、筛选/排序/分页、计数更新），不掺杂业务流程
* 把数据库里的“弱类型字段”（如 `status: VARCHAR`）在 Rust 侧封装成强类型（enum），避免业务层到处写字符串
* 给上层（service/handler）提供稳定、好用、可测试的原子接口
* **所有 SQL（包括原生 SQL 和 SeaORM 查询構建）都封装在 repo 层，service 层不直接操作数据库**

#### 目录与职责

* 实体层（codegen）：[yukilog-backend/src/entities](../src/entities)
	* `Entity/Model/ActiveModel` + 关系定义

* 领域类型层（枚举封装等）：[yukilog-backend/src/domain](../src/domain)
	* 例如 [yukilog-backend/src/domain/status.rs](../src/domain/status.rs) 里定义了 `PostStatus`/`CommentStatus`/`LinkStatus`/`NoteStatus`
	* `mood` 字段不使用枚举，直接以 `String` 透传，前端 config 维护预设查询表

* Repo 层（对外 CRUD 原子接口）：[yukilog-backend/src/repo](../src/repo)
	* 统一错误类型： [yukilog-backend/src/repo/error.rs](../src/repo/error.rs)
	* 每张表一个文件：例如 [yukilog-backend/src/repo/posts.rs](../src/repo/posts.rs)

#### CRUD 模板 + 高级查询 (以 posts 为例)

在 [yukilog-backend/src/repo/posts.rs](../src/repo/posts.rs) 中已实现：

CRUD 基础接口：

* `create_post(db, CreatePost) -> RepoResult<PostDto>`: (创建一条 post 记录)
* `get_post_by_id(db, id) -> RepoResult<PostDto>`: (用 id 获取 post 记录)
* `get_post_by_slug(db, slug) -> RepoResult<PostDto>`: (用 slug 获取 post 记录)
* `list_posts(db) -> RepoResult<Vec<PostDto>>`: (获取所有 post 记录)
* `update_post(db, id, UpdatePost) -> RepoResult<PostDto>`: (用 id 更新一条 post 记录)
* `delete_post(db, id) -> RepoResult<()>`: (用 id 删除一条记录)

高级查询接口：

* `count_posts(db, theme_ids, post_ids, status) -> RepoResult<u64>`: (按条件统计文章数量)
* `increment_view_count(db, id) -> RepoResult<()>`: (将指定文章的 view_count + 1)
* `list_posts_filtered(db, theme_ids, post_ids, status, sort_by, count, page) -> RepoResult<Vec<PostDto>>`: (按条件筛选文章列表)
* `get_post_ids_with_all_tags(db, tag_ids, required_count) -> RepoResult<Vec<i64>>`: (获取同时拥有所有指定标签的文章 ID)

数据结构介绍:

* `PostDto`

这是所有 `posts` 表的 **CUR接口** 都会返回的标准数据结构

```rust
pub struct PostDto {
    /// id 号, i64: 64 位有符号整数, 范围是 -2^63 ~ 2^63-1 
    pub id: i64,
    /// 标题, String: 位于堆上的UTF-8字节数组
    pub title: String,
    /// slug, 是 URL 友好型标题
    pub slug: String,
    /// 摘要, Option 是可选, 需要用 Some() 创建
    pub summary: Option<String>,
    /// 内容
    pub content: String,
    /// 文章封面
    pub cover_image: Option<String>,
    /// 状态, PostStatus 是一个枚举值
    pub status: Option<PostStatus>,
    /// 主题 id
    pub theme_id: Option<i64>,
    /// 浏览量
    pub view_count: Option<i64>,
    /// 创建时间
    pub created_at: Option<chrono::DateTime<chrono::FixedOffset>>,
    /// 最后更新时间
    pub updated_at: Option<chrono::DateTime<chrono::FixedOffset>>,
}
```

> `status` 字段在 orm 中的定义为 `Option<String>`, 这就是为什么我们需要 `repo` 层
>
> `repo` 层可以强制检查 `status` 字段的类型为 `Option<PostStatus>`
>
> 同时我们可以定义 `PostStatus::as_str()`, 在后端与数据库的边界进行类型转换

* `PostStatus`

```rust
pub enum PostStatus {
    Draft,
    Published,
}
```

* `CreatePost`

```rust
pub struct CreatePost {
    /// 标题
    pub title: String,
    /// slug
    pub slug: String,
    /// 摘要
    pub summary: Option<String>,
    /// 内容
    pub content: String,
    /// 封面
    pub cover_image: Option<String>,
    /// 主题
    pub theme_id: Option<i64>,
}
```

* `UpdatePost`

```rust
pub struct UpdatePost {
    /// 标题
    pub title: Option<String>,
    /// slug
    pub slug: Option<String>,
    /// 摘要
    pub summary: Option<Option<String>>,
    /// 内容
    pub content: Option<String>,
    /// 封面图
    pub cover_image: Option<Option<String>>,
    /// 状态
    pub status: Option<Option<PostStatus>>,
    /// 主题
    pub theme_id: Option<Option<i64>>,
}
```

---

## | finally

示例就放到这里结束啦~

完整的 `repo` 层文档请看这里~ [YukiLog Repo](./repo.md)
