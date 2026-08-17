---
id: 17
title: "YukiLog - 12 - 精选文章：一个字段的全栈旅程"
slug: "yukilog-12-articlefullstack"
summary: "主页不应该显示所有文章——有些文章只是草稿性质的记录，不适合放在首屏。于是加了一个 is_featured 字段，从数据库一路走到前端"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-12-articlefullstack.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["astro", "blog"]
view_count: 279
created_at: "2026-02-26 21:34:03"
updated_at: "2026-08-17 15:37:04"
source: "https://blog.yeastar.xin/posts/yukilog-12-articlefullstack"
---

## 问题的起点

YukiLog 的主页有一个文章列表，显示最新发布的文章。

这个设计在文章数量少的时候没有问题。但随着内容增多，我发现有些文章不适合出现在主页——它们是发布状态，但更像是个人记录，不是我想主动推荐给读者的内容。

最直接的解法是"不发布"，但这样它们就完全不可访问了。我想要的是：文章可以通过链接访问，但不出现在主页列表里。

于是有了"精选"这个概念：主页只显示被标记为精选的文章，其他文章仍然可以通过直链、主题页、标签页访问。

---

## 数据库层：加一个字段

最简单的部分。在 `posts` 表加一个 `is_featured` 布尔字段：

```sql
ALTER TABLE posts ADD COLUMN is_featured BOOLEAN NOT NULL DEFAULT FALSE;
CREATE INDEX idx_posts_is_featured ON posts (is_featured);
```

默认值是 `FALSE`——已有文章不会自动变成精选，需要手动标记。这是有意为之的：宁可主页暂时空着，也不要把不合适的内容推上去。

索引是必要的，因为主页查询会用 `WHERE is_featured = TRUE` 过滤，没有索引的话全表扫描在文章量大时会有问题。

这个改动作为增量迁移文件 `v2.sql` 存放，不修改主 schema 的全量 SQL——这是项目的迁移约定：`yukilog.sql` 是建库用的，`version/vN.sql` 是增量变更。

---

## 后端：字段穿透四层

后端是 Rust + Axum + SeaORM，分四层：`entities` → `repo` → `service` → `handler`。新字段需要在每一层都有对应的处理。

**Entity 层**

SeaORM 的 entity 文件手动加字段：

```rust
#[sea_orm(column_name = "is_featured")]
pub is_featured: bool,
```

**Repo 层**

查询过滤器 `PostFilter` 加上可选的 `is_featured`：

```rust
pub struct PostFilter {
    pub status: Option,
    pub is_featured: Option,
    // ...
}
```

构建查询时，如果 `is_featured` 有值就加上过滤条件：

```rust
if let Some(featured) = filter.is_featured {
    query = query.filter(posts::Column::IsFeatured.eq(featured));
}
```

**Service 层**

Service 的 DTO 同样加上 `is_featured: Option`，透传给 repo。

**Handler 层**

公开 API 的查询参数解析加上这个字段：

```rust
pub struct PostListQuery {
    pub is_featured: Option,
    // ...
}
```

然后把它传进 `PostFilter`。管理端的创建和更新接口也加上 `is_featured` 字段，让管理员可以标记文章。

---

## 前端：类型和调用

前端是 Astro + TypeScript。

**类型定义**

`PostListParams` 接口加一个字段：

```ts
export interface PostListParams {
  page?: number;
  page_size?: number;
  sort?: 'created_at' | 'updated_at' | 'view_count';
  status?: PostStatus;
  is_featured?: boolean;
}
```

**首页 API 调用**

主页的 `index.astro` 在调用文章列表接口时加上 `is_featured: true`：

```ts
const response = await postsApi.list({
  page: 1,
  page_size: 20,
  status: "published",
  sort: currentSort,
  is_featured: true,
});
```

`URLSearchParams` 会把 `is_featured: true` 序列化成 `is_featured=true`，Rust 端的 `serde` 能正确解析 `Option`，两端对齐。

其他页面——主题页、标签页、搜索、归档——不传这个参数，所以它们仍然能看到所有已发布文章。精选过滤只作用于主页。

---

## 主页标题的调整

加了精选逻辑之后，主页文章列表的标题也跟着改了。

原来是：

```
最新 n 篇博客
```

改成：

```
精选 · 最新 n 篇博客
精选 · 最热 n 篇博客
精选 · 最近更新 n 篇博客
```

"精选"作为前缀，说明这个列表是经过筛选的，不是全部文章。排序选项（最新 / 最热 / 最近更新）在精选范围内生效，逻辑没有变化。

---

## 一个细节：迁移顺序

部署时有一个需要注意的地方：`version/applied.txt` 记录了已执行的迁移文件名。如果服务器上没有这个文件，部署脚本会认为所有迁移都未执行，包括 `v1.sql`。

`v1.sql` 如果包含 `DROP TABLE` 或破坏性操作，重复执行会出问题。所以在首次运行增量迁移之前，需要确认 `applied.txt` 里已经记录了所有历史版本。

这次部署的实际结果：`v2.sql` 执行了 `ALTER TABLE posts ADD COLUMN is_featured` 和 `CREATE INDEX`，两个服务重启成功，主页只显示精选文章。

---

## 小结

`is_featured` 是一个很小的字段，但它穿透了整个技术栈：数据库 schema、迁移文件、SeaORM entity、repo 过滤器、service DTO、handler 查询参数、前端类型定义、API 调用。每一层都需要改动，缺一不可。

这种"一个字段的全栈旅程"在分层架构里很常见。好处是每一层的职责清晰，坏处是改动面广——加一个布尔值，改了七八个文件。
