---
id: 18
title: "YukiLog - 13 - 随记系统：从「再加一个表」到独立内容形态"
slug: "yukilog-13-notes"
summary: "博客文章太重了——标题、slug、摘要、封面图、主题分类、标签、评论、阅读计数。有些想法只想写三行字配个心情。于是我给 YukiLog 加了一种新的内容类型"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-12-notes.jpg"
status: "published"
is_featured: true
theme: "博客搭建 (bkdj)"
tags: ["astro", "blog", "psql", "rust"]
view_count: 468
created_at: "2026-03-17 00:05:28"
updated_at: "2026-08-17 15:50:37"
source: "https://blog.yeastar.xin/posts/yukilog-13-notes"
---

## 起因

写博客有一个门槛问题。

不是技术门槛，是心理门槛。每次点击"新建文章"，面前是一堆要填的字段：标题、slug、摘要、封面图、主题分类、标签选择。然后是正文——既然都填了这么多元信息了，正文总不能只写三行吧？

于是很多想法就死在了"打开编辑器"这一步。

推特和微信朋友圈之所以能让人写东西，核心不是社交属性，而是**输入成本低**——打开，写，发。没有标题，没有分类，没有"这篇文章应该放在哪个主题下"的焦虑。

我想给 YukiLog 加一个类似的东西。不是社交功能，只是一种更轻的内容形态。

---

## 为什么不复用 posts 表

最先想到的方案是在 `posts` 表上做减法：标题设为可空，slug 自动生成，摘要留空。

但写到 repo 层的查询时我就后悔了。

`posts` 表有 14 个字段，关联着 `post_tags` 中间表和 `comments` 评论表，首页查询带 `is_featured` 过滤，管理端查询支持按主题、标签、状态三维筛选。如果把「随记」塞进来，每个查询都要多加一个 `WHERE type != 'note'` 或者 `WHERE type = 'post'`——而且是每个查询，包括已经写好的十几个。

漏掉一个就是 bug。

另一个问题是 URL 模型。文章用 `/posts/:slug` 定位，slug 是语义化的字符串（`rust-web-backend`）。随记没有标题，造不出有意义的 slug，只能用 `/notes/:id`。两种 URL 模型混在一张表里，路由层也要做分支判断。

领域模型不同的东西就该放在不同的表：


| 维度 | posts 文章                                    | notes 随记                                        |
| ---- | --------------------------------------------- | ------------------------------------------------- |
| 字段 | 14 个（含 title/slug/summary/cover_image 等） | 5 个（content/mood/status/created_at/updated_at） |
| URL  | `/posts/:slug`                                | `/notes/:id`                                      |
| 分类 | theme + tags 两级                             | 无分类，纯时间流                                  |
| 交互 | 评论、浏览计数                                | 无                                                |

一张新表，几个新文件，换来的是**查询干净、职责清晰、不会误伤已有功能**。

---

## 数据库：极简表结构

