# YukiLog-Backend 架构设计

> 本文档描述后端系统的分层架构、目录结构和安全设计。

---

## 🏗️ 分层架构

> 📊 **[查看交互式架构图](./html/architecture-layers.html)**

| 层级                 | 目录        | 职责                  |
| -------------------- | ----------- | --------------------- |
| **HTTP Layer**       | `api/`      | 路由、中间件、Handler |
| **Service Layer**    | `core/`     | 业务逻辑、事务处理    |
| **Repository Layer** | `infra/`    | 数据访问、CRUD 抽象   |
| **Entity Layer**     | `entities/` | SeaORM 模型定义       |

---

## 📁 目录结构

```
yukilog-backend/
├── Cargo.toml
├── Contract.md          # API 接口契约文档
├── Architecture.md      # 本文档
├── .env                 # 环境变量 (不提交)
├── .env.example         # 环境变量模板
│
└── src/
    ├── main.rs          # 入口：初始化 + 启动服务器
    │
    ├── config/          # 🔧 配置管理
    │   ├── mod.rs
    │   ├── app.rs       # AppConfig 结构体
    │   └── database.rs  # 数据库连接配置
    │
    ├── api/             # 🌐 HTTP 接口层
    │   ├── mod.rs
    │   ├── router.rs    # 路由注册入口
    │   │
    │   ├── middleware/  # 中间件
    │   │   ├── mod.rs
    │   │   ├── auth.rs      # JWT 验证
    │   │   ├── admin.rs     # Admin 角色检查
    │   │   └── request_id.rs
    │   │
    │   ├── extractors/  # 自定义提取器
    │   │   ├── mod.rs
    │   │   ├── auth.rs      # CurrentUser 提取器
    │   │   └── pagination.rs
    │   │
    │   └── http/        # Handler 按资源分组
    │       ├── mod.rs
    │       ├── health.rs    # 健康检查
    │       ├── auth.rs      # 登录/刷新
    │       ├── posts.rs     # 公开文章接口
    │       ├── comments.rs
    │       ├── categories.rs
    │       ├── tags.rs
    │       ├── links.rs
    │       │
    │       ├── user/        # /api/user 用户接口
    │       │   ├── mod.rs
    │       │   └── profile.rs
    │       │
    │       └── admin/       # /api/admin 管理接口
    │           ├── mod.rs
    │           ├── posts.rs
    │           ├── comments.rs
    │           ├── categories.rs
    │           ├── tags.rs
    │           ├── links.rs
    │           ├── users.rs
    │           └── dashboard.rs
    │
    ├── core/            # 💼 业务逻辑层 (Services)
    │   ├── mod.rs
    │   ├── error.rs     # 业务错误定义
    │   │
    │   ├── auth/
    │   │   ├── mod.rs
    │   │   ├── service.rs   # AuthService
    │   │   ├── jwt.rs       # JWT 工具
    │   │   └── password.rs  # Argon2 哈希
    │   │
    │   ├── posts/
    │   │   ├── mod.rs
    │   │   ├── service.rs   # PostService
    │   │   └── dto.rs       # 输入/输出 DTO
    │   │
    │   ├── comments/
    │   │   ├── mod.rs
    │   │   ├── service.rs
    │   │   └── dto.rs
    │   │
    │   ├── categories/
    │   │   ├── mod.rs
    │   │   └── service.rs
    │   │
    │   ├── tags/
    │   │   ├── mod.rs
    │   │   └── service.rs
    │   │
    │   ├── links/
    │   │   ├── mod.rs
    │   │   └── service.rs
    │   │
    │   └── users/
    │       ├── mod.rs
    │       └── service.rs
    │
    ├── infra/           # 🗄️ 基础设施层 (Repository)
    │   ├── mod.rs
    │   ├── repository.rs    # 通用 Repository trait
    │   │
    │   ├── posts.rs         # PostRepository
    │   ├── users.rs         # UserRepository
    │   ├── comments.rs      # CommentRepository
    │   ├── categories.rs
    │   ├── tags.rs
    │   └── links.rs
    │
    ├── entities/        # 📊 SeaORM 实体 (自动生成)
    │   ├── mod.rs
    │   ├── prelude.rs
    │   ├── posts.rs
    │   ├── users.rs
    │   ├── categories.rs
    │   ├── tags.rs
    │   ├── post_tags.rs
    │   ├── comments.rs
    │   └── links.rs
    │
    └── common/          # 🔨 公共工具
        ├── mod.rs
        ├── response.rs  # 统一响应封装
        ├── pagination.rs
        └── time.rs
```

---

## 🔐 安全架构

### 1. 认证流程 (Authentication)

> 📊 **[查看认证流程时序图](./html/auth-flow.html)**

