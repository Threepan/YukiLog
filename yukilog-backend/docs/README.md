## 阶段 1️⃣ 基础设施层 (Foundation)

优先级	模块	文件	说明
1.1	配置	mod.rs, app.rs, database.rs	环境变量、数据库连接
1.2	错误	core/error.rs	AppError 定义
1.3	响应	common/response.rs, pagination.rs	ApiResponse 封装

## 阶段 2️⃣ Repository 层 (infra/)

优先级	模块	说明
2.1	repository.rs	通用 Repository<E> trait 定义
2.2	users.rs	UserRepository (登录依赖)
2.3	posts.rs	PostRepository (核心)
2.4	categories.rs, tags.rs	简单 CRUD
2.5	comments.rs	嵌套评论查询
2.6	links.rs	友链

## 阶段 3️⃣ Service 层 (core/)

优先级	模块	依赖
3.1	auth/	JWT + Argon2 + UserRepository
3.2	posts/	PostRepository + TagRepository
3.3	categories/, tags/	对应 Repository
3.4	comments/	评论树构建逻辑
3.5	links/, users/	剩余服务

## 阶段 4️⃣ HTTP 层 (api/)

优先级	模块	说明
4.1	middleware/auth.rs	AuthGuard
4.2	middleware/admin.rs	AdminGuard
4.3	extractors/	CurrentUser, Pagination
4.4	http/auth.rs	登录接口 (端到端验证)
4.5	posts.rs	公开文章接口
4.6	http/admin/*	管理接口
4.7	router.rs	路由组装

## 阶段 5️⃣ 集成启动

优先级	模块	说明
5.1	main.rs	AppState 构建、服务启动
5.2	测试	端到端测试
