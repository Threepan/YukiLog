---
id: 19
title: "YukiLog - 14 - 从 Astro 到 SvelteKit：一次不留退路的前端迁移"
slug: "yukilog-14-astro-sveltekitfrontend"
summary: "整个前端推倒重来。不是渐进式的，不是并行运行的——删掉旧代码，换上新框架。这篇记录为什么要迁移、怎么迁移、以及过程中踩的坑"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-14-astro-sveltekitfrontend.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["astro", "blog", "sveltekit"]
view_count: 371
created_at: "2026-03-17 00:10:30"
updated_at: "2026-08-17 15:51:52"
source: "https://blog.yeastar.xin/posts/yukilog-14-astro-sveltekitfrontend"
---

## 为什么要换

第 9 篇里我写过一句话：大部分页面都标了 `prerender: false`。

这句话在 Astro 的语境里很违和。Astro 的核心卖点是**内容站点生成器**——它的设计假设是大部分页面可以在构建时生成静态 HTML，只在需要交互的地方插入动态组件（岛屿架构）。

但 YukiLog 是一个 API 驱动的 CMS。文章内容在数据库里，每次访问都要请求后端。"构建时生成"这个假设从根基上就不成立。

结果是，我在一个为静态站点设计的框架里，手动把每个页面标记成"不要预渲染"。数据加载写在 `.astro` 文件的 frontmatter 里——那段代码夹在 YAML 分隔符和 HTML 模板之间，既不是模板，也不是脚本，是 Astro 特有的"组件顶部代码块"。

```astro
---
// 这是 Astro 的数据加载方式
// 在 frontmatter 区域直接写 await
const posts = await fetchApi('/api/public/posts');
---

{posts.map(p => )}
```

SvelteKit 的数据加载是一等公民：

```typescript
// +page.server.ts — 专门为此设计的文件
export async function load({ params }) {
    const posts = await fetchApi('/api/public/posts');
    return { posts };
}
```

两者功能相同，但 SvelteKit 的 `load` 函数有类型推导、错误边界、数据依赖声明，是框架核心 API 而不是一个"也能用"的附带功能。

其他促成迁移的因素：

- **Vue 依赖**：Astro 版的交互组件用 Vue 写，意味着构建时需要同时处理 `.astro`、`.vue` 两种组件格式。Svelte 组件一种格式解决所有问题
- **脚本生命周期**：Astro 的 `` 标签行为和 View Transitions 的交互很微妙——页面切换后脚本是否重新执行、DOM 引用是否还有效、全局清理函数怎么管理——第 9 篇已经讨论过这些问题
- **Svelte 5 的 rune 语法**：`$state`、`$derived`、`$effect` 比 Vue 的 `ref()` / `computed()` / `watch()` 更直觉，编译期消化掉响应式 runtime

---

## 迁移策略

一开始考虑过渐进式迁移——Astro 和 SvelteKit 并行运行在不同端口，逐页切换。

最后选了直接替换。原因：

1. 并行运行意味着维护两套路由、两套布局、两套组件，短期内工作量反而翻倍
2. Astro 版的代码在 `astro_yukilog` 分支上有完整快照，随时可以回退
3. 两个前端共享同一个后端 API，接口不变，切换的只是渲染层

分支策略：

```
main (生产)
  ├── astro_yukilog    — Astro 代码冻结归档
  └── sveltekit_yukilog — SvelteKit 开发
```

开发完成后 `sveltekit_yukilog` 合并到 `main`，直接替换。`astro_yukilog` 长期保留作为保险。

---

## 可以直接搬的东西

大概百分之四十的代码与框架无关：

- `lib/api.ts`——所有 API 封装，纯 TypeScript 函数，不依赖任何框架
- `lib/markdown.ts`——marked + shiki + katex 渲染管线
- `lib/date.ts`、`lib/avatar.ts`、`lib/slugify.ts`——工具函数
- `types/`——全部 TypeScript 类型定义
- `yukilog.config.ts`——站点配置

这些文件从旧项目复制过来，零修改就能用。这也是第 7、8 篇反复强调"API 层是纯 TypeScript"的回报——不绑定框架的代码，换框架时价值最大。

