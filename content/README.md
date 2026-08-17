# YukiLog 站点内容镜像

这个目录是 <https://blog.yeastar.xin/> 线上内容的完整本地镜像，抓取时间 **2026-08-17 (CST)**。

站点结构与本目录的对应关系：

| 站点栏目 | 线上地址 | 本地目录 |
| --- | --- | --- |
| 主页 | `/` | [`pages/home.md`](./pages/home.md) |
| 归档 | `/archive` | [`pages/archive.md`](./pages/archive.md) |
| 关于 | `/about` | [`pages/about.md`](./pages/about.md) |
| 文章 | `/posts/<slug>` | [`posts/`](./posts/README.md) |
| 随记 | `/notes` | [`notes/`](./notes/README.md) |
| 主题 | `/themes` | [`themes/`](./themes/README.md) |
| 标签 | `/tags` | [`tags/`](./tags/README.md) |
| 评论 | 各文章页评论区 | [`comments/`](./comments/README.md) |
| 友链 | `/links` | [`links/`](./links/README.md) |
| 原始 API 响应 | `/api/public/*` | [`_api/`](./_api/) |

## 内容规模

- 文章 **23** 篇（2026 年 22 篇，2025 年 1 篇）
- 随记 **12** 条
- 主题 **5** 个：博客搭建 / Linux系统 / 神经网络 / AI开发 / 计算机网络
- 标签 **30** 个
- 评论 **4** 条（分布在 4 篇文章上，其余 19 篇暂无评论）
- 友链 **6** 条
- 站点统计：`total_posts=23`、`total_views=9110`、`total_words=138735`

## 文件约定

- 每篇文章 / 随记都是一个 `.md` 文件，顶部为 YAML front-matter（元数据），下方为线上原始 Markdown 正文。
- 所有时间戳均已由接口返回的 UTC 转换为 **CST（UTC+8）**，格式 `YYYY-MM-DD HH:MM:SS`。
- 每个文件的 `source` 字段指向对应的线上页面地址。
- `_api/` 下保存了各公开接口的原始 JSON 响应（`notes` / `links` / `themes` / `tags` / `stats` / `comments`），可作为二次处理的数据源。
- 有评论的文章，其 front-matter 额外带 `comment_count` 字段；评论正文统一收在 `comments/`。

## 已知损耗

抓取通道会剥离正文中形如 `<T>` 的尖括号片段，因此少量文章里的 Rust / TypeScript 泛型参数（例如 `Option<String>`、`ApiResponse<T>`）与内联 HTML 标签在本地副本中缺失。文字内容本身完整。

## 相关目录

仓库根目录下的 [`../farewell-lianself/`](../farewell-lianself/) 是 <https://farewell.yeastar.xin/>（`Lianself` / `not for your eyes ...`）的逐字节离线克隆，与本目录相互独立。
