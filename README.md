<div align="center">

## YukiLog ❄️

</div>

**YukiLog** 是一个追求极致性能与完全自控力的全栈博客系统。
它抛弃了开箱即用的成品框架，采用 **Rust (Backend) + Astro (Frontend)** 的现代架构，旨在通过高性能的底层技术与精致的前端视觉效果，打造一个属于开发者个人的数字花园。

---

## 🏗️ 整体架构 (Architecture)

**YukiLog** 采用前后端分离架构，通过 **RESTful API** 进行通信，确保系统的高可扩展性。

- **后端 (The Heart):** 基于 `Rust` 语言，利用 `Axum` 处理高并发请求，`SeaORM` 进行类型安全的数据库操作。
- **前端 (The Face):** 采用 `Astro` 岛屿架构，核心页面 0-JS 加载，动态交互（如评论、搜索）由 **Vue 3** 驱动，样式基于 **Tailwind CSS**。
- **数据库 (The Memory):** **PostgreSQL 16**，包含自研的无限层级评论系统模型。

---

## 🛠️ 技术栈 (Tech Stack)
**Backend (Rust)**
- **Framework:** `Axum`
- **Runtime:** `Tokio`
- **ORM:** `SeaORM`
- **Security:** `Argon2` + `JWT`

**Frontend (TypeScript)**
- **Static Site Engine:** `Astro`
- **UI Framework:** `Vue 3`
- **Styling:** `Tailwind CSS` + `SCSS`

**Infrastructure**
- **Database:** `PostgreSQL`

---

## 📖 文档 (Docs)

[YukiLog 设计文档](./yukilog-backend/docs/yukilog.md)

[YukiLog 数据库映射文档](./yukilog-backend/docs/orm.md)

[YukiLog 仓储层定义文档](./yukilog-backend/docs/repo.md)

[YukiLog 业务层封装文档](./yukilog-backend/docs/service.md)

[YukiLog 运行时配置文档](./yukilog-backend/docs/config.md)

[YukiLog 处理层规范文档](./yukilog-backend/docs/handler.md)

[YukiLog 公共接口文档](./yukilog-backend/docs/handler_public.md)

[YukiLog 管理接口文档](./yukilog-backend/docs/handler_admin.md)

[YukiLog 应用层文档](./yukilog-backend/docs/axum.md)

---

## 📄 License
本项目采用组合授权协议：

- Source Code is licensed under GNU AGPL-3.0
    - 意味着如果你对源代码进行了修改并用于云服务，你需要公开你的源代码。

- Blog Content and Creative Materials are licensed under CC BY-NC-SA 4.0
    - 署名-非商业性使用-相同方式共享。
