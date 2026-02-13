<div align="center">

## YukiLog ❄️

</div>

**YukiLog** 是一个追求极致性能与完全自控力的全栈博客系统。
它抛弃了开箱即用的成品框架，采用 **Rust (Backend) + Astro (Frontend)** 的现代架构，旨在通过高性能的底层技术与精致的前端视觉效果，打造一个属于开发者个人的数字花园。

---

## 🏗️ 整体架构 (Architecture)

**YukiLog** 采用前后端分离架构，通过 **RESTful API** 进行通信，确保系统的高可扩展性。

* **后端 (The Heart):** 基于 `Rust` 语言，利用 `Axum` 处理高并发请求，`SeaORM` 进行类型安全的数据库操作。
* **前端 (The Face):** 采用 `Astro` 岛屿架构，核心页面 0-JS 加载，动态交互（如评论、搜索）由 **Vue 3** 驱动，样式基于 **Tailwind CSS**。
* **数据库 (The Memory):** **PostgreSQL 16**，包含自研的无限层级评论系统模型。

---

## 🛠️ 技术栈 (Tech Stack)
**Backend (Rust)**
* **Framework:** `Axum`
* **Runtime:** `Tokio`
* **ORM:** `SeaORM`
* **Security:** `Argon2` + `JWT`

**Frontend (TypeScript)**
* **Static Site Engine:** `Astro`
* **UI Framework:** `Vue 3`
* **Styling:** `Tailwind CSS` + `SCSS`

**Infrastructure**
* **Database:** `PostgreSQL`

---

## 📖 文档 (Docs)

#### | YukiLog-BackEnd 后端

[YukiLog 设计文档](./yukilog-backend/docs/yukilog.md)

[YukiLog API文档](./yukilog-backend/docs/api.md)

[YukiLog 保留接口文档](./yukilog-backend/docs/_api.md)

[YukiLog 数据库映射文档](./yukilog-backend/docs/orm.md)

[YukiLog 仓储层定义文档](./yukilog-backend/docs/repo.md)

[YukiLog 业务层封装文档](./yukilog-backend/docs/service.md)

[YukiLog 运行时配置文档](./yukilog-backend/docs/config.md)

[YukiLog 处理层规范文档](./yukilog-backend/docs/handler.md)

[YukiLog 公共接口文档](./yukilog-backend/docs/handler_public.md)

[YukiLog 管理接口文档](./yukilog-backend/docs/handler_admin.md)

[YukiLog 应用层文档](./yukilog-backend/docs/axum.md)

[YukiLog 网络路由文档](./yukilog-backend/docs/route.md)

#### | YukiLog-Hanakoi 前端

[YukiLog 前端文档索引](./yukilog-hanakoi/docs/README.md)

[YukiLog 前端架构文档](./yukilog-hanakoi/docs/architecture.md)

[YukiLog 前端API封装文档](./yukilog-hanakoi/docs/api.md)

[YukiLog 前台页面文档](./yukilog-hanakoi/docs/pages.md)

[YukiLog 管理后台页面文档](./yukilog-hanakoi/docs/pages-admin.md)

[YukiLog 组件文档](./yukilog-hanakoi/docs/components.md)

[YukiLog 样式与配置文档](./yukilog-hanakoi/docs/config.md)

[YukiLog 工具函数与类型文档](./yukilog-hanakoi/docs/lib.md)

[YukiLog 后续功能规划](./yukilog-hanakoi/docs/roadmap.md)

---

## � 部署 (Deploy)

YukiLog 提供一键部署脚本，自动完成环境检测、依赖安装、构建、服务注册和 SSL 配置。

```bash
# 克隆仓库后在根目录执行
chmod +x deploy.sh
sudo ./deploy.sh
```

脚本会自动完成以下流程：

1. **交互式收集** — 域名、数据库配置、管理员账号密码
2. **端口自动检测** — 从默认端口开始递增扫描，避免占用已有服务
3. **依赖安装** — PostgreSQL / Redis / Nginx / Certbot / Rust / Node.js / pnpm（已安装则跳过）
4. **数据库初始化** — 创建用户、数据库、导入表结构（已存在则跳过）
5. **生成 `.env`** — 后端 + 前端环境变量文件（已存在则跳过）
6. **构建** — `cargo build --release`（后端）+ `pnpm build`（前端）
7. **Systemd 服务** — 注册 `yukilog-backend` / `yukilog-hanakoi` 服务并启动
8. **Nginx 反向代理** — 自动生成配置、创建软链、重载
9. **SSL 证书** — Let's Encrypt (certbot) 自动申请，HTTPS 一键启用

> 所有操作均为幂等设计：已存在的文件 / 服务 / 数据库不会被覆写，可安全重复执行。

---

## �📄 License
本项目采用组合授权协议：

* Source Code is licensed under GNU AGPL-3.0
    * 意味着如果你对源代码进行了修改并用于云服务，你需要公开你的源代码。

* Blog Content and Creative Materials are licensed under CC BY-NC-SA 4.0
    * 署名-非商业性使用-相同方式共享。
