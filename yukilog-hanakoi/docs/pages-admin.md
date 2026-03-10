<div align="center">

# YukiLog 管理后台页面文档

所有管理页面都在 `/admin` 下, 需要 JWT 认证, 使用 `AdminLayout` 布局

</div>

---

## 目录

| 页面 | 路由 | 说明 |
| --- | --- | --- |
| [登录](#login) | `/admin/login` | 管理员登录 |
| [仪表盘](#dashboard) | `/admin` | 管理后台首页 |
| [文章管理](#posts) | `/admin/posts` | 文章列表 |
| [新建文章](#posts-new) | `/admin/posts/new` | Vditor 编辑器 |
| [编辑文章](#posts-edit) | `/admin/posts/edit/:slug` | 编辑已有文章 |
| [评论管理](#comments) | `/admin/comments` | 评论审核/删除 |
| [主题管理](#themes) | `/admin/themes` | 主题 CRUD |
| [标签管理](#tags) | `/admin/tags` | 标签 CRUD + 合并 |
| [友链管理](#links) | `/admin/links` | 友链审核/CRUD |
| [随记管理](#notes) | `/admin/notes` | 随记 CRUD |

> 所有管理页面均标记 `export const prerender = false`（SSR）
> 
> 每个页面的 `<script>` 起始处都调用 `guardAdminRoute()` 做登录检查

---

<a id="login"></a>

## 登录 `/admin/login`

源码: [src/pages/admin/login.astro](../src/pages/admin/login.astro)

**流程：**

1. 用户输入用户名 + 密码
2. 调用 `authApi.login({ username, password })`
3. 成功 → `setToken(token, expires_in)` → 跳转 `/admin`
4. 失败 → 显示错误提示

**特性：**
* 不使用 `AdminLayout`（无侧边栏）
* 已登录时自动跳转到 `/admin`

---

<a id="dashboard"></a>

## 仪表盘 `/admin`

源码: [src/pages/admin/index.astro](../src/pages/admin/index.astro)

**概述：** 管理后台首页, 展示各模块快捷入口

**使用的组件：** `AdminLayout`（侧边栏 + 顶栏）

---

<a id="posts"></a>

## 文章管理 `/admin/posts`

源码: [src/pages/admin/posts/index.astro](../src/pages/admin/posts/index.astro)

**功能：**
* 分页文章列表（`adminApi.posts.list()`）
* 按状态筛选（draft / published）
* 删除文章（`adminApi.posts.delete(slug)`）
* 点击进入编辑页

---

<a id="posts-new"></a>

## 新建文章 `/admin/posts/new`

源码: [src/pages/admin/posts/new.astro](../src/pages/admin/posts/new.astro)

**功能：**
* Vditor Markdown 编辑器（从 CDN 加载）
* 自动生成 slug（`slugify()` 中文拼音映射）
* 选择主题（下拉框, 数据来自 `themesApi.list()`）
* 选择标签（复选框, 数据来自 `tagsApi.list()`）
* 发布 / 存为草稿

**发布流程：**

```typescript
adminApi.posts.create({
  title, slug, content,
  summary, cover_image,
  status: 'published' | 'draft',
  theme_slug,    // 主题 slug 或 null
  tag_slugs: [], // 标签 slug 数组
})
```

---

<a id="posts-edit"></a>

## 编辑文章 `/admin/posts/edit/:slug`

源码: [src/pages/admin/posts/edit/\[slug\].astro](../src/pages/admin/posts/edit/[slug].astro)

**功能与新建基本一致**, 额外特性:
* 加载已有文章数据（`adminApi.posts.getBySlug(slug)`）
* 回填标题、slug、摘要、封面、内容、主题、标签
* 保存时调用 `adminApi.posts.update(slug, data)`

---

<a id="comments"></a>

## 评论管理 `/admin/comments`

源码: [src/pages/admin/comments.astro](../src/pages/admin/comments.astro)

**功能：**
* 评论列表（分页, 可按状态筛选）
* 待审核评论高亮
* 操作: 通过 / 拒绝 / 编辑 / 删除

| 操作 | API 调用 |
| --- | --- |
| 通过 | `adminApi.comments.approve(id)` |
| 拒绝 | `adminApi.comments.reject(id)` |
| 编辑 | `adminApi.comments.update(id, data)` |
| 删除 | `adminApi.comments.delete(id)` |

---

<a id="themes"></a>

## 主题管理 `/admin/themes`

源码: [src/pages/admin/themes.astro](../src/pages/admin/themes.astro)

**功能：** 主题的增/改/删

| 操作 | API 调用 |
| --- | --- |
| 新建 | `adminApi.themes.create({ name, slug, description })` |
| 编辑 | `adminApi.themes.update(id, data)` |
| 删除 | `adminApi.themes.delete(id)` |

---

<a id="tags"></a>

## 标签管理 `/admin/tags`

源码: [src/pages/admin/tags.astro](../src/pages/admin/tags.astro)

**功能：** 标签的增/改/删 + 标签合并

| 操作 | API 调用 |
| --- | --- |
| 新建 | `adminApi.tags.create({ name, slug })` |
| 编辑 | `adminApi.tags.update(id, data)` |
| 删除 | `adminApi.tags.delete(id)` |
| 合并 | `adminApi.tags.merge({ source_id, target_id })` |

**合并说明：** 将 source 标签下的所有文章转移到 target 标签, 然后删除 source

---

<a id="links"></a>

## 友链管理 `/admin/links`

源码: [src/pages/admin/links.astro](../src/pages/admin/links.astro)

**功能：** 友链的审核/CRUD

| 操作 | API 调用 |
| --- | --- |
| 列表 | `adminApi.links.list()` |
| 待审核 | `adminApi.links.pending()` |
| 通过 | `adminApi.links.approve(id)` |
| 标记失效 | `adminApi.links.markBroken(id)` |
| 新建 | `adminApi.links.create(data)` |
| 编辑 | `adminApi.links.update(id, data)` |
| 删除 | `adminApi.links.delete(id)` |

---

<a id="notes"></a>

## 随记管理 `/admin/notes`

源码: [src/pages/admin/notes.astro](../src/pages/admin/notes.astro)

**功能：** 随记的 CRUD 管理 + 状态筛选

| 操作 | API 调用 |
| --- | --- |
| 列表 | `adminApi.notes.list(params)` |
| 创建 | `adminApi.notes.create(data)` |
| 编辑 | `adminApi.notes.update(id, data)` |
| 删除 | `adminApi.notes.delete(id)` |

**交互：**
* 顶部 Tab 切换状态筛选（全部 / 已发布 / 草稿 / 私密）
* 创建/编辑使用模态框（Markdown 文本域 + 心情选择 + 状态选择）
* 删除需二次确认
* 列表分页（每页 15 条）

---

## AdminLayout 布局

源码: [src/layouts/AdminLayout.astro](../src/layouts/AdminLayout.astro)

**结构：**

```text
┌──────────────────────────────────┐
│ AdminLayout                      │
│ ┌──────┬───────────────────────┐ │
│ │      │ 顶栏（页面标题）       │ │
│ │ 侧   ├───────────────────────┤ │
│ │ 边   │                       │ │
│ │ 栏   │    <slot /> 内容区    │ │
│ │      │                       │ │
│ └──────┴───────────────────────┘ │
└──────────────────────────────────┘
```

**侧边栏菜单项：**

| key | 标签 | 路由 | 图标 |
| --- | --- | --- | --- |
| posts | 文章管理 | `/admin/posts` | `folderOpen` |
| comments | 评论管理 | `/admin/comments` | `envelope` |
| themes | 主题管理 | `/admin/themes` | `theme` |
| tags | 标签管理 | `/admin/tags` | `tag` |
| links | 友链管理 | `/admin/links` | `links` |
| notes | 随记管理 | `/admin/notes` | `notes` |

底部有"← 返回前台"链接回到 `/`
