---
id: 11
title: "YukiLog - 7 - 前端与后端的契约"
slug: "yukilog-7-frontendbackend"
summary: "后端写好了接口，前端要怎么\"认识\"它？这篇讲的是如何用 TypeScript 把后端的 API 翻译成前端能理解的类型"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-7-frontendbackend.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["astro", "blog", "教程"]
view_count: 276
created_at: "2026-02-20 19:13:23"
updated_at: "2026-08-17 15:51:30"
source: "https://blog.yeastar.xin/posts/yukilog-7-frontendbackend"
---

## 后端收束，前端开始

在进入前端之前，先回顾一下后端做了什么：

```
HTTP 请求
  → Handler（协议翻译 + 身份验证）
  → Service（业务编排 + 事务）
  → Repo（类型安全的数据访问）
  → ORM（SQL ↔ Rust 类型）
  → PostgreSQL
```

这条链路的终点是 Handler 层暴露出来的 API 接口。后端用 Rust 的类型系统保证了数据的正确性，但前端运行在浏览器里，它只能收到 JSON，不知道 Rust 类型的存在

这就带来了一个问题：**前端怎么知道一个 API 返回的数据长什么样？**

---

## 契约的两端

后端定义了统一的响应格式：

```rust
// Rust（后端）
pub struct ApiResponse {
    pub success: bool,
    pub data: Option,
    pub message: Option,
}
```

前端收到的是 JSON：

```json
{
  "success": true,
  "data": { "id": 1, "title": "Hello World", ... },
  "message": null
}
```

如果前端不做任何类型定义，`data` 就是 `any`——TypeScript 的类型检查形同虚设，写错字段名也不会报错，只有运行时才会出问题

解决方案是在前端侧**镜像**后端的类型定义，把这份 JSON 契约翻译成 TypeScript 接口

---

## 翻译契约

`src/types/api.ts` 里定义了所有和后端对应的类型。以文章为例：

后端的 Rust 结构体：

```rust
pub struct Post {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub summary: Option,
    pub content: String,
    pub status: PostStatus,  // 枚举
    pub theme_id: Option,
    pub view_count: i64,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

前端的 TypeScript 接口：

```typescript
export type PostStatus = 'draft' | 'published';

export interface Post {
  id: number;
  title: string;
  slug: string;
  summary: string | null;
  content: string;
  status: PostStatus;
  theme_id: number | null;
  view_count: number;
  created_at: string;   // JSON 里时间是字符串
  updated_at: string;
}
```

几个值得注意的翻译规则：

* Rust 的 `Option` → TypeScript 的 `T | null`
* Rust 的 `i64` / `i32` → TypeScript 的 `number`（JS 没有整数类型）
* Rust 的 `DateTime` → TypeScript 的 `string`（JSON 序列化后是 ISO 8601 字符串）
* Rust 的枚举 `PostStatus` → TypeScript 的联合类型 `'draft' | 'published'`

---

## 统一响应格式的翻译

后端所有接口都返回 `ApiResponse`，前端也定义了对应的泛型接口：

```typescript
export interface ApiResponse {
  success: boolean;
  data: T | null;
  message: string | null;
}

export interface PaginatedData {
  items: T[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}
```

有了这两个泛型，所有接口的响应类型都能精确描述：

```typescript
// 文章列表接口返回的类型
ApiResponse>

// 文章详情接口返回的类型
ApiResponse

// 提交评论接口返回的类型
ApiResponse
```

---

## API 调用封装

有了类型定义，下一步是封装 API 调用。`src/lib/api.ts` 里有一个通用的 `fetchApi` 函数，负责处理所有请求的公共逻辑：

```typescript
async function fetchApi(endpoint: string, options?: RequestInit): Promise {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    headers: { 'Content-Type': 'application/json', ...options?.headers },
    ...options,
  });

  if (!response.ok) {
    throw new Error(`API Error: ${response.status}`);
  }

  const result: ApiResponse = await response.json();

  if (!result.success) {
    throw new Error(result.message || 'API 请求失败');
  }

  return result.data!;
}
```

这个函数做了三件事：发请求、检查 HTTP 状态码、检查业务层的 `success` 字段。调用方拿到的直接是 `data`，不需要每次都写 `if (result.success)` 的判断

基于这个函数，每个业务模块封装自己的 API：

```typescript
export const postsApi = {
  async list(params?: PostListParams): Promise> {
    const query = new URLSearchParams();
    if (params?.page) query.set('page', params.page.toString());
    if (params?.sort) query.set('sort', params.sort);
    // ...
    return fetchApi(`/api/public/posts?${query}`);
  },

  async getBySlug(slug: string): Promise {
    return fetchApi(`/api/public/posts/${slug}`);
  },
}
```

调用方的代码变得非常干净，而且有完整的类型推断：

```typescript
const posts = await postsApi.list({ page: 1, sort: 'created_at' });
// posts 的类型是 PaginatedData，编译器知道 posts.items[0].post.title 是 string
```

---

## 一个有意思的细节：SSR 与浏览器的 API 地址不同

`fetchApi` 里有一个 `API_BASE` 变量，它的值取决于当前运行环境：

```typescript
const API_BASE = import.meta.env.SSR
  ? (import.meta.env.PUBLIC_API_URL || 'http://localhost:3639')
  : (import.meta.env.PUBLIC_SITE_URL || 'https://blog.yeastar.xin');
```

这解决了一个具体问题：同一套代码在两个不同的地方运行

* **服务端渲染时**：Astro 服务器和后端服务器在同一台机器上，直接用内网地址通信，不需要走公网，速度更快
* **浏览器运行时**：用户的浏览器在外网，必须通过公网域名访问，经过 Nginx 反向代理转发到后端

`import.meta.env.SSR` 是 Astro 提供的环境变量，在服务端为 `true`，在浏览器为 `false`。同一行代码，在不同环境里走不同的路径

---

## 小结

前端和后端之间的契约，体现在两个地方：

* **类型定义**（`src/types/api.ts`）：把后端的 Rust 类型翻译成 TypeScript 接口，让编译器帮你检查字段是否用对
* **API 封装**（`src/lib/api.ts`）：把 HTTP 调用的细节隐藏起来，让业务代码只关心"我要什么数据"

下一篇讲 `yukilog.config.ts`——一个文件管理整个博客的所有配置
