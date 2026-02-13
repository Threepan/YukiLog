<div align="center">

# YukiLog 前端 API 封装文档

前端对后端 RESTful 接口的 TypeScript 封装层, 所有网络请求都走这里

</div>

---

## 目录

| 模块 | 说明 |
| --- | --- |
| 基础 | [fetchApi 通用封装](#fetchapi) |
| 公开 | [postsApi 文章](#postsapi) / [themesApi 主题](#themesapi) / [tagsApi 标签](#tagsapi) / [commentsApi 评论](#commentsapi) / [linksApi 友链](#linksapi) |
| 认证 | [authApi 登录](#authapi) |
| 管理 | [adminApi（JWT 保护）](#adminapi) |

---

<a id="fetchapi"></a>

## fetchApi 通用封装

源码: [src/lib/api.ts](../src/lib/api.ts)

所有 API 调用都通过 `fetchApi<T>()` 封装, 它做了这几件事:

1. 拼接 `PUBLIC_API_URL` 环境变量作为基础地址
2. 自动添加 `Content-Type: application/json`
3. 解析后端统一响应格式 `ApiResponse<T>`, 提取 `data` 字段
4. 非 2xx 或 `success: false` 时自动抛出 `Error`

```typescript
const API_BASE = import.meta.env.PUBLIC_API_URL || 'http://localhost:3000';

async function fetchApi<T>(endpoint: string, options?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE}${endpoint}`, { ... });
  const result: ApiResponse<T> = await response.json();
  if (!result.success) throw new Error(result.message || 'API 请求失败');
  return result.data!;
}
```

**后端统一响应格式：**

```json
{
  "success": true,
  "data": { ... },
  "message": null
}
```

**分页数据格式（PaginatedData）：**

```json
{
  "items": [],
  "total": 123,
  "page": 1,
  "page_size": 10,
  "total_pages": 13
}
```

---

<a id="postsapi"></a>

## postsApi（文章）

| 方法 | 说明 | 后端路由 |
| --- | --- | --- |
| `list(params?)` | 文章列表（分页/筛选/排序） | `GET /api/public/posts` |
| `getBySlug(slug)` | 文章详情 | `GET /api/public/posts/:slug` |
| `incrementView(slug)` | 增加浏览计数（fire & forget） | `POST /api/public/posts/:slug/view` |
| `search(params)` | 全文搜索 | `GET /api/public/search` |

**PostListParams 参数：**

```typescript
interface PostListParams {
  page?: number;
  page_size?: number;
  sort?: 'created_at' | 'updated_at' | 'view_count';
  status?: 'draft' | 'published';
  theme_slugs?: string;  // 逗号分隔
  tag_slugs?: string;    // 逗号分隔, AND 关系
}
```

**返回类型 PostWithRelations：**

```typescript
interface PostWithRelations {
  post: Post;
  theme: Theme | null;
  tags: Tag[];
}
```

**search 特殊说明：** 搜索结果中 `title` / `summary` 的关键词被 `<mark>` 标签包裹, `content` 被截取为关键词附近摘要

---

<a id="themesapi"></a>

## themesApi（主题）

| 方法 | 说明 | 后端路由 |
| --- | --- | --- |
| `list(sort?)` | 主题列表 | `GET /api/public/themes` |
| `getBySlug(slug)` | 主题详情 | `GET /api/public/themes/:slug` |
| `incrementView(slug)` | 增加浏览计数 | `POST /api/public/themes/:slug/view` |

sort 可选值: `'post_count'` / `'view_count'` / `'created_at'`

---

<a id="tagsapi"></a>

## tagsApi（标签）

| 方法 | 说明 | 后端路由 |
| --- | --- | --- |
| `list(sort?)` | 标签列表 | `GET /api/public/tags` |
| `getBySlug(slug)` | 标签详情 | `GET /api/public/tags/:slug` |
| `incrementView(slug)` | 增加浏览计数 | `POST /api/public/tags/:slug/view` |

sort 可选值: `'post_count'` / `'view_count'` / `'created_at'` / `'name'`

---

<a id="commentsapi"></a>

## commentsApi（评论）

| 方法 | 说明 | 后端路由 |
| --- | --- | --- |
| `getPostComments(slug)` | 获取文章评论树 | `GET /api/public/posts/:slug/comments` |
| `getCommentReplies(slug, id)` | 获取某条评论的直接回复 | `GET /api/public/posts/:slug/comments/:id` |
| `submit(slug, data)` | 提交评论/回复 | `POST /api/public/posts/:slug/comments` |

**CreateCommentRequest：**

```typescript
interface CreateCommentRequest {
  nickname: string;
  email: string;
  content: string;
  parent_id?: number | null;  // 回复时传父评论 ID
  website?: string | null;
}
```

**评论树结构（CommentNode）：**

```typescript
interface CommentNode {
  comment: Comment;
  children: CommentNode[];
}
```

---

<a id="linksapi"></a>

## linksApi（友链）

| 方法 | 说明 | 后端路由 |
| --- | --- | --- |
| `list()` | 友链列表（仅 active） | `GET /api/public/links` |
| `submit(data)` | 提交友链申请 | `POST /api/public/links/submit` |

---

<a id="authapi"></a>

## authApi（认证）

| 方法 | 说明 | 后端路由 |
| --- | --- | --- |
| `login(data)` | 管理员登录 | `POST /api/admin/login` |

```typescript
interface LoginRequest {
  username: string;
  password: string;
}

interface LoginResponse {
  token: string;       // JWT
  expires_in: number;  // 有效期（秒）
}
```

---

<a id="adminapi"></a>

## adminApi（管理端, JWT 保护）

所有管理端请求通过 `createAuthHeaders()` 自动从 `localStorage` 读取 JWT 并注入 `Authorization: Bearer <token>`

### adminApi.posts

| 方法 | HTTP | 路由 |
| --- | --- | --- |
| `list(params?)` | GET | `/api/admin/posts` |
| `getBySlug(slug)` | GET | `/api/admin/posts/:slug` |
| `create(data)` | POST | `/api/admin/posts` |
| `update(slug, data)` | PUT | `/api/admin/posts/:slug` |
| `delete(slug)` | DELETE | `/api/admin/posts/:slug` |

### adminApi.comments

| 方法 | HTTP | 路由 |
| --- | --- | --- |
| `list(params?)` | GET | `/api/admin/comments` |
| `pending()` | GET | `/api/admin/comments/pending` |
| `approve(id)` | PUT | `/api/admin/comments/:id/approve` |
| `reject(id)` | PUT | `/api/admin/comments/:id/reject` |
| `update(id, data)` | PUT | `/api/admin/comments/:id` |
| `delete(id)` | DELETE | `/api/admin/comments/:id` |

### adminApi.themes

| 方法 | HTTP | 路由 |
| --- | --- | --- |
| `create(data)` | POST | `/api/admin/themes` |
| `update(id, data)` | PUT | `/api/admin/themes/:id` |
| `delete(id)` | DELETE | `/api/admin/themes/:id` |

### adminApi.tags

| 方法 | HTTP | 路由 |
| --- | --- | --- |
| `create(data)` | POST | `/api/admin/tags` |
| `update(id, data)` | PUT | `/api/admin/tags/:id` |
| `delete(id)` | DELETE | `/api/admin/tags/:id` |
| `merge(data)` | POST | `/api/admin/tags/merge` |

### adminApi.links

| 方法 | HTTP | 路由 |
| --- | --- | --- |
| `list()` | GET | `/api/admin/links` |
| `create(data)` | POST | `/api/admin/links` |
| `pending()` | GET | `/api/admin/links/pending` |
| `approve(id)` | PUT | `/api/admin/links/:id/approve` |
| `markBroken(id)` | PUT | `/api/admin/links/:id/broken` |
| `update(id, data)` | PUT | `/api/admin/links/:id` |
| `delete(id)` | DELETE | `/api/admin/links/:id` |

---

## JWT 认证流程

源码: [src/lib/auth.ts](../src/lib/auth.ts) / [src/lib/admin-guard.ts](../src/lib/admin-guard.ts)

```text
用户输入密码 → authApi.login()
                  ↓
            返回 { token, expires_in }
                  ↓
            setToken() 写入 localStorage
            ├── yukilog_token = JWT 字符串
            └── yukilog_token_expiry = Date.now() + expires_in * 1000
                  ↓
            后续请求 → createAuthHeaders()
            → 从 localStorage 读取 token
            → 注入 Authorization: Bearer <token>
```

**路由守卫 guardAdminRoute()：**

每个管理页面在 `<script>` 中调用 `guardAdminRoute()`

* 未登录 → 跳转 `/admin/login`
* Token 即将过期 → 预留 toast 提示位

**登出 logout()：**

清除 localStorage 中的 token → 跳转 `/admin/login`