```sql
CREATE TABLE IF NOT EXISTS notes (
    id          BIGSERIAL PRIMARY KEY,
    content     TEXT NOT NULL,
    mood        VARCHAR(20),
    status      VARCHAR(20) DEFAULT 'published',
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

五个字段。和 `posts` 的十四个字段比，这就是「轻量内容形态」的态度。

`mood` 是可选的心情标记，存成字符串。没用 PostgreSQL 的 `ENUM` 类型——加新心情时 `ALTER TYPE` 的语法比改应用代码麻烦。后端也没有 Rust 枚举校验，mood 字段直接透传写入数据库，合法值完全由前端 config 的预设查询表（`moodLabels`）定义。想加新情绪只需在 config 里加一行，不需要动任何后端代码，不需要迁移数据。

`status` 比文章多了一个 `private` 状态——有些日记只想自己看。三种状态：`published`（公开）、`draft`（草稿）、`private`（私密）。

索引方面只建了两个：

```sql
CREATE INDEX idx_notes_status_created_at ON notes (status, created_at DESC);
CREATE INDEX idx_notes_created_at ON notes (created_at DESC);
```

第一个是复合索引，覆盖"公开随记按时间倒序"这个最常见的查询。第二个是纯时间排序索引，管理端不带 status 过滤时用。

---

## 后端：熟悉的四层穿透

和第 12 篇写精选文章时一样，一个新功能要穿透 entities → repo → service → handler 四层。但随记因为没有关联表，每层都比文章简单。

**Entity 层**——直接映射表结构，没有 `Relation`（文章有 theme、post_tags、comments 三种关联，随记一个都没有）。

**Domain 层**——新增 `NoteStatus` 枚举，和 `PostStatus` 类似但多了 `Private` 变体。mood 字段没有对应的枚举——它就是一个 `Option`，直接透传，不做任何校验。情绪的合法值完全由前端 config 定义，后端不关心。

**Repo 层**——标准 CRUD 加分页。吸取了之前用 `.len()` 算总数的教训（TODO.md 里记录的 P1 问题），分页计数也改用独立的 `SELECT COUNT(*)` 查询：

```rust
pub async fn count_notes(db: &DbConn, status: Option) -> RepoResult {
    let mut query = Notes::find();
    if let Some(s) = status {
        query = query.filter(notes::Column::Status.eq(s.as_str()));
    }
    query.count(db).await.map_err(RepoError::from)
}
```

**Service 层**——公开接口和管理接口的区别在于 status 过滤：公开端只返回 `published`，管理端可以看到所有状态。一个 repo 函数，两个 service 函数，过滤条件不同。

**Handler 层**——公开 2 个 API（列表 + 详情），管理 4 个 API（列表 + 创建 + 更新 + 删除）。路由注册在 `handler/mod.rs` 里加 6 行。

整个后端的增量：新增 6 个文件，修改 9 个文件。`cargo check` 通过即上线。

---

## 前端：时间线与无限滚动

随记列表不适合分页器——你不会想点"第 3 页"来看三天前的想法。自然的交互是**滚到底自动加载更多**。

服务端渲染第一页（10 条），客户端接管后续分页：

```svelte
let notes = $state([...initialNotes]);
let page = $state(1);
let hasMore = $derived(page < totalPages);

async function loadMore() {
    if (loading || !hasMore) return;
    loading = true;
    const res = await notesApi.list({ page: page + 1, page_size: 10 });
    notes = [...notes, ...res.items];
    page += 1;
    loading = false;
}
```

用 `IntersectionObserver` 监测一个底部哨兵元素，进入视口就触发 `loadMore()`。比监听 scroll 事件性能好，也不需要算什么"距离底部 200px"。

列表按日期分组——"今天"、"昨天"、"3月8日"——用 `$derived` 从 notes 数组实时计算分组，不需要额外状态。

单条随记的渲染更简单：Markdown 内容（复用 `marked` + `shiki` 管线）、心情 emoji（如果有的话）、相对时间。没有标题栏，没有封面图，没有标签云——就是一段文字。

---

## mood 设计

心情标记是可选的。我不想让"选一个心情"变成和"填一个标题"一样的负担。

管理端用一个可搜索的 combobox：输入框可以直接打字过滤，也可以从预设列表里点选，还可以输入完全自定义的 key——不在列表里的值也能保存。

后端不做枚举校验，mood 字段就是一个普通字符串，直接透传写入数据库。预设的查询表只存在于前端 config 文件（`yukilog.config.ts` 里的 `moodLabels`），维护一张 `key → emoji + 标签` 的映射。想加新心情只需要在 config 里加一行，不需要动后端，不需要迁移数据。

预设了约 49 种情绪，从日常的「😊 开心」「😴 疲惫」，到更细腻的「🫥 空洞」「🌫️ 漂浮」「🧊 抽离」「🌑 寂静」。combobox 支持搜索过滤，所以列表长度不是负担——打两个字母就能定位。

---

## 管理端

随记的管理界面比文章简单得多。文章编辑器用了 Vditor（一个重量级的 Markdown 编辑器，CDN 加载约 1.5MB），随记不需要——一个 `` 加实时预览就够了。

列表页支持按状态筛选（全部/已发布/草稿/私密），创建和编辑在同一页面内完成，不需要跳转。mood 字段用一个可搜索的 combobox：打字过滤预设列表，也可以直接输入不在列表里的自定义 key。

整个管理端的代码量大概是文章管理的三分之一。

---

## 回头看

加一个新的内容类型，工作量其实不大——后端是机械性的四层穿透，前端是已有组件的组合。真正花时间的是决策：独立表还是复用表、要不要分类系统、心情标记的枚举校验放在哪一层（最终答案是：不放，后端透传，前端 config 维护查询表）、管理端用什么编辑器。

这些决策一旦确定，编码就是执行计划。

下一篇讲另一件事：把整个前端从 Astro 迁移到 SvelteKit。这个工作量就不是"加一个表"能概括的了。
