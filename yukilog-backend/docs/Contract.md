# YukiLog-Backend API Contract

> 本文档定义后端所有 HTTP 接口规范，作为前后端对接的契约文档。
> 
> 架构设计详见 [Architecture.md](./Architecture.md)

---

## 📐 通用规范

### 响应格式
```json
// 成功响应
{
  "code": 0,
  "data": { ... },
  "message": "success"
}

// 错误响应
{
  "code": 40001,
  "data": null,
  "message": "Invalid token"
}
```

### 分页参数
| 参数   | 类型 | 默认值 | 说明                |
| ------ | ---- | ------ | ------------------- |
| `page` | u64  | 1      | 页码                |
| `size` | u64  | 10     | 每页数量 (max: 100) |

### 分页响应
```json
{
  "items": [...],
  "total": 100,
  "page": 1,
  "size": 10,
  "total_pages": 10
}
```

---

## 📥 接口清单

### 🌐 公开接口 `/api` (无需认证)

#### 📝 文章 (Posts)

| 路径                      | 方法  | 功能                   | 查询参数                          |
| ------------------------- | ----- | ---------------------- | --------------------------------- |
| `/api/posts`              | `GET` | 分页获取已发布文章     | `page`, `size`, `category`, `tag` |
| `/api/posts/:slug`        | `GET` | 根据 slug 获取文章详情 | -                                 |
| `/api/posts/:id/comments` | `GET` | 获取文章评论树         | -                                 |

#### 🏷️ 分类与标签 (Categories & Tags)

| 路径              | 方法  | 功能           | 说明                 |
| ----------------- | ----- | -------------- | -------------------- |
| `/api/categories` | `GET` | 获取分类列表   | 含每个分类的文章计数 |
| `/api/tags`       | `GET` | 获取标签云     | 含每个标签的引用计数 |
| `/api/archives`   | `GET` | 获取归档时间轴 | 按年月分组           |

#### 💬 评论与友链 (Comments & Links)

| 路径               | 方法   | 功能         | 说明                            |
| ------------------ | ------ | ------------ | ------------------------------- |
| `/api/comments`    | `POST` | 发表评论     | 游客需填 nickname/email/website |
| `/api/links`       | `GET`  | 获取友链列表 | 仅返回 status=active            |
| `/api/links/apply` | `POST` | 申请友链     | 需审核后生效                    |

#### 🔐 认证 (Auth)

| 路径                | 方法   | 功能       | 说明                 |
| ------------------- | ------ | ---------- | -------------------- |
| `/api/auth/login`   | `POST` | 登录       | 返回 JWT Token       |
| `/api/auth/refresh` | `POST` | 刷新 Token | 需提供 refresh_token |

---

### 👤 用户接口 `/api/user` (需 JWT Token)

| 路径                    | 方法    | 功能             | 说明                 |
| ----------------------- | ------- | ---------------- | -------------------- |
| `/api/user/me`          | `GET`   | 获取当前用户信息 | -                    |
| `/api/user/me`          | `PATCH` | 更新个人资料     | nickname, avatar_url |
| `/api/user/me/password` | `PUT`   | 修改密码         | 需验证旧密码         |

---

### 🛠️ 管理接口 `/api/admin` (需 Admin 角色)

#### 📝 文章管理

| 路径                          | 方法     | 功能         | 说明                     |
| ----------------------------- | -------- | ------------ | ------------------------ |
| `/api/admin/posts`            | `GET`    | 获取所有文章 | 含草稿、已归档           |
| `/api/admin/posts`            | `POST`   | 创建文章     | -                        |
| `/api/admin/posts/:id`        | `GET`    | 获取文章详情 | 含所有字段               |
| `/api/admin/posts/:id`        | `PUT`    | 更新文章     | -                        |
| `/api/admin/posts/:id`        | `DELETE` | 删除文章     | 硬删除                   |
| `/api/admin/posts/:id/status` | `PATCH`  | 切换文章状态 | draft/published/archived |

