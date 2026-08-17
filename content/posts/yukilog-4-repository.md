---
id: 8
title: "YukiLog - 4 - Repository 层"
slug: "yukilog-4-repository"
summary: "ORM 打通了后端与数据库的信道，但我们不想让 SQL 的影子渗透到整个项目。本篇介绍 Repository 层如何在这里竖起一道墙"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-4-repository.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["blog", "rust", "教程"]
view_count: 234
created_at: "2026-02-20 03:30:52"
updated_at: "2026-08-17 10:41:31"
source: "https://blog.yeastar.xin/posts/yukilog-4-repository"
---

## 引言

上一篇我们用 SeaORM 打通了后端与数据库的连接，生成了对应每张表的 Entity

但现在有一个问题：如果在业务逻辑里直接写 SeaORM 查询，整个项目就会到处散落着 `posts::Entity::find()`、`Column::Status.eq("published")` 这样的代码。一旦数据库表结构变动，或者哪天想换一个 ORM，改动就会蔓延到每一个角落

**Repository 层的职责就是在这里竖起一道墙**

墙的这一侧是数据库的世界：SQL、ORM、`Model`、`ActiveModel`

墙的另一侧是纯粹的 Rust 世界：结构体、枚举、`Result`

越过这道墙之后，上层代码永远不需要知道数据是怎么存的，只需要调用函数、拿到结果

---

## DTO：跨越边界的数据载体

Entity 层生成的 `Model` 是数据库行的直接映射，它的类型反映的是数据库的现实：

```rust
// Entity 生成的 Model，status 是 Option
pub struct Model {
    pub id: i64,
    pub title: String,
    pub status: Option,  // 数据库存的是字符串
    // ...
}
```

但在 Rust 的世界里，一个只能是 `"draft"` 或 `"published"` 的字段，不应该是 `String`。`String` 意味着任何值都合法，这不是我们想要的

所以 Repository 层定义了自己的数据结构：**DTO (Data Transfer Object)**

```rust
pub struct PostDto {
    pub id: i64,
    pub title: String,
    pub status: Option,  // Rust 枚举，只有合法值
    // ...
}
```

`Model` 到 `PostDto` 的转换发生在边界上，通过 `TryFrom` 实现：

```rust
impl TryFrom for PostDto {
    type Error = RepoError;

    fn try_from(model: posts::Model) -> Result {
        let status = match model.status.as_deref() {
            None => None,
            Some(s) => Some(PostStatus::try_from(s)?),
        };

        Ok(Self {
            id: model.id,
            title: model.title,
            status,
            // ...
        })
    }
}
```

这个转换是**有可能失败的**，所以返回的是 `Result`。如果数据库里存了一个不认识的 status 字符串，这里会返回错误，而不是把脏数据悄悄传递给上层

---

## Enum 约束：业务语义属于代码

这里值得停下来聊一个设计决策：**为什么不在数据库里直接用 Enum 类型？**

PostgreSQL 原生支持 `CREATE TYPE post_status AS ENUM ('draft', 'published')`，这样数据库本身就能拒绝非法值。但我没有这么做，原因是我对数据库的定位是：**数据库只是存储的抽象，业务语义由代码负责**

数据库不应该承载业务规则。它的职责是可靠地存取数据，至于这些数据代表什么含义、有哪些合法状态，是应用层的事情。这个边界一旦模糊，业务逻辑就会开始向数据库迁移，维护成本会越来越高

