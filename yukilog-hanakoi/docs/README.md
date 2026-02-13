<div align="center">

# YukiLog 前端文档

`yukilog-hanakoi` — 恋的博客前端, 基于 Astro 混合渲染

</div>

---

## 文档索引

| 文档 | 内容 |
| --- | --- |
| [architecture.md](./architecture.md) | **架构总览** — 技术栈、渲染策略、目录结构、数据流、构建部署 |
| [api.md](./api.md) | **API 封装** — fetchApi 通用封装、所有前/后台 API 调用方法、JWT 认证流程 |
| [pages.md](./pages.md) | **前台页面** — 首页/文章/主题/标签/归档/友链/关于/错误页, 含数据来源与组件组合 |
| [pages-admin.md](./pages-admin.md) | **管理后台页面** — 登录/仪表盘/文章/评论/主题/标签/友链管理, 含 AdminLayout 布局 |
| [components.md](./components.md) | **组件文档** — 全部 24 个组件的职责、Props、交互逻辑 |
| [config.md](./config.md) | **样式与配置** — 三层配置体系、SCSS 变量、SVG 图标系统、环境变量 |
| [lib.md](./lib.md) | **工具函数与类型** — auth/markdown/utils/date/avatar/slugify + TypeScript 类型定义 |
| [roadmap.md](./roadmap.md) | **后续功能规划** — 分页排序、懒加载、暗色模式、Toast、部署脚本等 |

---

## 快速上手

```bash
# 1. 安装依赖
pnpm install

# 2. 配置环境变量
cp .env.example .env
# 编辑 .env 设置 PUBLIC_API_URL

# 3. 启动开发服务器
pnpm dev

# 4. 构建
pnpm build

# 5. 生产运行
node dist/server/entry.mjs
```

**前提：** 需要后端服务（Rust Axum）在 `PUBLIC_API_URL` 地址运行

---

## 项目概览

```text
前台页面 (7)                管理后台 (7)              组件 (24)
├ /          首页            ├ /admin/login  登录      ├ 布局 ×3
├ /posts/:slug 文章详情      ├ /admin        仪表盘     ├ 导航 ×2
├ /themes    主题列表        ├ /admin/posts  文章管理   ├ 首页 ×6
├ /themes/:slug 主题详情     ├ /admin/posts/new 新建    ├ 评论 ×5
├ /tags      标签           ├ /admin/posts/edit/:slug  ├ 友链 ×3
├ /archive   归档            ├ /admin/comments 评论    ├ 文章 ×1
├ /links     友链            ├ /admin/themes  主题     └ 共享 ×7 (含 3 错误页)
├ /about     关于            ├ /admin/tags    标签
├ /404       404            └ /admin/links   友链
└ /500       500
```