#### 🏷️ 分类管理

| 路径                        | 方法     | 功能         |
| --------------------------- | -------- | ------------ |
| `/api/admin/categories`     | `GET`    | 获取所有分类 |
| `/api/admin/categories`     | `POST`   | 创建分类     |
| `/api/admin/categories/:id` | `PUT`    | 更新分类     |
| `/api/admin/categories/:id` | `DELETE` | 删除分类     |

#### 🔖 标签管理

| 路径                  | 方法     | 功能         |
| --------------------- | -------- | ------------ |
| `/api/admin/tags`     | `GET`    | 获取所有标签 |
| `/api/admin/tags`     | `POST`   | 创建标签     |
| `/api/admin/tags/:id` | `PUT`    | 更新标签     |
| `/api/admin/tags/:id` | `DELETE` | 删除标签     |

#### 💬 评论管理

| 路径                             | 方法     | 功能         | 说明               |
| -------------------------------- | -------- | ------------ | ------------------ |
| `/api/admin/comments`            | `GET`    | 获取所有评论 | 支持按审核状态筛选 |
| `/api/admin/comments/:id`        | `DELETE` | 删除评论     | -                  |
| `/api/admin/comments/:id/review` | `PATCH`  | 审核评论     | 通过/拒绝          |

#### 🔗 友链管理

| 路径                   | 方法     | 功能         |
| ---------------------- | -------- | ------------ |
| `/api/admin/links`     | `GET`    | 获取所有友链 |
| `/api/admin/links`     | `POST`   | 创建友链     |
| `/api/admin/links/:id` | `PUT`    | 更新友链     |
| `/api/admin/links/:id` | `DELETE` | 删除友链     |

#### 👥 用户管理

| 路径                        | 方法    | 功能         |
| --------------------------- | ------- | ------------ |
| `/api/admin/users`          | `GET`   | 获取用户列表 |
| `/api/admin/users/:id`      | `GET`   | 获取用户详情 |
| `/api/admin/users/:id/role` | `PATCH` | 修改用户角色 |

#### 📊 仪表盘

| 路径                          | 方法  | 功能         |
| ----------------------------- | ----- | ------------ |
| `/api/admin/dashboard/stats`  | `GET` | 获取统计概览 |
| `/api/admin/dashboard/recent` | `GET` | 获取最近动态 |

---

## 📦 请求/响应 DTO 定义

### Auth

```typescript
// POST /api/auth/login
interface LoginRequest {
  username: string;
  password: string;
}

interface LoginResponse {
  access_token: string;
  refresh_token: string;
  expires_in: number;  // seconds
  user: UserInfo;
}
```

### Posts

```typescript
// GET /api/posts
interface PostListItem {
  id: number;
  title: string;
  sub_title?: string;
  slug: string;
  summary?: string;
  cover_image?: string;
  category?: CategoryInfo;
  tags: TagInfo[];
  author: AuthorInfo;
  view_count: number;
  is_pinned: boolean;
  published_at: string;
}

// POST /api/admin/posts
interface CreatePostRequest {
  title: string;
  sub_title?: string;
  slug: string;
  summary?: string;
  content: string;
  cover_image?: string;
  status: "draft" | "published";
  category_id?: number;
  tag_ids: number[];
  is_pinned?: boolean;
}
```

### Comments

```typescript
// POST /api/comments
interface CreateCommentRequest {
  post_id: number;
  content: string;
  parent_id?: number;      // 回复某条评论
  guest_nickname?: string; // 游客必填
  guest_email?: string;    // 游客必填
  guest_website?: string;
}

// 评论树节点
interface CommentNode {
  id: number;
  content: string;
  author: CommentAuthor;
  created_at: string;
  children: CommentNode[];
}
```

### Links

```typescript
// POST /api/links/apply
interface ApplyLinkRequest {
  link_title: string;
  link_url: string;
  link_avatar?: string;
  link_desc?: string;
}
