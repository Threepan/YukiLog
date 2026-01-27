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

### ✅ Phase 1: AuthService 🔐（最优先）

- ✅ `core/auth/password.rs` - Argon2 密码哈希/验证
- ✅ `core/auth/jwt.rs` - JWT 生成/验证工具（HS256, 15min/7天）
- ✅ `core/auth/dto.rs` - LoginRequest, LoginResponse, UserInfo
- ✅ `core/auth/service.rs` - login(), refresh_token(), verify_token()
- ✅ `core/auth/mod.rs` - 模块导出

### Phase 2: UsersService 👤

- `core/users/dto.rs` - CreateUserRequest, UpdateProfileRequest, ChangePasswordRequest, UserDetailResponse
- `core/users/service.rs` - create_user(), update_profile(), change_password(), update_role()
- `core/users/mod.rs` - 模块导出

### Phase 3: CategoriesService 🏷️

- `core/categories/dto.rs` - CreateCategoryRequest, CategoryResponse
- `core/categories/service.rs` - create(), update(), delete(), get_all_with_count()
- `core/categories/mod.rs` - 模块导出

### Phase 4: TagsService 🔖

- `core/tags/dto.rs` - CreateTagRequest, TagResponse
- `core/tags/service.rs` - create(), update(), delete(), find_or_create_batch()
- `core/tags/mod.rs` - 模块导出

### Phase 5: PostsService 📝（最复杂）

- `core/posts/dto.rs` - CreatePostRequest, PostDetailResponse, PostListItemResponse
- `core/posts/service.rs` - create_post(), update_post(), publish_post(), get_post_by_slug(), sync_tags()
- `core/posts/mod.rs` - 模块导出

### Phase 6: CommentsService 💬

- `core/comments/dto.rs` - CreateCommentRequest, CommentNode, CommentResponse
- `core/comments/service.rs` - create_comment(), build_comment_tree(), review_comment(), batch_review()
- `core/comments/mod.rs` - 模块导出

### Phase 7: LinksService 🔗
x
- `core/links/dto.rs` - ApplyLinkRequest, LinkResponse
- `core/links/service.rs` - apply_link(), approve_link(), batch_update_status()
- `core/links/mod.rs` - 模块导出

### 通用工具 🔨

- `common/dto.rs` - PaginatedResponse<T> 通用分页响应
- 更新 `lib.rs` - 添加所有 Service 模块声明

### 依赖项补充 📦

需要在 `Cargo.toml` 添加：
- `argon2 = "0.5"` - 密码哈希
- `jsonwebtoken = "9"` - JWT
- `chrono = { version = "0.4", features = ["serde"] }` - 时间处理
- `validator = { version = "0.18", features = ["derive"] }` - 输入校验

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
