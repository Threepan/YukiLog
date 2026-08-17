---
id: 10
title: "YukiLog - 6 - Handler 层"
slug: "yukilog-6-handler"
summary: "Service 层的函数写好了，但前端不认识 Rust 函数。这篇讲的是如何把函数变成接口，以及这个过程里藏着哪些意想不到的复杂性"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-6-handler.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["blog", "rust", "教程"]
view_count: 235
created_at: "2026-02-20 09:01:23"
updated_at: "2026-08-17 15:51:39"
source: "https://blog.yeastar.xin/posts/yukilog-6-handler"
---

## 引言

Service 层建好之后，我有了一组干净的 Rust 函数：`create_post`、`list_posts`、`submit_comment`……

但前端不认识这些函数。前端运行在浏览器里，它只能发 HTTP 请求，只能收 JSON 响应。我需要在 Service 和前端之间建立一套通信协议——这就是 **API**

Handler 层的本质，就是**把 Rust 函数变成 HTTP 接口**

但这个翻译过程比想象中复杂。HTTP 请求里夹带了很多"业务之外的东西"：浏览器的身份信息、跨域请求头、客户端 IP、User-Agent……这些都需要在业务处理之前被处理掉。这就是为什么 Handler 层看起来比其他层"杂"——它是前后端世界的交接地带，两边的复杂性都在这里汇聚

---

## 接口契约

在动手写代码之前，我需要先想清楚：这个系统对外暴露哪些接口？

博客系统有两类用户：**访客**和**管理员**。他们能做的事情完全不同：

```
公开接口（任何人都能访问）
  GET  /api/public/posts              获取文章列表
  GET  /api/public/posts/:slug        获取文章详情
  POST /api/public/posts/:slug/view   增加浏览量
  GET  /api/public/search             全文搜索
  POST /api/public/posts/:slug/comments  提交评论
  GET  /api/public/links              获取友链
  ...

管理接口（需要登录）
  POST   /api/admin/posts             创建文章
  PUT    /api/admin/posts/:slug       更新文章
  DELETE /api/admin/posts/:slug       删除文章
  PUT    /api/admin/comments/:id/approve  审核评论
  ...
```

这份路由表就是前后端之间的契约。前端按照这份契约发请求，后端按照这份契约处理请求。任何一方改变了契约，另一方就会出错

路由在代码里按权限分成两组，管理路由整体套一层 JWT 中间件：

```rust
pub fn app_routes(state: AppState) -> Router {
    let admin_routes_with_auth = admin_routes()
        .layer(middleware::from_fn_with_state(state.clone(), jwt_auth));

    Router::new()
        .merge(public_routes())
        .merge(admin_routes_with_auth)
        .with_state(state)
}
```

`.layer()` 把中间件应用到整个路由组。新增管理接口时不需要记得加认证——结构本身保证了安全

---

## 浏览器带来的"额外信息"

一个 HTTP 请求不只是业务数据，它还携带了大量浏览器和网络层面的信息：

```
POST /api/admin/posts HTTP/1.1
Host: api.yukilog.com
Authorization: Bearer eyJhbGciOiJIUzI1N...   ← 身份信息
User-Agent: Mozilla/5.0 (wayland; Linux x86_64)  ← 客户端信息
Content-Type: application/json
Origin: https://yukilog.com                   ← 来源
X-Forwarded-For: 203.0.113.1                  ← 真实 IP（经过反向代理）
```

这些信息不是业务数据，但后端需要处理它们：

* `Authorization` 里的 JWT Token 需要验证，才能知道请求者是不是管理员
* `X-Forwarded-For` 里的 IP 需要提取，用于限流判断
* `User-Agent` 里的客户端信息可以用于过滤机器人

这些处理逻辑不属于业务，但必须在业务处理之前完成。这就是**中间件**存在的原因

---

## 中间件：请求的预处理

一个请求到达服务器后，不会直接进入 handler 函数，而是先经过一层层中间件：

```
客户端请求
    ↓
[CORS 中间件]      ← 处理跨域
    ↓
[日志中间件]       ← 记录请求
    ↓
[JWT 中间件]       ← 验证身份（仅管理路由）
    ↓
[Handler 函数]     ← 业务处理
    ↓
客户端收到响应
```

