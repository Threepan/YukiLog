<div align="center">

# YukiLog Handler 层文档

这一层的主要工做就是把 `service` 接口全部变成 `网络接口`

</div>

## 搭地基~

首先我们要做网络层的基本适配, 也就是

* 定义 `service` 层与 `handler` 层的通信协议
* 将 **业务报错** 转换为 **网络错误码**
* 封装 **管理员的鉴权逻辑**
* 封装 **JWT 中间件**

---

## AppState 统一注入

为了避免出现多个 State（比如一部分 handler 注入配置，一部分 handler 注入 DB），本项目在 handler 层统一使用 `State<AppState>`。

源码: [yukilog-backend/src/handler/state.rs](../src/handler/state.rs)

```rust
pub struct AppState {
    /// 应用配置
    pub config: AppConfig,
    /// SeaORM 数据库连接
    pub db: DatabaseConnection,
    /// Redis 客户端（用于限流和缓存）
    pub redis: redis::Client,
}
```

使用约定:

* **auth 登录**: 通过 `state.config` 读取 `ADMIN_USERNAME` / `ADMIN_PASSWORD_HASH` / `JWT_SECRET` / `JWT_EXPIRES_IN`
* **JWT 中间件**: 通过 `state.config.jwt_secret` 验证 token
* **业务 handler**: 通过 `state.db` 调用 service；需要限流/防刷时使用 `state.redis`

---

## 统一响应格式

这里说的是, 所有 `handler` 层接口都应该返回指定格式的数据, 方便前端解析

源码: [yukilog-backend/src/handler/response.rs](../src/handler/response.rs)

```rust
/// 统一响应格式
pub struct ApiResponse<T> {
    /// 请求是否成功
    pub success: bool,
    /// 响应数据（成功时存在）
    pub data: Option<T>,
    /// 错误或提示信息（失败时存在）
    pub message: Option<String>,
}

/// 分页数据响应
pub struct PagedData<T> {
    /// 当前页数据
    pub items: Vec<T>,
    /// 数据总数
    pub total: u64,
    /// 当前页码（从 1 开始）
    pub page: u64,
    /// 每页大小
    pub page_size: u64,
    /// 总页数
    pub total_pages: u64,
}
```

---

## 错误转换

这一层是将 `service error` 转换成 `HTTP` 响应

源码: [yukilog-backend/src/handler/error.rs](../src/handler/error.rs)

#### ServiceError 映射

* `NotFound` -> **404 NOT_FOUND**
* `InvalidInput` -> **400 BAD_REQUEST**
* `Repo(Db(_))` -> **500 INTERNAL_SERVER_ERROR**
* `Repo(NotFound)` -> **404 NOT_FOUND**
* `Repo(InvalidStatus)` -> **400 BAD_REQUEST**


```rust
/// 错误映射
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServiceError::NotFound => {
                (StatusCode::NOT_FOUND, "资源不存在")
            }
            ServiceError::InvalidInput(msg) => {
                (StatusCode::BAD_REQUEST, msg.as_str())
            }
            ServiceError::Repo(RepoError::Db(e)) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "数据库错误")
            }
            ServiceError::Repo(RepoError::NotFound) => {
                (StatusCode::NOT_FOUND, "资源不存在")
            }
            ServiceError::Repo(RepoError::InvalidStatus(s)) => {
                (StatusCode::BAD_REQUEST, &format!("无效状态: {}", s))
            }
        };
        
        (status, Json(ApiResponse::error(message))).into_response()
    }
}
```

#### 认证错误

* `InvalidToken`
* `TokenExpired`
* `MissingToken`
* `InvalidCredentials`

```rust
/// 认证错误枚举
pub enum AuthError {
    /// 无效的令牌
    InvalidToken,
    /// 令牌已过期
    TokenExpired,
    /// 缺少认证令牌
    MissingToken,
    /// 用户名或密码错误
    InvalidCredentials,
}
```

---

## Auth 登录

这里用来处理登录逻辑, 用于管理员登录后端

源码: [yukilog-backend/src/handler/auth.rs](../src/handler/auth.rs)

#### 登录接口

```bash
POST    /api/admin/login  - 管理员登录
```

#### 接口定义

```rust
/// 管理员登录接口
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AuthError>
```

#### DTO 定义

**JWT 令牌声明**

```rust
pub struct Claims {
    /// 用户名（subject）
    pub sub: String,
    /// 过期时间（Unix 时间戳）
    pub exp: usize,
}
```

**登录相关**

```rust
/// 登录请求
pub struct LoginRequest {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}
/// 登录响应
pub struct LoginResponse {
    /// JWT 令牌
    pub token: String,
    /// 过期时间（秒）
    pub expires_in: i64,
}
```

## JWT 认证中间件

源码: [yukilog-backend/src/handler/middleware.rs](../src/handler/middleware.rs)

**这部分比较复杂, 我在设计文档里详细说明:**

[YukiLog 设计文档](./yukilog.md)
