<div align="center">

# YukiLog 前端架构文档

Astro 混合渲染 + 组件化 + 分层配置的整体设计

</div>

---

## 技术栈

| 领域 | 方案 |
| --- | --- |
| 框架 | **Astro 5.x**（混合渲染） |
| 适配器 | `@astrojs/node`（standalone 模式） |
| 样式 | **Tailwind CSS 4** + **SCSS**（`sass`） |
| Markdown | `marked` + `shiki`（代码高亮）+ `katex`（数学公式）+ `marked-footnote` |
| 编辑器 | `Vditor`（管理后台文章编辑, CDN 加载） |
| UI 框架 | Vue 3（通过 `@astrojs/vue`, 当前未大量使用） |
| 图片优化 | `sharp` |

---

## 渲染策略

Astro 全局 `output: 'static'`, 动态页面通过 `export const prerender = false` 逐页标记为 SSR:

| 模式 | 页面 |
| --- | --- |
| **SSG**（构建时生成） | `/about`, `/404`, `/500` |
| **SSR**（服务端渲染） | `/`, `/posts/:slug`, `/themes`, `/themes/:slug`, `/tags`, `/archive`, `/links`, `/admin/*` |

SSR 页面在 frontmatter 中调用后端 API 获取数据, 若 API 失败则重定向到 `/500`

---

## 目录结构

```text
yukilog-hanakoi/
├── yukilog.config.ts       ← 全局唯一配置源
├── astro.config.mjs        ← Astro + Vite 配置
├── .env.example            ← 环境变量模板
├── package.json
├── docs/                   ← 你正在看的文档
└── src/
    ├── assets/icon/        ← SVG 图标（ui / nav / social）
    ├── components/         ← Astro 组件
    │   ├── blog/           （暂空）
    │   ├── comments/       评论相关（5 个组件）
    │   ├── home/           首页卡片（6 个组件）
    │   ├── links/          友链相关（3 个组件）
    │   ├── navigation/     导航栏（2 个组件）
    │   ├── posts/          文章详情（1 个组件）
    │   └── shared/         共享组件（7 个组件）
    ├── layouts/            ← 布局组件
    │   ├── BaseLayout.astro
    │   ├── BlogLayout.astro
    │   └── AdminLayout.astro
    ├── lib/                ← 工具函数和 API 封装
    ├── pages/              ← 路由页面
    │   ├── admin/          管理后台页面
    │   ├── posts/          [slug].astro 文章详情
    │   └── themes/         [slug].astro 主题详情
    ├── styles/             ← 全局样式
    └── types/              ← TypeScript 类型定义
```

---

## 数据流

```text
┌─────────────────────────────────────────────────────┐
│  yukilog.config.ts                                   │
│  siteConfig / navItems / contentConfig / designTokens│
└─────────────┬───────────────────────────────────────┘
              │ import
              ▼
┌─────────────────────────┐    ┌────────────────────┐
│  src/lib/config.ts       │    │ src/lib/api.ts      │
│  re-export + API_BASE_URL│    │ fetchApi 封装       │
└─────────────┬───────────┘    └──────┬─────────────┘
              │                       │ fetch()
              ▼                       ▼
┌─────────────────────────────────────────────────────┐
│                     页面层                            │
│  src/pages/*.astro                                   │
│  · Frontmatter: 调用 API, 处理数据, 错误兜底         │
│  · Template: 组装组件, 传入 Props                     │
│  · Script: 客户端交互逻辑                             │
└─────────────┬───────────────────────────────────────┘
              │ Props
              ▼
┌─────────────────────────────────────────────────────┐
│                    组件层                             │
│  src/components/**/*.astro                           │
│  · 纯展示: 接收 Props → 渲染 HTML + Scoped SCSS      │
│  · 交互型: 含 <script> 处理 DOM 事件、调用 API       │
└─────────────────────────────────────────────────────┘
```

---

## 布局继承

```text
BaseLayout
├── BlogLayout (NavBar + SearchOverlay + ScrollProgress + Footer)
│   ├── 首页
│   ├── 文章详情
│   ├── 主题列表/详情
│   ├── 标签
│   ├── 归档
│   ├── 友链
│   └── 关于
└── AdminLayout (侧边栏 + 顶栏)
    ├── 仪表盘
    ├── 文章管理
    ├── 评论管理
    ├── 主题管理
    ├── 标签管理
    └── 友链管理
```

---

## 构建与部署

### 开发

```bash
pnpm dev        # 本地开发服务器 (http://localhost:4321)
```

### 构建

```bash
pnpm build      # 生成 dist/ 目录
```

构建产物:
* `dist/client/` — 静态资源（CSS / JS / 图片）
* `dist/server/entry.mjs` — Node.js 服务端入口

### 运行

```bash
node dist/server/entry.mjs   # 启动生产服务器
```

### 预览

```bash
pnpm preview    # 本地预览构建产物
```

---

## 前后端对接

前端通过环境变量 `PUBLIC_API_URL` 连接后端:

```text
前端 (Astro)                              后端 (Rust Axum)
┌──────────┐   HTTP / JSON   ┌──────────────────┐
│ :4321    │ ──────────────→ │ :3000            │
│          │                 │ /api/public/*    │
│          │                 │ /api/admin/*     │
└──────────┘                 └──────────────────┘
```

* SSR 页面的 API 调用在**服务端 frontmatter** 中执行（Node.js → 后端, 同机器时延极低）
* 客户端交互（评论提交、搜索等）通过浏览器直接调用后端 API
* JWT Token 存储在浏览器 `localStorage` 中

---

## 设计理念

YukiLog 的前端设计遵循"**信笺**"风格:

* **色调**: 蓝粉双色系 —— 代码的理性（蓝）与情绪的温柔（粉）
* **质感**: 大圆角（16-28px）、柔和阴影、暖色信纸底色
* **动效**: 温柔节奏（300ms 标准、gentle 缓动）
* **排版**: 中文优先字体栈, 充裕行高 (1.6-1.8)
* **交互**: 页面失焦时标题变化、散落式卡片布局、时间轴归档
