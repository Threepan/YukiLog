---
id: 7
title: "YukiLog - 3 - ORM 与 Entity 生成"
slug: "yukilog-3-orm-entity"
summary: "数据库建好了，但代码还不认识它。本篇介绍 ORM 的作用，以及如何让 Rust 自动理解数据库的表结构"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-3-orm-entity.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["blog", "rust", "教程"]
view_count: 235
created_at: "2026-02-20 02:16:19"
updated_at: "2026-08-15 21:30:18"
source: "https://blog.yeastar.xin/posts/yukilog-3-orm-entity"
---

## 引言

上一篇我们完成了数据库的设计与部署，现在数据库已经就绪，但 Rust 后端还不认识它

本篇聚焦于后端的第一层：**ORM 与 Entity**，也就是打通后端与数据库之间的那道墙

---

## 两个孤岛

先退一步想一个问题：后端和数据库，究竟是什么关系？

答案是：**它们是两个完全独立的进程**

* `PostgreSQL` 是一个独立运行的服务，默认监听在 `5432` 端口，等待连接
* `Axum` 后端是另一个独立运行的进程，监听在 `3000` 端口，处理 HTTP 请求

它们之间没有任何天然的联系，本质上是两座孤岛。要让后端能够操作数据库，首先要做的是**建立一条通信信道**——也就是通过 `DATABASE_URL` 发起 TCP 连接：

```
postgres://lian:密码@localhost:5432/yukilog
```

这条连接字符串包含了所有必要的信息：协议、用户名、密码、地址、端口、数据库名。后端程序启动时，`SeaORM` 会用它建立一个**连接池**，维护若干条复用的数据库连接，避免每次查询都重新握手的开销

信道建好了，但还有第二个问题：**两边说的语言不一样**

数据库说 SQL，返回的是一行行扁平的数据；Rust 程序操作的是结构体和枚举。这就是 ORM 要解决的事情

---

## 什么是 ORM

> ORM 负责在 Rust 的类型世界和数据库的表格世界之间做翻译

数据库返回的一行数据长这样：

```
id=1, title="Hello", status="published", theme_id=3, ...
```

而 Rust 程序想要的是这样：

```rust
Post { id: 1, title: "Hello", status: Some("published"), theme_id: Some(3), ... }
```

如果每次查询都手写这个转换，不仅枯燥，一旦表结构变动就会出错。**ORM 把这个翻译过程自动化了**，让你直接操作 Rust 类型，不用关心底层的 SQL 和数据转换

---

## 安装 sea-orm-cli

`SeaORM` 提供了一个命令行工具，可以直接连接数据库，读取表结构，然后自动生成 Rust 代码

```shell
cargo install sea-orm-cli
```

安装完成后，只需要一条命令就能完成 Entity 生成：

```shell
sea-orm-cli generate entity \
  -u postgres://lian:你的密码@localhost/yukilog \
  -o src/entities
```

* `-u` 是数据库连接字符串
* `-o` 是输出目录，生成的文件会放在 `src/entities/` 下

---

## 生成的文件结构

执行命令后，`src/entities/` 目录下会出现这些文件：

```
src/entities/
├── mod.rs       # 模块声明
├── prelude.rs   # 统一导出，方便外部使用
├── posts.rs
├── comments.rs
├── tags.rs
├── post_tags.rs
├── themes.rs
└── links.rs
```

`prelude.rs` 的作用是把所有 Entity 集中导出，这样在其他模块里只需要 `use crate::entities::prelude::*` 就能拿到所有类型：

```rust
pub use super::comments::Entity as Comments;
pub use super::links::Entity as Links;
pub use super::post_tags::Entity as PostTags;
pub use super::posts::Entity as Posts;
pub use super::tags::Entity as Tags;
pub use super::themes::Entity as Themes;
```

---

## 解读生成的 Entity

以 `posts.rs` 为例，来看看 SeaORM 生成了什么：

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(unique)]
    pub slug: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub summary: Option,
    #[sea_orm(column_type = "Text")]
    pub content: String,
    pub cover_image: Option,
    pub status: Option,
    pub theme_id: Option,
    pub view_count: Option,
    pub created_at: Option,
    pub updated_at: Option,
}
```

这里有几个值得注意的地方：

* `#[derive(DeriveEntityModel)]` 是 SeaORM 的核心宏，它会自动生成 `Entity`、`Column`、`ActiveModel` 等一系列类型
* `#[sea_orm(table_name = "posts")]` 告诉 ORM 这个结构体对应数据库里的哪张表
* 可以为 `null` 的字段被映射成了 `Option`，这和 Rust 的类型系统完美契合
* `DateTimeWithTimeZone` 对应 SQL 里的 `TIMESTAMP WITH TIME ZONE`