**流程概述：**
1. Client 发送 `POST /api/auth/login` 携带 `{ username, password }`
2. Handler 调用 `AuthService.login()`
3. Service 通过 Repository 查询用户
4. 验证密码（Argon2）→ 签发 JWT
5. 返回 `{ access_token, refresh_token }`

### 2. JWT 结构

```rust
pub struct Claims {
    pub sub: i64,           // user_id
    pub username: String,
    pub role: String,       // "admin" | "user"
    pub exp: i64,           // 过期时间
    pub iat: i64,           // 签发时间
}
```

**Token 策略:**
- Access Token: 有效期 15 分钟
- Refresh Token: 有效期 7 天
- 使用 RS256 算法签名

### 3. 中间件守卫

> 📊 **[查看中间件管道图](./html/middleware-pipeline.html)**

| 中间件          | 作用范围                      | 功能                    |
| --------------- | ----------------------------- | ----------------------- |
| **RequestId**   | 所有请求                      | 生成追踪 ID，注入响应头 |
| **RateLimiter** | 所有请求                      | IP 限流，防暴力破解     |
| **AuthGuard**   | `/api/user/*`, `/api/admin/*` | JWT 验证，注入 Claims   |
| **AdminGuard**  | `/api/admin/*`                | 检查 `role == "admin"`  |

### 4. 中间件实现示例

```rust
// src/api/middleware/auth.rs
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

pub async fn auth_guard(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // 1. 提取 Bearer Token
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized("Missing token"))?;

    // 2. 验证 JWT
    let claims = state.auth_service
        .verify_token(token)
        .map_err(|_| AppError::Unauthorized("Invalid token"))?;

    // 3. 注入到 Request Extensions
    request.extensions_mut().insert(claims);

    // 4. 继续处理
    Ok(next.run(request).await)
}

// src/api/middleware/admin.rs
pub async fn admin_guard(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized("No claims"))?;

    if claims.role != "admin" {
        return Err(AppError::Forbidden("Admin only"));
    }

    Ok(next.run(request).await)
}
```

### 5. 路由层应用

```rust
// src/api/router.rs
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 公开接口 - 无中间件
        .nest("/api", public_routes())
        
        // 用户接口 - 需要登录
        .nest("/api/user", 
            user_routes()
                .layer(from_fn_with_state(state.clone(), auth_guard))
        )
        
        // 管理接口 - 需要 Admin
        .nest("/api/admin",
            admin_routes()
                .layer(from_fn(admin_guard))
                .layer(from_fn_with_state(state.clone(), auth_guard))
        )
        
        .with_state(state)
}
```

---

## 🛡️ 安全最佳实践

### 密码存储
- 使用 **Argon2id** 算法
- 配置: memory=64MB, iterations=3, parallelism=4

### 输入校验
- 所有输入使用 `validator` crate 校验
- SQL 注入: SeaORM 自动参数化
- XSS: 前端渲染时转义，后端存储原文

### 敏感数据
- 响应中永远不返回 `password_hash`
- 日志中脱敏 email、IP

### CORS 配置
```rust
CorsLayer::new()
    .allow_origin(["https://yukiblog.com".parse().unwrap()])
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
    .allow_credentials(true)
```

### 限流策略
- 公开接口: 100 req/min per IP
- 登录接口: 5 req/min per IP (防暴力破解)
- Admin 接口: 300 req/min per User

---

## 📦 AppState 结构

```rust
// src/main.rs 或 src/config/state.rs
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
    
    // Services (Arc 包装实现 Clone)
    pub auth_service: Arc<AuthService>,
    pub post_service: Arc<PostService>,
    pub comment_service: Arc<CommentService>,
    pub category_service: Arc<CategoryService>,
    pub tag_service: Arc<TagService>,
    pub link_service: Arc<LinkService>,
    pub user_service: Arc<UserService>,
}
```

---

## 🔄 请求处理流程示例

### 创建文章 (POST /api/admin/posts)

> 📊 **[查看完整请求流程图](./html/request-flow.html)**

| 步骤 | 层级       | 操作                            |
| ---- | ---------- | ------------------------------- |
| 1    | HTTP       | Request 到达                    |
| 2    | Middleware | AuthGuard 验证 JWT，注入 Claims |
| 3    | Middleware | AdminGuard 检查角色             |
| 4    | Handler    | 解析请求，调用 Service          |
| 5    | Service    | 业务逻辑，事务处理              |
| 6    | Handler    | 构造 ApiResponse                |
| 7    | HTTP       | Response 返回                   |

---

## 📝 错误处理

```rust
// src/core/error.rs
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Database error")]
    Database(#[from] sea_orm::DbErr),
    
    #[error("Internal error")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, 40100, msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, 40300, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, 40400, msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, 40000, msg.clone()),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50000, "Database error".into()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 50000, msg.clone()),
        };
        
        Json(ApiResponse::<()>::error(code, message)).into_response()
    }
}
```