---

## 需要重写的东西

剩下百分之六十是 `.astro` 和 `.vue` 文件，需要转写成 `.svelte`。

### 布局

Astro 用两层布局：`BaseLayout.astro`（HTML 外壳 + SEO）和 `BlogLayout.astro`（导航栏 + 页脚 + 内容 slot）。

SvelteKit 只需要一个 `+layout.svelte`：

```svelte
<NavBar stickyOnly={!isHome} />
<SearchOverlay />
<ScrollProgress />
<MusicPlayer />

<main>
    {@render children()}
</main>

<Footer />
```

layout 在 SvelteKit 里不会在页面导航间销毁重建——导航栏状态、音乐播放器、搜索覆盖层都自然保持。这比 Astro 的 `transition:persist` 属性优雅。

### 导航栏

导航栏的核心行为：首屏（0-100vh）隐藏导航栏，鼠标靠近顶部才展开；滚过首屏后变为 sticky 圆角模式。

Astro 版用 `<script>` 标签 + `astro:page-load` 事件在每次页面导航后重新初始化。SvelteKit 版用 `$effect`：

```svelte
$effect(() => {
    if (!navbar || !navbarFixed) return;
    navbar.classList.remove('sticky', 'active');
    navbarFixed.classList.remove('hidden');

    if (stickyOnly) {
        navbar.classList.add('sticky');
        navbarFixed.classList.add('hidden');
        return;
    }

    // ... 绑定 scroll / mousemove 事件
    return () => { /* 清理 */ };
});
```

`$effect` 的关键优势：当 `stickyOnly` prop 变化时（比如从非首页导航回首页），effect 自动重新执行，事件监听器自动销毁重建。Astro 版需要手动管理全局 cleanup 函数（`window.__yukilogNavbarCleanup`），SvelteKit 的响应式系统帮你做了这件事。

### 首页六个卡片

WelcomeCard、ProfileCard、HitokotoCard、SiteInfoCard、ArticleList、ArticleCard——每个从 `.astro` + `.vue` 转写成 `.svelte`。

主要变化是响应式系统：Vue 的 `ref()` / `onMounted()` → Svelte 的 `$state` / `onMount`。模板语法差异不大，`v-for` → `{#each}`，`v-if` → `{#if}`，`v-bind:class` → `class:` 指令。

### 管理后台

11 个管理页面（仪表盘、登录、文章列表/新建/编辑、随记、评论、主题、标签、友链）。这是工作量最大的部分，但逻辑最简单——都是表单和列表，没有复杂的交互。

Vditor 编辑器（文章用的 Markdown 编辑器）改为 CDN 动态加载，不打入构建产物。`onMount` 里创建 `<script>` 标签，加载完成后初始化编辑器。

---

## 踩的坑

### Vite 的 `fs.allow`

`yukilog.config.ts` 放在项目根目录（`yukilog-hanakoi/` 的上级），SvelteKit 的 `src/lib/config.ts` 通过 `import` 引用它。

本地开发正常，但换一个环境就 500 了——Vite 默认只允许访问项目根目录内的文件。解法是在 `vite.config.ts` 里显式放行：

```typescript
server: {
    fs: { allow: ['..'] }
}
```

这个问题隐蔽的地方在于，它不是类型错误或模块找不到，而是 Vite 安全策略拒绝文件访问——错误信息只在终端打印，浏览器看到的是导航栏和按钮全部消失（因为配置加载失败，所有依赖配置的组件都渲染不出来）。

### 侧栏响应式断点

首页在 1400px 以下把 ProfileCard 从固定侧栏变成流式布局。文章详情页也有一个 ProfileCard 侧栏，但它的隐藏断点写的是 1024px。

结果在 1024px 到 1400px 之间，文章页的侧栏和正文内容重叠。修复很简单——统一断点到 1400px——但发现这个问题花了比修复更多的时间。

### 后端不可用时的崩溃

开发环境经常不开后端。Astro 版会跳到 `/500` 错误页、SvelteKit 版直接白屏——`+page.server.ts` 的 `load` 函数 throw 了，没有 fallback。