---

## Model 与 ActiveModel

SeaORM 里有两个核心概念经常让人困惑：`Model` 和 `ActiveModel`

**`Model`** 是只读的，代表从数据库里查出来的一行数据：

```rust
// 查询返回的是 Model
let post: posts::Model = posts::Entity::find_by_id(1)
    .one(&db)
    .await?
    .unwrap();

println!("{}", post.title); // 只能读
```

**`ActiveModel`** 是可写的，用于插入和更新操作。它的每个字段都被包裹在 `ActiveValue` 里，有三种状态：

* `Set(value)` — 明确设置这个字段
* `NotSet` — 不操作这个字段（更新时会跳过）
* `Unchanged(value)` — 字段值没有变化

```rust
// 插入新文章
let new_post = posts::ActiveModel {
    title: Set("Hello World".to_string()),
    slug: Set("hello-world".to_string()),
    content: Set("这是内容".to_string()),
    status: Set(Some("draft".to_string())),
    ..Default::default() // 其余字段交给数据库默认值
};

let result = new_post.insert(&db).await?;
```

这个设计的好处是：更新时只需要 `Set` 你想改的字段，其他字段保持 `NotSet`，ORM 生成的 SQL 就只会 `UPDATE` 那几列，不会误改其他数据

---

## 关联关系

SeaORM 会根据数据库的外键约束，自动生成 `Relation` 枚举来描述表之间的关系

还是以 `posts.rs` 为例：

```rust
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::comments::Entity")]
    Comments,
    #[sea_orm(has_many = "super::post_tags::Entity")]
    PostTags,
    #[sea_orm(
        belongs_to = "super::themes::Entity",
        from = "Column::ThemeId",
        to = "super::themes::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    Themes,
}
```

* `has_many` — 一对多，一篇文章有多条评论
* `belongs_to` — 多对一，文章通过 `theme_id` 外键属于某个主题

---

## 多对多关系的处理

`posts` 和 `tags` 之间是多对多关系，通过 `post_tags` 关联表实现。SeaORM 对这种情况有专门的处理方式

`post_tags.rs` 本身很简单，就是两个外键组成的复合主键：

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "post_tags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub post_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub tag_id: i64,
}
```

注意 `auto_increment = false`，因为这张表没有自增 ID，主键是 `(post_id, tag_id)` 的组合

真正有意思的是 `posts.rs` 里通过 `via()` 实现的跨表关联：

```rust
impl Related for Entity {
    fn to() -> RelationDef {
        super::post_tags::Relation::Tags.def()
    }
    fn via() -> Option {
        Some(super::post_tags::Relation::Posts.def().rev())
    }
}
```

这段代码告诉 SeaORM：从 `posts` 到 `tags`，要先经过 `post_tags` 这张中间表。有了这个定义，查询一篇文章的所有标签就变得非常简洁：

```rust
let tags = post_model
    .find_related(Tags)
    .all(&db)
    .await?;
```

---

## 评论的自引用关系

评论表有一个特殊的地方：`parent_id` 和 `root_id` 都指向 `comments` 表自身，这叫做**自引用关系**

SeaORM 生成的代码里，这两个关系被命名为 `SelfRef1` 和 `SelfRef2`：

```rust
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    SelfRef2,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::RootId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    SelfRef1,
    // ...
}
```

`on_delete = "Cascade"` 在这里非常关键：当一条根评论被删除时，它下面所有的子评论也会被数据库自动级联删除，不需要在应用层手动处理

---

## 小结

到这里，Entity 层就完成了。它做的事情很纯粹：

1. 用 `Model` 描述数据库里的每一行数据长什么样
2. 用 `ActiveModel` 提供类型安全的写入接口
3. 用 `Relation` 描述表与表之间的关联

Entity 层本身不包含任何业务逻辑，它只是数据库 Schema 在 Rust 类型系统里的镜像

下一篇我们来看 **Repository 层**，在 Entity 的基础上封装具体的查询操作
