## ✅ 阶段 1：基础设施层（已完成）

这些是其他层都会用到的基础组件，优先完成：

1. ✅ Common 公共工具 📦

- ✅ `common/response.rs` - 统一 API 响应格式 ApiResponse<T>
- ✅ `common/pagination.rs` - 分页请求/响应结构
- ✅ `common/mod.rs` - 模块导出

2. ✅ Config 配置管理 ⚙️

- ✅ `config/app.rs` - 应用配置（读取 .env）
- ✅ `config/database.rs` - 数据库连接配置
- ✅ `config/mod.rs` - 模块导出

3. ✅ Core Error 错误定义 ⚠️

- ✅ `core/error.rs` - 完善业务错误类型
- ✅ `.env.example` - 环境变量模板

---

## 🎯 阶段 2：Service 层（核心业务）

从最基础的认证开始，逐步扩展：

1. AuthService 🔐（最优先）

- `core/auth/service.rs` - 登录、注册、Token 验证
- `core/auth/jwt.rs` - JWT 生成/验证工具
- `core/auth/password.rs` - Argon2 密码哈希

2. UsersService 👤

- `core/users/service.rs` - 用户管理
- `core/users/dto.rs` - 用户相关 DTO

3. PostsService 📝

- `core/posts/service.rs` - 文章 CRUD、发布流程
- `core/posts/dto.rs` - 文章相关 DTO

4. 其他 Service（按需）

- CategoriesService、TagsService
- CommentsService（评论树构建）
- LinksService

---

## 🎯 阶段 3：API 层（HTTP 接口）

有了 Service 才能写 Handler：

1. Middleware 🛡️

- `api/middleware/auth.rs` - JWT 验证中间件
- `api/middleware/admin.rs` - Admin 角色检查

2. 公开接口 🌐

- `api/http/auth.rs` - 登录/注册
- `posts.rs` - 文章列表/详情
- `categories.rs`、`tags.rs`

3. 认证接口 👤

- `api/http/user/profile.rs` - 用户资料管理

4. 管理接口 🛠️

- `posts.rs` - 后台文章管理
- 其他管理接口...

---

## 🎯 阶段 4：应用入口 🚀

- `main.rs` - 初始化、启动服务器、路由注册
