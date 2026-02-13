<div align="center">

# YukiLog 工具函数与类型文档

`src/lib` 下的工具模块和 `src/types` 下的类型定义

</div>

---

## 目录

| 模块 | 源码 | 说明 |
| --- | --- | --- |
| [api.ts](#api) | `src/lib/api.ts` | API 封装（详见 [api.md](./api.md)） |
| [auth.ts](#auth) | `src/lib/auth.ts` | JWT 认证管理 |
| [admin-guard.ts](#admin-guard) | `src/lib/admin-guard.ts` | 管理端路由守卫 |
| [config.ts](#config) | `src/lib/config.ts` | 配置兼容导出层 |
| [markdown.ts](#markdown) | `src/lib/markdown.ts` | Markdown 渲染引擎 |
| [utils.ts](#utils) | `src/lib/utils.ts` | 通用工具函数 |
| [date.ts](#date) | `src/lib/date.ts` | 时间格式化 |
| [avatar.ts](#avatar) | `src/lib/avatar.ts` | 评论头像生成 |
| [slugify.ts](#slugify) | `src/lib/slugify.ts` | Slug 生成 |
| [svg-icons.ts](#svg-icons) | `src/lib/svg-icons.ts` | 图标管理（详见 [config.md](./config.md#svg-icons)） |
| [类型定义](#types) | `src/types/` | TypeScript 类型 |

---

<a id="auth"></a>

## auth.ts — JWT 认证管理

源码: [src/lib/auth.ts](../src/lib/auth.ts)

管理端 JWT Token 的存取, 基于 `localStorage`

**导出函数：**

| 函数 | 说明 |
| --- | --- |
| `setToken(token, expiresIn)` | 保存 JWT + 计算过期时间戳 |
| `getToken()` | 获取 JWT（过期则自动清除并返回 null） |
| `clearToken()` | 清除 token + 过期时间 |
| `isAuthenticated()` | 检查是否有有效 token |
| `isTokenExpiring()` | 判断 token 是否即将过期（剩余 < 5 分钟） |
| `getTokenTimeLeft()` | 获取 token 剩余有效秒数 |

**存储键值：**

```
localStorage:
  yukilog_token        = <JWT 字符串>
  yukilog_token_expiry = <过期时间戳（毫秒）>
```

---

<a id="admin-guard"></a>

## admin-guard.ts — 管理端路由守卫

源码: [src/lib/admin-guard.ts](../src/lib/admin-guard.ts)

| 函数 | 说明 |
| --- | --- |
| `guardAdminRoute()` | 检查登录状态, 未登录则跳转 `/admin/login` |
| `logout()` | 清除 token → 跳转登录页 |

---

<a id="config"></a>

## config.ts — 配置兼容导出层

源码: [src/lib/config.ts](../src/lib/config.ts)

仅 re-export `yukilog.config.ts` 中的配置, 便于现有代码通过短路径引用:

```typescript
import { siteConfig } from '../lib/config';
// 等价于
import { siteConfig } from '../../yukilog.config';
```

同时导出 `API_BASE_URL`（运行时环境变量）

---

<a id="markdown"></a>

## markdown.ts — Markdown 渲染引擎

源码: [src/lib/markdown.ts](../src/lib/markdown.ts)

**核心函数: `renderMarkdown(content: string): Promise<string>`**

将 Markdown 文本转换为 HTML, 集成:

| 功能 | 实现 |
| --- | --- |
| 基础 Markdown | `marked` (GFM) |
| 代码高亮 | `shiki`（github-light 主题） |
| 数学公式 | `katex`（`$...$` 行内, `$$...$$` 块级） |
| 脚注 | `marked-footnote` |
| 标题锚点 | 自定义 renderer, 生成 `id` 属性 |
| 标题前缀 | 从 `contentConfig.markdown.headingPrefixes` 读取 emoji 前缀 |

**支持的语言高亮：** javascript, typescript, rust, css, scss, html, markdown, yaml, json, bash, shell, python, sql, toml, vue, jsx, tsx

**Shiki 使用单例模式：** `_highlighter` 全局缓存, 首次调用时初始化

---

<a id="utils"></a>

## utils.ts — 通用工具函数

源码: [src/lib/utils.ts](../src/lib/utils.ts)

| 函数 | 说明 |
| --- | --- |
| `formatDate(dateStr, format)` | 格式化日期（`'full'` / `'short'` / `'relative'`） |
| `getSiteUptime(startDate)` | 计算站点运行时长（X 年 X 个月） |
| `getGravatarUrl(email, size)` | 生成 Gravatar 头像 URL |
| `truncate(text, maxLength)` | 截断文本 |
| `sleep(ms)` | 延迟 Promise |

---

<a id="date"></a>

## date.ts — 时间格式化

源码: [src/lib/date.ts](../src/lib/date.ts)

| 函数 | 说明 |
| --- | --- |
| `getRelativeTime(isoString)` | ISO 时间戳 → 相对时间（"3 分钟前"） |
| `formatDateTime(isoString)` | ISO 时间戳 → `YYYY-MM-DD HH:mm:ss` |

---

<a id="avatar"></a>

## avatar.ts — 评论头像

源码: [src/lib/avatar.ts](../src/lib/avatar.ts)

**`getCommentAvatar(website, email): string`**

头像优先级:
1. 有 website → `${website}/favicon.ico`
2. 有 email → Gravatar
3. 都没有 → 默认灰色头像

---

<a id="slugify"></a>

## slugify.ts — Slug 生成

源码: [src/lib/slugify.ts](../src/lib/slugify.ts)

**`slugify(title: string): string`**

将中文标题转换为 URL-friendly 的 slug:

1. 中文词汇 → 预定义拼音映射表（"前端" → "frontend", "教程" → "tutorial" 等）
2. 单个中文字符 → 拼音
3. 英文/数字 → 保留
4. 特殊字符 → 移除
5. 空格 → 短横线

---

<a id="types"></a>

## 类型定义 src/types/

### api.ts — API 响应类型

源码: [src/types/api.ts](../src/types/api.ts)

完整对应后端响应结构:

| 类型 | 说明 |
| --- | --- |
| `ApiResponse<T>` | 统一响应格式 `{ success, data, message }` |
| `PaginatedData<T>` | 分页数据 `{ items, total, page, page_size, total_pages }` |
| `Theme` | 主题 |
| `Tag` | 标签 |
| `Post` / `PostWithRelations` | 文章（及关联数据） |
| `Comment` / `CommentNode` | 评论（及评论树） |
| `Link` | 友链 |
| `LoginRequest` / `LoginResponse` | 登录 |
| `CreatePostRequest` / `UpdatePostRequest` | 文章 CRUD DTO |
| `CreateCommentRequest` / `UpdateCommentRequest` | 评论 CRUD DTO |
| `SubmitLinkRequest` / `UpdateLinkRequest` | 友链 CRUD DTO |
| `CreateThemeRequest` / `UpdateThemeRequest` | 主题 CRUD DTO |
| `CreateTagRequest` / `UpdateTagRequest` / `MergeTagsRequest` | 标签 CRUD DTO |
| `SearchQuery` / `SearchResponse` | 搜索 |

### blog.ts — 前端专属类型

源码: [src/types/blog.ts](../src/types/blog.ts)

| 类型 | 说明 |
| --- | --- |
| `NavItem` | 导航项 `{ label, href, icon }` |
| `SocialLink` | 社交链接 `{ name, url, icon, color }` |
| `SiteConfig` | 站点配置（详见 [config.md](./config.md#yukilog-config)） |
| `NavState` | 导航栏状态 `'hidden' \| 'visible' \| 'sticky'` |
| `ThemeMode` | 主题模式 `'light' \| 'dark' \| 'auto'` |
| `ArchiveYear` / `ArchivePost` | 归档数据（按年份分组） |
| `TagCloud` | 标签云数据 `{ name, slug, count, size }` |
| `PostCardData` | 文章卡片数据 |

### index.ts — 统一导出

源码: [src/types/index.ts](../src/types/index.ts)

将 `api.ts` 和 `blog.ts` 的所有类型统一 re-export, 使用时:

```typescript
import type { Post, NavItem, SiteConfig } from '../types';
```
