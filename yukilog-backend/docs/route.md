<div align="center">

# YukiLog Route 路由文档

这一层我们就要把之前所有的 公开路由, 管理路由, JWT中间件全部组装起来, 暴露为真正的网络接口

</div>

---

## | 这一层是干嘛的

简单来说, 就是"把所有接口挂载到对应的 URL 路径上"

之前我们写了一大堆 handler 函数, 比如 `create_post`, `list_comments` 之类的, 但是这些函数还没有对应的 HTTP 路径呢

所以这一层的工作就是:

* 把 `create_post` 挂到 `POST /api/admin/posts` 上
* 把 `list_comments` 挂到 `GET /api/admin/comments` 上
* 把 JWT 中间件加到所有管理路由上
* 最后把所有路由合并成一个完整的应用

---

## | 源码位置

这一层的代码在这个文件里:

* [yukilog-backend/src/handler/mod.rs](../src/handler/mod.rs) - 路由注册和组装

---

## | 路由分组

我们把路由分成了 3 组:

#### 1. 认证路由 `auth_routes()`

只有一个登录接口, 用来给管理员获取 JWT token

```rust
POST    /api/admin/login    -> auth::login
```

#### 2. 公开路由 `public_routes()`

前台接口, 任何人都可以访问, 不需要登录

**总共 14 个接口:**

**Themes 主题 (3个)**
```rust
GET     /api/themes              -> public::themes::list_themes
GET     /api/themes/:slug        -> public::themes::get_theme
POST    /api/themes/:slug/view   -> public::themes::increment_theme_view
```

**Tags 标签 (3个)**
```rust
GET     /api/tags                -> public::tags::list_tags
GET     /api/tags/:slug          -> public::tags::get_tag
POST    /api/tags/:slug/view     -> public::tags::increment_tag_view
```

**Posts 文章 (3个)**
```rust
GET     /api/posts               -> public::posts::list_posts
GET     /api/posts/:slug         -> public::posts::get_post
POST    /api/posts/:slug/view    -> public::posts::increment_post_view
```

**Comments 评论 (3个)**
```rust
GET     /api/posts/:slug/comments           -> public::comments::get_post_comments
POST    /api/posts/:slug/comments           -> public::comments::create_comment
GET     /api/posts/:slug/comments/:id       -> public::comments::get_comment_replies
```

**Links 友链 (2个)**
```rust
GET     /api/links               -> public::links::list_links
POST    /api/links/submit        -> public::links::submit_link
```

#### 3. 管理路由 `admin_routes()`

后台接口, 需要 JWT 认证才能访问

**总共 23 个接口:**

**Themes 主题 (3个)**
```rust
POST    /api/admin/themes        -> admin::themes::create_theme
PUT     /api/admin/themes/:id    -> admin::themes::update_theme
DELETE  /api/admin/themes/:id    -> admin::themes::delete_theme
```

**Tags 标签 (4个)**
```rust
POST    /api/admin/tags          -> admin::tags::create_tag
PUT     /api/admin/tags/:id      -> admin::tags::update_tag
DELETE  /api/admin/tags/:id      -> admin::tags::delete_tag
POST    /api/admin/tags/merge    -> admin::tags::merge_tags
```

**Posts 文章 (4个)**
```rust
GET     /api/admin/posts         -> admin::posts::list_posts
POST    /api/admin/posts         -> admin::posts::create_post
PUT     /api/admin/posts/:slug   -> admin::posts::update_post
DELETE  /api/admin/posts/:slug   -> admin::posts::delete_post
```

**Comments 评论 (6个)**
```rust
GET     /api/admin/comments              -> admin::comments::list_comments
GET     /api/admin/comments/pending      -> admin::comments::list_pending_comments
PUT     /api/admin/comments/:id/approve  -> admin::comments::approve_comment
PUT     /api/admin/comments/:id/reject   -> admin::comments::reject_comment
PUT     /api/admin/comments/:id          -> admin::comments::update_comment
DELETE  /api/admin/comments/:id          -> admin::comments::delete_comment
```

**Links 友链 (6个)**
```rust
GET     /api/admin/links             -> admin::links::list_links
GET     /api/admin/links/pending     -> admin::links::list_pending_links
PUT     /api/admin/links/:id/approve -> admin::links::approve_link
PUT     /api/admin/links/:id/broken  -> admin::links::mark_link_broken
PUT     /api/admin/links/:id         -> admin::links::update_link
DELETE  /api/admin/links/:id         -> admin::links::delete_link
```

---

## | JWT 中间件怎么加的

管理路由需要登录才能访问, 所以我们得给它们加上 JWT 认证中间件

```rust
pub fn app_routes(state: AppState) -> Router {
    let admin_routes_with_auth = admin_routes()
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            self::middleware::jwt_auth,
        ));

    Router::new()
        .merge(auth_routes())
        .merge(public_routes())
        .merge(admin_routes_with_auth)
        .with_state(state)
}
```

这样一来:

* 访问 `/api/admin/*` 的任何接口都会先经过 `jwt_auth` 中间件
* 中间件会从 `Authorization: Bearer <token>` header 里提取 token
* 验证 token 是否有效、是否过期
* 如果验证通过, 就把用户信息注入到 handler 里
* 如果验证失败, 直接返回 401 错误

---

## | 路由注册顺序

有一个小坑要注意: **具体路径要放在参数路径前面**

比如:

```rust
// ✅ 正确: /pending 在前
.route("/api/admin/comments/pending", get(...))
.route("/api/admin/comments/:id", get(...))

// ❌ 错误: /pending 会被当成 :id 参数
.route("/api/admin/comments/:id", get(...))
.route("/api/admin/comments/pending", get(...))
```

如果顺序反了, 访问 `/api/admin/comments/pending` 时, Axum 会匹配到 `/:id` 这条路由, 把 `pending` 当成 id 参数...

---

## | 统计

把所有东西加起来, 一共有:

* **认证接口**: 1 个
* **公开接口**: 14 个
* **管理接口**: 23 个
* **总计**: 38 个网络接口~

---

## | 完整代码示例

```rust
use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handler::state::AppState;

/// 认证路由（登录）
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/api/admin/login", post(auth::login))
}

/// 公开路由（前台，无需认证）
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/api/themes", get(public::themes::list_themes))
        .route("/api/tags", get(public::tags::list_tags))
        // ... 其他路由
}

/// 管理路由（后台，需要 JWT 认证）
pub fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/api/admin/themes", post(admin::themes::create_theme))
        .route("/api/admin/tags", post(admin::tags::create_tag))
        // ... 其他路由
}

/// 应用根路由（合并所有路由）
pub fn app_routes(state: AppState) -> Router {
    let admin_routes_with_auth = admin_routes()
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            self::middleware::jwt_auth,
        ));

    Router::new()
        .merge(auth_routes())
        .merge(public_routes())
        .merge(admin_routes_with_auth)
        .with_state(state)
}
```

---

## | 下一步

路由组装好之后, 就可以在 `main.rs` 里启动服务器啦~

把 `app_routes(state)` 传给 `axum::serve()`, 然后监听一个端口, 就可以对外提供服务了

详细的启动代码可以看 [yukilog-backend/src/main.rs](../src/main.rs)

