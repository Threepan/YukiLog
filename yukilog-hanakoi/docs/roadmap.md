<div align="center">

# YukiLog 后续功能规划

前端待开发的功能清单, 按优先级排列

</div>

---

## 目录

| 优先级 | 功能 | 状态 |
| --- | --- | --- |
| 🔴 P0 | [首页文章列表分页与排序](#home-pagination) | 待开发 |
| 🔴 P0 | [懒加载无限滚动](#lazy-load) | 待开发 |
| 🟡 P1 | [主题详情页分页](#theme-pagination) | 待开发 |
| 🟡 P1 | [标签筛选页分页](#tag-pagination) | 待开发 |
| 🟡 P1 | [归档页分页/折叠优化](#archive-optimization) | 待开发 |
| 🟡 P1 | [评论区 Toast 提示替代 alert](#toast) | 待开发 |
| 🟢 P2 | [暗色模式](#dark-mode) | 待开发 |
| 🟢 P2 | [真实 Gravatar MD5 哈希](#gravatar-md5) | 待开发 |
| 🟢 P2 | [管理后台 alert 替换为 UI 组件](#admin-ui) | 待开发 |
| 🟢 P2 | [一键部署脚本](#deploy-script) | 待开发 |

---

<a id="home-pagination"></a>

## 首页文章列表 — 分页与排序

**现状：** 首页一次性加载最新 20 篇文章, 无分页、无排序切换

**目标：** 支持客户端切换排序模式 + 分页浏览

### 后端接口（已就绪）

```
GET /api/public/posts
```

| 参数 | 类型 | 说明 |
| --- | --- | --- |
| `page` | `number` | 页码（从 1 开始） |
| `page_size` | `number` | 每页数量 |
| `sort` | `string` | 排序: `created_at` / `updated_at` / `view_count` |
| `theme_slugs` | `string` | 主题筛选（逗号分隔） |
| `tag_slugs` | `string` | 标签筛选（逗号分隔, AND 关系） |

### 实现方案

#### 1. 排序切换器

在文章列表顶部（"最近 N 篇博客" 旁边）添加排序切换按钮:

```text
最近 20 篇博客           [最新发布] [最近更新] [最多浏览]
```

| 按钮 | sort 值 | 说明 |
| --- | --- | --- |
| 最新发布 | `created_at` | 按创建时间倒序（默认） |
| 最近更新 | `updated_at` | 按最后修改时间倒序 |
| 最多浏览 | `view_count` | 按浏览量倒序 |

**交互：** 切换排序时, 客户端重新调用 API（fetch）, 替换文章列表内容, 无需刷新页面

#### 2. 分页控件

在文章列表底部（替换当前的"暂时只有这些了"）添加分页:

```text
← 上一页   第 1 / 5 页   下一页 →
```

**实现路径：**
* 客户端 fetch 方案: 用 JS 动态请求 `postsApi.list({ page, sort })`
* 服务端方案: URL 参数 `/?page=2&sort=view_count`, 每次整页 SSR

**推荐：** 客户端 fetch + DOM 替换（避免页面闪烁, 体验更好）

#### 3. 前端代码改动点

| 文件 | 改动 |
| --- | --- |
| `src/components/home/ArticleList.astro` | 添加排序切换器 UI + 分页控件 |
| `src/components/home/ArticleList.astro` `<script>` | 添加 fetch 逻辑: 排序切换/翻页时请求 API, 重新渲染卡片 |
| `src/components/home/ArticleCard.astro` | 可能需要拆出 `renderCard()` 模板函数供 JS 动态渲染 |
| `src/pages/index.astro` | 初始 page_size 可能需调整 |

---

<a id="lazy-load"></a>

## 懒加载无限滚动

**现状：** 文章列表一次加载固定数量, 到底部显示"暂时只有这些了"

**目标：** 滚动到底部时自动加载下一页, 直到没有更多数据

### 实现方案

```text
滚动到底部
    ↓
IntersectionObserver 触发 load-more-sentinel
    ↓
fetch postsApi.list({ page: nextPage, sort: currentSort })
    ↓
将新文章卡片 append 到列表
    ↓
更新 nextPage++
    ↓
如果 page >= total_pages → 显示"没有更多了"
```

**关键点：**
* 复用现有 `#load-more-sentinel` 元素作为 Observer 触发点
* 需维护 `currentPage` / `totalPages` / `currentSort` 状态
* 排序切换时应清空列表、重置页码、重新加载
* Loading 状态: sentinel 显示 spinner

---

<a id="theme-pagination"></a>

## 主题详情页分页

**现状：** `/themes/:slug` 页面一次加载该主题下所有文章

**目标：** 分页显示, 每页 10-20 篇

### 后端接口

```
GET /api/public/posts?theme_slugs=tech&page=1&page_size=10
```

已支持, 无需后端改动

### 改动点

| 文件 | 说明 |
| --- | --- |
| `src/pages/themes/[slug].astro` | 读取 URL 参数 `?page=N`, 传入 API 调用 |
| 同上 | 渲染分页控件（上下页 + 页码） |

---

<a id="tag-pagination"></a>

## 标签筛选页分页

**现状：** `/tags` 页面加载 999 篇文章, 前端按标签筛选

**目标：** 改用后端 `tag_slugs` 参数筛选, 支持分页

### 后端接口

```
GET /api/public/posts?tag_slugs=rust&page=1&page_size=10
```

已支持

### 改动点

| 文件 | 说明 |
| --- | --- |
| `src/pages/tags.astro` | 点击标签时用 API 按 tag_slug 查询, 而非前端全量筛选 |
| 同上 | 添加分页控件 |

---

<a id="archive-optimization"></a>

## 归档页优化

**现状：** 加载 999 篇文章, 前端按年份分组

**可选优化：**
1. 分页加载（每次加载一年的数据）
2. 年份折叠/展开动画
3. 文章数量多时虚拟滚动

---

<a id="toast"></a>

## Toast 提示替代 alert()

**现状：** 前台评论提交、友链申请等使用 `alert()` 提示

**目标：** 实现一个轻量 Toast 组件, 替代所有公开页面的 `alert()`

### 需要替换的位置（前台 4 处）

| 文件 | 场景 |
| --- | --- |
| `CommentForm.astro` | 评论提交成功/失败 |
| `CommentSection.astro` | 回复提交成功/失败 |
| `ReplyModal.astro` | 回复提交成功/失败 |
| `ApplyLinkModal.astro` | 友链申请成功/失败 |

### Toast 组件设计

```text
┌──────────────────────────┐
│ ✅ 评论已提交，审核通过后显示 │  ← 顶部浮动, 3s 后自动消失
└──────────────────────────┘
```

* 位置: 页面顶部居中
* 类型: success / error / info
* 动画: 从上方滑入 → 停留 3s → 淡出
* 使用 `variables.scss` 中的功能色 (`$lian-success` / `$lian-error` / `$lian-info`)

---

<a id="dark-mode"></a>

## 暗色模式

**现状：** 仅有亮色模式, `ThemeMode` 类型已定义但未使用

**目标：** 支持亮色/暗色/跟随系统三种模式

### 实现路径

1. 在 `variables.scss` 中定义暗色变量集
2. 通过 `data-theme="dark"` 属性切换
3. 在 NavBar 右侧添加主题切换按钮（已有 `light` 图标）
4. 用 `localStorage` 持久化用户选择
5. 用 `prefers-color-scheme` 媒体查询实现跟随系统

---

<a id="gravatar-md5"></a>

## 真实 Gravatar MD5

**现状：** `getGravatarUrl()` 使用 `simpleHash` 代替 MD5, 生成的 URL 无法匹配真实 Gravatar 头像

**目标：** 使用正确的 MD5 哈希

**方案：** 引入轻量 MD5 库（如 `blueimp-md5`, ~1KB gzipped）或使用 Web Crypto API

---

<a id="admin-ui"></a>

## 管理后台 UI 改进

**现状：** 管理后台大量使用 `alert()` / `confirm()` 进行操作反馈

**目标：**
1. 操作确认: 用自定义 Modal 替代 `confirm()`
2. 操作反馈: 用 Toast 替代 `alert()`
3. 列表操作: 添加 Loading 状态

---

<a id="deploy-script"></a>

## 一键部署脚本

**目标：** 提供 shell 脚本或 docker-compose 方案, 一键启动完整服务

### 组件

| 服务 | 说明 |
| --- | --- |
| PostgreSQL | 数据库 + 初始化 SQL |
| Redis | 限流缓存 |
| yukilog-backend | Rust Axum 后端 |
| yukilog-hanakoi | Astro Node.js 前端 |

### 方案选择

| 方案 | 优点 | 缺点 |
| --- | --- | --- |
| `docker-compose` | 一键启动, 环境隔离 | 需要 Docker |
| Shell 脚本 | 无额外依赖 | 需要手动安装 Rust/Node/PG/Redis |
| systemd units | 生产级管理 | 仅限 Linux |