所以 `domain/status.rs` 里定义了三组枚举，覆盖项目里所有有状态的实体：

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostStatus {
    Draft,
    Published,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommentStatus {
    Approved,
    Pending,
    Spam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkStatus {
    Active,
    Pending,
    Broken,
}
```

每个枚举都实现了 `TryFrom<&str>`，负责从数据库字符串转换过来；以及 `as_str()`，负责写入数据库时转回字符串。转换的边界就在 Repository 层，出了这道门，上层代码只会看到 `PostStatus::Published`，永远不会看到裸字符串 `"published"`

---

## CRUD：输入结构体的设计

Repository 层为每个实体定义了两个输入结构体：`CreateXxx` 和 `UpdateXxx`

**创建**很直接，就是一组必填和可选字段：

```rust
pub struct CreatePost {
    pub title: String,
    pub slug: String,
    pub summary: Option,
    pub content: String,
    pub cover_image: Option,
    pub status: Option,
    pub theme_id: Option,
}
```

**更新**稍微有意思一点。先想一个问题：调用方发来一个更新 `summary` 的请求，可能有三种意图：

```
1. 我没提 summary，你别动它
2. 我要把 summary 清空，设成 null
3. 我要把 summary 改成 "新的摘要"
```

用普通的 `Option` 来表达：

```
summary: None              // 到底是意图1还是意图2？不知道
summary: Some("新的摘要")  // 意图3，没问题
```

`None` 的语义是模糊的，没办法区分"没提这个字段"和"想清空这个字段"

解决方案是 `Option>`，把三种意图全部表达清楚：

```
summary: None                    // 外层 None：没提这个字段，别动它
summary: Some(None)              // 外层 Some，内层 None：要清空它
summary: Some(Some("新的摘要"))  // 外层 Some，内层 Some：要改成这个值
```

外层 `Option` 回答：**这次更新有没有涉及这个字段？**
内层 `Option` 回答：**如果涉及了，新值是什么（可以是 null）？**

注意只有本身就可以为 null 的字段才需要这个模式，像 `title` 这种不能为空的字段，`Option` 就够了——不存在"清空 title"这种操作

```rust
pub struct UpdatePost {
    pub title: Option,           // 普通字段：None=不改，Some(v)=改成v
    pub summary: Option>, // 可空字段：None=不改，Some(None)=清空，Some(Some(v))=改成v
    pub cover_image: Option>,
    pub status: Option>,
    pub theme_id: Option>,
}
```

写入时，只有 `Some` 的字段才会被 `Set`，其余保持 `NotSet`，SeaORM 生成的 SQL 只会 `UPDATE` 真正变动的列：

```rust
let mut active = model.into_active_model();

if let Some(v) = patch.title {
    active.title = Set(v);
}
if let Some(v) = patch.status {
    active.status = Set(v.map(|s| s.as_str().to_string()));
}
// 没有 Set 的字段，SeaORM 不会碰它们

let updated = active.update(db).await?;
```

---

## 错误处理

Repository 层定义了自己的错误类型，把所有可能出现的错误统一收口：

```rust
#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("db error: {0}")]
    Db(#[from] DbErr),       // SeaORM 的数据库错误

    #[error("not found")]
    NotFound,                // 查询结果为空

    #[error(transparent)]
    InvalidStatus(#[from] InvalidStatus),  // 数据库里存了非法的 status 字符串
}

pub type RepoResult = Result;
```

`#[from]` 让 `DbErr` 和 `InvalidStatus` 可以用 `?` 自动转换成 `RepoError`，不需要手写 `map_err`

上层代码拿到的永远是 `RepoResult`，不会接触到 `DbErr` 这种 ORM 内部的错误类型，边界再次被守住了

---

## 高级查询

基础 CRUD 之外，Repository 层还封装了一些上层会用到的复合查询

**分页列表**接受过滤条件，返回数据和总数：

```rust
pub async fn list_posts_filtered(
    db: &C,
    theme_id: Option,
    status: Option<&str>,
    page: u64,
    page_size: u64,
) -> RepoResult<(Vec, u64)>
```

**全文搜索**需要跨 `title`、`summary`、`content` 三列模糊匹配，并按相关性排序。这类查询 SeaORM 的链式 API 表达起来比较繁琐，直接用原生 SQL 更清晰：

```rust
let sql = r#"
    SELECT * FROM posts
    WHERE status = 'published'
      AND (title ILIKE $1 OR summary ILIKE $1 OR content ILIKE $1)
    ORDER BY
      CASE WHEN title ILIKE $1 THEN 0 ELSE 1 END,
      created_at DESC
    LIMIT $2 OFFSET $3
"#;
```

SeaORM 允许在需要的地方退回到原生 SQL，这是一个务实的设计——ORM 不是枷锁，是工具

---

## 小结

Repository 层做的事情可以用一句话概括：**让数据库的细节止步于此**

* `DTO` 替代 `Model`，切断上层对 Entity 类型的依赖
* `Enum` 替代 `String`，把业务语义收归代码管理
* `RepoError` 替代 `DbErr`，统一错误边界
* 输入结构体替代裸参数，让调用方的意图清晰可读

从下一篇开始，我们进入 Service 层。那里不再有任何数据库的痕迹，只有业务逻辑