JWT 中间件是其中最重要的一个。它做四件事：

1. 从 `Authorization: Bearer ` 里提取 Token
2. 用 `JWT_SECRET` 验证签名和过期时间
3. 把解析出的 `Claims`（包含用户名和过期时间）注入到请求上下文
4. 放行，让请求继续往下走

如果任何一步失败，直接返回 401，请求不会到达 handler：

```
缺少 Token  → 401 "缺少认证令牌"
Token 无效  → 401 "认证令牌无效"
Token 过期  → 401 "认证令牌已过期"
```

验证通过后，`Claims` 被存入请求的临时上下文。下游的 handler 函数通过 `Extension` 提取器拿到它，不需要重新解析 Token：

```rust
pub async fn create_post(
    State(state): State,
    Extension(claims): Extension,  // 中间件注入的，直接用
    Json(req): Json,
) -> Result>, ServiceError> {
    // claims.sub 是当前登录的用户名
}
```

---

## JWT：前后端共同维护的约定

JWT 不是纯后端的事，它是前后端之间的一个约定：

* **后端**：用户登录后，签发一个 JWT Token，包含用户名和过期时间，用 `JWT_SECRET` 签名
* **前端**：把 Token 存在本地（localStorage 或 Cookie），每次请求管理接口时带上它
* **后端**：收到请求时，验证 Token 的签名和有效期

这个约定的好处是**无状态**：后端不需要存储会话信息，只需要验证 Token 的签名。任何一台服务器都能验证，不需要共享会话存储

---

## 依赖注入：AppState

Handler 函数需要访问数据库和 Redis，但函数签名不能直接依赖全局变量。Axum 的解决方案是把所有依赖打包成 `AppState`，在应用启动时创建一次，然后注入到所有路由：

```rust
pub struct AppState {
    pub config: AppConfig,
    pub db: DatabaseConnection,   // 数据库连接池
    pub redis: redis::Client,     // Redis 客户端
}
```

每个 handler 函数通过 `State` 提取器声明自己需要什么，Axum 负责传递：

```rust
pub async fn list_posts(
    State(state): State,
    Query(params): Query,
) -> ... {
    service::posts::list_posts(&state.db, filter).await
}
```

---

## 统一响应格式

前端需要知道每个接口返回的数据长什么样。如果每个接口的响应格式都不一样，前端的解析代码会变得非常混乱

所有接口统一返回同一种结构：

```rust
pub struct ApiResponse {
    pub success: bool,
    pub data: Option,      // 成功时有数据
    pub message: Option,  // 失败时有错误信息
}
```

成功：`{ "success": true, "data": { ... } }`
失败：`{ "success": false, "message": "资源不存在" }`

前端只需要检查 `success` 字段，就能知道请求是否成功，不需要猜测响应格式

---

## 错误传播链的终点

第五篇定义了 `ServiceError`，但没有说它最终去哪里。答案是：在 Handler 层被转成 HTTP 状态码

```
RepoError → ServiceError → HTTP 响应
```

`ServiceError` 实现了 `IntoResponse`，Axum 会自动调用它：

```rust
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServiceError::NotFound => (404, "资源不存在"),
            ServiceError::InvalidInput(msg) => (400, msg),
            ServiceError::Repo(RepoError::Db(e)) => {
                tracing::error!("Database error: {:?}", e);  // 记录日志
                (500, "数据库错误")  // 不暴露具体错误给客户端
            }
        };
        // ...
    }
}
```

数据库错误不会把具体信息暴露给客户端，只返回通用的"数据库错误"，真实原因记录在服务端日志里。这是基本的安全实践——内部错误不应该泄漏给外部

---

## 小结

Handler 层是前后端世界的交接地带。它做的事情可以分成三类：

* **协议翻译**：JSON ↔ Rust 类型，请求参数提取，响应格式统一
* **身份验证**：JWT 中间件，在业务处理之前解决"你是谁"的问题
* **接口契约**：路由设计，public vs admin 的权限边界

这一层的复杂性是诚实的——它如实反映了前后端协作的复杂性，没有把这种复杂性向下转移给 Service 层

后端到这里就完整了。下一篇进入前端，后端暴露了这些接口，前端需要做的第一件事是为它们建立对应的 TypeScript 类型定义——然后才是真正的页面和交互