解法分两层：

1. 列表页的 `load` 函数不再 throw，改为返回空数据
2. `fetchApi` 加入自动 mock fallback——检测到 `ECONNREFUSED` 后，动态 import mock 数据模块，后续所有请求走 mock

```typescript
if (err?.cause?.code === 'ECONNREFUSED') {
    useMock = true;
    const mock = await loadMock(endpoint, options);
    if (mock !== undefined) return mock as T;
}
```

mock 模块用动态 import 而非静态 import，确保生产构建不会把测试数据打进去。

---

## 构建优化

迁移完成后首次部署，构建时间 28 秒。检查构建日志，发现几个问题：

**6.9MB 图片经过 Vite 处理**——三张壁纸和一个 GIF 通过 `import.meta.glob` 导入，Vite 会给每张图计算 hash、复制到 client 和 server 两个输出目录。总共 13.8MB 的文件复制操作。

解法：把它们移到 `static/` 目录，用纯 URL 字符串引用。Vite 不处理 `static/` 下的文件。

**shiki 预加载 18 种语言**——博客常用的其实只有 10 种不到。去掉 `scss`、`markdown`、`yaml`、`shell`、`python`、`vue`、`jsx`、`tsx`。未预加载的语言只是没有语法高亮，不会报错。

**vditor 在 package.json 里**——但实际是通过 CDN 动态加载的，`pnpm install` 白下载了一个没人 import 的包。

优化后：28 秒 → 4 秒。

---

## 遗漏的功能

迁移不是翻译——逐文件对照转写很容易漏掉那些「不在任何一个组件文件里」的行为。

**页面失焦标题变化**——Astro 版在 `BaseLayout.astro` 的 `<script>` 标签里，用 `visibilitychange` 事件把标题改成"...你 ... 要走了吗?"。迁移时 layout 逻辑完全重写了，这段脚本就丢了。

**背景音乐播放器**——`MusicPlayer.astro` 是一个独立组件，嵌在 `BlogLayout.astro` 里。迁移时 layout 结构变了，这个组件没有被列入迁移清单。

**favicon**——Astro 的 `public/favicon.ico` 对应 SvelteKit 的 `static/favicon.ico`。目录名不同，文件没跟过来。

这类问题很难靠代码审查发现。你 diff 新旧代码时看到的是"这个文件被重写了"，但不会看到"这个文件根本没出现在新项目里"。发现它们的方式是：部署上线后，用浏览器一个页面一个页面地点。

---

## 部署切换

SvelteKit 用 `adapter-node` 构建成 Node.js 服务（`build/index.js`），部署模型和 Astro 的 `@astrojs/node`（`dist/server/entry.mjs`）几乎相同。

要改的是 systemd 服务文件里的入口路径、构建产物检测目录（`dist/` → `build/`）、Nginx 静态资源路由（`/_astro/` → `/_app/`）。

一个低级错误：`deploy.sh` 检测到已有的 service 文件就跳过创建。旧文件指向 `dist/server/entry.mjs`，新构建产物在 `build/index.js`——服务启动后找不到 `vue` 模块直接崩溃（因为跑的还是旧代码的 Astro 入口）。

修复后改成了每次部署都重写前端 service 文件，不再 skip-if-exists。

---

## 回头看

框架迁移的真正成本不是"重写每个组件"——那是机械性劳动。真正的成本是：

1. **发现遗漏**——你以为迁完了，但总有几个边角功能没跟过来
2. **适配差异**——两个框架对同一个概念的处理方式不同（`onMount` vs `$effect`、layout 生命周期、脚本执行时机）
3. **环境一致性**——本地能跑不代表服务器能跑（Vite fs.allow、systemd 入口路径）

好处也是真实的：代码量减少了（不再有 `.astro` + `.vue` 两种组件），构建速度快了（4 秒 vs 旧版 18 秒），数据加载模式和框架设计一致了而不是在"反着用"。

这可能是 YukiLog 最后一次框架迁移——但谁知道呢，第 1 篇也说过"最终选择了 Astro"。
