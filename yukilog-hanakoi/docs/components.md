<div align="center">

# YukiLog 组件文档

所有可复用 Astro 组件的职责、Props、内部交互

</div>

---

## 目录

| 分类 | 组件 |
| --- | --- |
| 布局 | [BaseLayout](#baselayout) / [BlogLayout](#bloglayout) / [AdminLayout](#adminlayout) |
| 导航 | [NavBar](#navbar) / [NavItem](#navitem) |
| 首页 | [ProfileCard](#profilecard) / [WelcomeCard](#welcomecard) / [HitokotoCard](#hitokotocard) / [SiteInfoCard](#siteinfocard) / [ArticleCard](#articlecard) / [ArticleList](#articlelist) |
| 评论 | [CommentSection](#commentsection) / [CommentList](#commentlist) / [CommentItem](#commentitem) / [CommentForm](#commentform) / [ReplyModal](#replymodal) |
| 友链 | [FriendSpotlight](#friendspotlight) / [FriendCard](#friendcard) / [ApplyLinkModal](#applylinkmodal) |
| 文章 | [TableOfContents](#tableofcontents) |
| 共享 | [PageHero](#pagehero) / [PostListCard](#postlistcard) / [Footer](#footer) / [SEO](#seo) / [ScrollProgress](#scrollprogress) / [SearchOverlay](#searchoverlay) / [MusicPlayer](#musicplayer) |

---

## 布局组件

<a id="baselayout"></a>

### BaseLayout

源码: [src/layouts/BaseLayout.astro](../src/layouts/BaseLayout.astro)

**职责：** 所有页面的根布局, 提供 `<html>` / `<head>` / `<body>` 骨架

**Props：**

| 属性 | 类型 | 说明 |
| --- | --- | --- |
| `title` | `string` | 页面标题 |
| `description?` | `string` | SEO 描述 |
| `keywords?` | `string[]` | SEO 关键词 |
| `ogImage?` | `string` | Open Graph 图片 |

**Slots：** `default` / `head`（注入额外 `<head>` 内容）

**内置行为：** 页面失焦时标题变为"...你 ... 要走了吗?"

---

<a id="bloglayout"></a>

### BlogLayout

源码: [src/layouts/BlogLayout.astro](../src/layouts/BlogLayout.astro)

**职责：** 带导航栏 + 页脚的标准博客页面布局

**构成：** `BaseLayout` > `NavBar` + `SearchOverlay` + `ScrollProgress` + `<main>` + `Footer`

**额外 Props：**

| 属性 | 类型 | 默认 | 说明 |
| --- | --- | --- | --- |
| `stickyNav?` | `boolean` | `false` | 是否让导航栏直接以 sticky 岛屿模式展示 |

---

<a id="adminlayout"></a>

### AdminLayout

源码: [src/layouts/AdminLayout.astro](../src/layouts/AdminLayout.astro)

**职责：** 管理后台布局（侧边栏 + 顶栏 + 内容区）

**Props：**

| 属性 | 类型 | 说明 |
| --- | --- | --- |
| `title` | `string` | 当前页标题 |
| `activeMenu?` | `string` | 高亮的侧边栏菜单项 |

详见 [管理后台页面文档](./pages-admin.md#adminlayout)

---

## 导航组件

<a id="navbar"></a>

### NavBar

源码: [src/components/navigation/NavBar.astro](../src/components/navigation/NavBar.astro)

**职责：** 顶部导航栏, 支持滚动隐藏/显示 + 岛屿模式

**导航项：** 从 `navItems`（`yukilog.config.ts`）读取

**状态机：**
* 页面顶部 → 透明宽幅
* 向下滚动 → 隐藏
* 向上滚动 → 显示为悬浮岛屿（圆角 + 阴影）
* `stickyOnly` 模式 → 直接以岛屿形态展示

**客户端功能：**
* 搜索快捷键 `Ctrl/Cmd + K` → 打开 `SearchOverlay`
* 品牌图标点击 → 回首页

---

<a id="navitem"></a>

### NavItem

源码: [src/components/navigation/NavItem.astro](../src/components/navigation/NavItem.astro)

**职责：** 单个导航项, 支持 SVG 图标 + 文字

---

## 首页组件

<a id="profilecard"></a>

### ProfileCard

源码: [src/components/home/ProfileCard.astro](../src/components/home/ProfileCard.astro)

**职责：** 个人信息面板（头像 + 名字 + Bio + 社交链接）

**数据来源：** `siteConfig.author` + `siteConfig.social`

---

<a id="welcomecard"></a>

### WelcomeCard

源码: [src/components/home/WelcomeCard.astro](../src/components/home/WelcomeCard.astro)

**职责：** 首页欢迎语卡片, 带引号装饰

**数据来源：** `contentConfig.components.welcomeCard.quoteText`

---

<a id="hitokotocard"></a>

### HitokotoCard

源码: [src/components/home/HitokotoCard.astro](../src/components/home/HitokotoCard.astro)

**职责：** 一言卡片, 从外部 API `https://v1.hitokoto.cn` 获取随机名言

**客户端行为：** 点击刷新按钮可重新获取

---

<a id="siteinfocard"></a>

### SiteInfoCard

源码: [src/components/home/SiteInfoCard.astro](../src/components/home/SiteInfoCard.astro)

**职责：** 站点信息面板（主站链接 + GitHub 链接 + 运行时长计时器）

**数据来源：** `contentConfig.components.siteInfoCard` + `siteConfig.startDate`

**客户端行为：** `setInterval` 每秒更新"已运行 X 天 X 时 X 分 X 秒"

---

<a id="articlecard"></a>

### ArticleCard

源码: [src/components/home/ArticleCard.astro](../src/components/home/ArticleCard.astro)

**职责：** 首页文章卡片, 显示标题 + 摘要 + 主题标签 + 浏览量 + 日期

---

<a id="articlelist"></a>

### ArticleList

源码: [src/components/home/ArticleList.astro](../src/components/home/ArticleList.astro)

**职责：** 首页文章列表容器, 渲染多个 `ArticleCard`

---

## 评论组件

<a id="commentsection"></a>

### CommentSection

源码: [src/components/comments/CommentSection.astro](../src/components/comments/CommentSection.astro)

**职责：** 文章页评论区的完整容器

**构成：**
* 评论计数 + 标题
* `CommentList`（评论树）
* `CommentForm`（顶级评论表单）
* `ReplyModal`（移动端回复弹窗模板）

**内联回复逻辑：**

每条评论的"回复"按钮点击后, 在该评论下方动态插入回复表单:

1. 克隆 `.comment-form-template` 模板
2. 绑定字符计数、localStorage 草稿保存、提交事件
3. 提交时调用 `fetch(POST /api/public/comments/:slug)`, 传入 `parent_id`
4. 成功 → 清空表单 + 收起容器; 失败 → alert 提示

---

<a id="commentlist"></a>

### CommentList

源码: [src/components/comments/CommentList.astro](../src/components/comments/CommentList.astro)

**职责：** 递归渲染评论树（`CommentNode[]`）

---

<a id="commentitem"></a>

### CommentItem

源码: [src/components/comments/CommentItem.astro](../src/components/comments/CommentItem.astro)

**职责：** 单条评论（头像 + 昵称 + 时间 + 内容 + 回复按钮）

**头像生成：** 使用 `getCommentAvatar(website, email)` → 优先 website favicon → Gravatar → 默认头像

---

<a id="commentform"></a>

### CommentForm

源码: [src/components/comments/CommentForm.astro](../src/components/comments/CommentForm.astro)

**职责：** 评论/回复表单（昵称 + 邮箱 + 网站 + 内容 + 提交按钮）

**客户端行为：**
* 输入时自动保存到 localStorage（key: `comment-form-{postSlug}` 或 `comment-form-reply-{commentId}`）
* 字符计数实时更新
* 可折叠展开（节省首屏空间）
* 提交 → `POST /api/public/comments/:slug`（real API）

---

<a id="replymodal"></a>

### ReplyModal

源码: [src/components/comments/ReplyModal.astro](../src/components/comments/ReplyModal.astro)

**职责：** 移动端回复弹窗（桌面端使用内联回复, 移动端弹窗）

**打开方式：** `window.openReplyModal(commentId, commentNick)`

**提交逻辑：** 与 CommentSection 相同, 调用真实 API

---

## 友链组件

<a id="friendspotlight"></a>

### FriendSpotlight

源码: [src/components/links/FriendSpotlight.astro](../src/components/links/FriendSpotlight.astro)

**职责：** 特别关注位——"致最好的你"卡片, 信笺风格

**数据来源：** `contentConfig.pages.links.bestFriend`

---

<a id="friendcard"></a>

### FriendCard

源码: [src/components/links/FriendCard.astro](../src/components/links/FriendCard.astro)

**职责：** 普通友链卡片（头像 + 名称 + 描述）

---

<a id="applylinkmodal"></a>

### ApplyLinkModal

源码: [src/components/links/ApplyLinkModal.astro](../src/components/links/ApplyLinkModal.astro)

**职责：** 友链申请弹窗, 收集站点信息并提交

**提交：** `linksApi.submit({ title, url, avatar, description })`

**文案来源：** `contentConfig.components.applyLinkModal`

---

## 文章组件

<a id="tableofcontents"></a>

### TableOfContents

源码: [src/components/posts/TableOfContents.astro](../src/components/posts/TableOfContents.astro)

**职责：** 右侧浮动目录, 自动提取文章 h2-h4 标题

**客户端行为：**
* `IntersectionObserver` 监听标题元素
* 滚动时高亮当前可见标题
* 点击目录项平滑滚动到对应位置

---

## 共享组件

<a id="pagehero"></a>

### PageHero

源码: [src/components/shared/PageHero.astro](../src/components/shared/PageHero.astro)

**职责：** 通用页面 Hero 区域（图标 + 标题 + 副标题）

**Props：**

| 属性 | 类型 | 说明 |
| --- | --- | --- |
| `title` | `string` | 标题 |
| `subtitle?` | `string` | 副标题 |
| `icon?` | `string` | SVG 图标（raw HTML） |

---

<a id="postlistcard"></a>

### PostListCard

源码: [src/components/shared/PostListCard.astro](../src/components/shared/PostListCard.astro)

**职责：** 文章列表卡片（用于主题详情/标签筛选等场景）

---

<a id="footer"></a>

### Footer

源码: [src/components/shared/Footer.astro](../src/components/shared/Footer.astro)

**职责：** 页脚, 显示版权信息

---

<a id="seo"></a>

### SEO

源码: [src/components/shared/SEO.astro](../src/components/shared/SEO.astro)

**职责：** `<head>` 中的 SEO meta 标签（title / description / keywords / Open Graph）

---

<a id="scrollprogress"></a>

### ScrollProgress

源码: [src/components/shared/ScrollProgress.astro](../src/components/shared/ScrollProgress.astro)

**职责：** 页面滚动进度条 + 回到顶部按钮

---

<a id="searchoverlay"></a>

### SearchOverlay

源码: [src/components/shared/SearchOverlay.astro](../src/components/shared/SearchOverlay.astro)

**职责：** 全屏搜索遮罩层

**触发方式：** 导航栏搜索图标 / `Ctrl/Cmd + K`

**搜索调用：** `postsApi.search({ q, page, page_size })`

**交互：**
* 实时搜索（输入防抖）
* 键盘导航: ↑↓ 选择, Enter 打开, Esc 关闭
* 搜索结果高亮关键词（后端返回 `<mark>` 标签）
* 分页

---

<a id="musicplayer"></a>

### MusicPlayer

源码: [src/components/shared/MusicPlayer.astro](../src/components/shared/MusicPlayer.astro)

**职责：** 浮动音乐播放器（仅首页展示）
