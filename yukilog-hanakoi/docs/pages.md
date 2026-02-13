<div align="center">

# YukiLog 前台页面文档

每个页面的用途、数据来源、使用的组件, 以及它里面的交互逻辑

</div>

---

## 目录

| 页面 | 路由 | 渲染模式 |
| --- | --- | --- |
| [首页](#home) | `/` | SSR |
| [文章详情](#post-detail) | `/posts/:slug` | SSR |
| [主题列表](#themes) | `/themes` | SSR |
| [主题详情](#theme-detail) | `/themes/:slug` | SSR |
| [标签](#tags) | `/tags` | SSR |
| [归档](#archive) | `/archive` | SSR |
| [友链](#links) | `/links` | SSR |
| [关于](#about) | `/about` | SSG |
| [404](#error-404) | `/404` | SSG |
| [500](#error-500) | `/500` | SSG (fallback) |

> **渲染模式**：SSR 页面标记了 `export const prerender = false`，由 Node.js 适配器处理；SSG 页面在构建时生成。

---

<a id="home"></a>

## 首页 `/`

源码: [src/pages/index.astro](../src/pages/index.astro)

**概述：** 双屏设计——左侧文章卡片流, 右侧个人信息面板

**数据来源：**

```typescript
postsApi.list({ page: 1, page_size: 7, status: 'published' })
```

API 失败时 → 重定向到 `/500`

**使用的组件：**

| 组件 | 位置 | 说明 |
| --- | --- | --- |
| `BlogLayout` | 布局 | 带导航栏、页脚、搜索、滚动进度 |
| `MusicPlayer` | 浮动 | 音乐播放器 |
| `WelcomeCard` | 左列顶部 | 引述文案（来自 `contentConfig.components.welcomeCard`） |
| `ArticleList` | 左列 | 文章卡片列表（7 篇） |
| `ProfileCard` | 右列 | 个人信息面板 |
| `HitokotoCard` | 右列 | 一言卡片（外部 API） |
| `SiteInfoCard` | 右列 | 站点运行信息 |

**客户端行为：**
* 文章列表底部无分页, 显示"暂时只有这些了"
* 点击文章卡片 → `/posts/:slug`

---

<a id="post-detail"></a>

## 文章详情 `/posts/:slug`

源码: [src/pages/posts/\[slug\].astro](../src/pages/posts/[slug].astro)

**概述：** 文章正文、目录、作者面板、评论区

**数据来源：**

```typescript
postsApi.getBySlug(slug)           // 文章数据
commentsApi.getPostComments(slug)  // 评论树
postsApi.incrementView(slug)       // 浏览计数（fire & forget）
```

文章不存在 → 重定向到 `/404`

**Markdown 渲染：** 使用 `renderMarkdown()` 将 `post.content` 转换为 HTML

* Shiki 代码高亮（github-light 主题）
* KaTeX 数学公式（`$...$` 行内, `$$...$$` 块级）
* marked-footnote 脚注
* 标题自动生成锚点 ID

**使用的组件：**

| 组件 | 说明 |
| --- | --- |
| `BlogLayout` | 带 `stickyNav` 模式 |
| `ProfileCard` | 右侧作者信息 |
| `TableOfContents` | 右侧目录（自动提取 h2-h4 标题） |
| `CommentSection` | 评论区（完整树形评论 + 回复表单） |

**客户端行为：**
* 目录联动：滚动时高亮当前标题
* 评论提交：调用 `POST /api/public/comments/:slug`
* 回复表单：内联展开, 本地 localStorage 草稿保存

---

<a id="themes"></a>

## 主题列表 `/themes`

源码: [src/pages/themes.astro](../src/pages/themes.astro)

**概述：** Hero 区 + 分类卡片散落式布局

**数据来源：**

```typescript
themesApi.list()
```

**使用的组件：**

| 组件 | 说明 |
| --- | --- |
| `PageHero` | 页面标题区域 |
| 内联 `.theme-card` | 主题卡片（CSS 微旋转 + 错位, 模拟散落感） |

**视觉特性：**
* 每张卡片通过 CSS 变量 `--rot` / `--offset` 实现确定性微旋转和水平错位
* 显示主题名称 + 文章计数

---

<a id="theme-detail"></a>

## 主题详情 `/themes/:slug`

源码: [src/pages/themes/\[slug\].astro](../src/pages/themes/[slug].astro)

**概述：** 指定主题下的文章列表

**数据来源：**

```typescript
themesApi.getBySlug(slug)
postsApi.list({ theme_slugs: slug })
themesApi.incrementView(slug)  // 浏览计数
```

**使用的组件：**

| 组件 | 说明 |
| --- | --- |
| `PageHero` | 显示主题名 + 描述 |
| `PostListCard` | 文章列表卡片 |

---

<a id="tags"></a>

## 标签 `/tags`

源码: [src/pages/tags.astro](../src/pages/tags.astro)

**概述：** 标签云 + 点击后展开该标签下的文章列表

**数据来源：**

```typescript
tagsApi.list('post_count')
postsApi.list({ page_size: 999, status: 'published' })
```

**视觉特性：**
* 标签按三色循环着色（pink / blue / white）, 顺序由 `contentConfig.pages.tags.colorCycle` 控制
* 点击标签 → 页面内筛选, 显示对应文章列表

---

<a id="archive"></a>

## 归档 `/archive`

源码: [src/pages/archive.astro](../src/pages/archive.astro)

**概述：** Hero + 时间轴（按年份分组, 时间倒序）

**数据来源：**

```typescript
postsApi.list({ page_size: 999, status: 'published' })
```

在前端按年份分组 (`yearMap`)

**视觉特性：**
* 左侧时间轴竖线 + 圆点节点
* 每年一个折叠组, 显示月/日 + 文章标题
* 底部"故事从这里开始"结尾语

---

<a id="links"></a>

## 友链 `/links`

源码: [src/pages/links.astro](../src/pages/links.astro)

**概述：** 信笺风格——特别关注区 + 友链卡片网格 + 申请弹窗

**数据来源：**

```typescript
linksApi.list()  // 过滤 status === 'active'
```

**使用的组件：**

| 组件 | 说明 |
| --- | --- |
| `FriendSpotlight` | 特别关注位（最好的朋友, 配置在 `contentConfig.pages.links.bestFriend`） |
| `FriendCard` | 普通友链卡片 |
| `ApplyLinkModal` | 申请友链弹窗（提交调用 `linksApi.submit()`） |

---

<a id="about"></a>

## 关于 `/about`

源码: [src/pages/about.astro](../src/pages/about.astro)

**概述：** 个人介绍页, 静态生成

**特性：**
* 作者信息来自 `siteConfig.author`
* 社交链接来自 `siteConfig.social`
* 对话体排版（左右分栏: "代码与理性" / "情绪与挣扎"）

---

<a id="error-404"></a>

## 404 页面

源码: [src/pages/404.astro](../src/pages/404.astro)

**概述：** "页面走失了" 错误页, 静态生成

文案来自 `contentConfig.pages.error404`

---

<a id="error-500"></a>

## 500 页面

源码: [src/pages/500.astro](../src/pages/500.astro)

**概述：** "服务器开小差了" 错误页, 作为 API 失败时的兜底

文案来自 `contentConfig.pages.error500`, 包含可能的原因列表（后端未启动 / 数据库连接失败 / Redis 不可用等）
