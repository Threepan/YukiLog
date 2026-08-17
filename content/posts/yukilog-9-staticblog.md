---
id: 13
title: "YukiLog - 9 - 静态优先，但不是真的静态"
slug: "yukilog-9-staticblog"
summary: "Astro 默认生成静态页面，但 YukiLog 几乎所有页面都标记了 prerender: false——这篇讲的是为什么，以及 View Transitions 带来的一个隐藏问题"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-9-staticblog.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["astro", "blog", "教程"]
view_count: 261
created_at: "2026-02-20 21:27:24"
updated_at: "2026-08-17 15:51:47"
source: "https://blog.yeastar.xin/posts/yukilog-9-staticblog"
---

## 从"静态优先"开始

Astro 的设计哲学是静态优先：默认情况下，所有页面在构建时生成 HTML，部署后直接由 CDN 或静态服务器分发，不需要运行时服务器。

`astro.config.mjs` 里只有一行关键配置：

```javascript
export default defineConfig({
  output: 'static',  // 默认静态，admin 页面通过 prerender: false 标记为动态
  adapter: node({ mode: 'standalone' }),
  integrations: [vue()],
});
```

`output: 'static'` 意味着构建时生成所有页面。但紧接着有一个 `adapter: node`——这两个同时存在，说明这个项目并不是纯静态的。

---

## 几乎所有页面都是动态的

打开 `src/pages/` 目录，几乎每个页面文件的顶部都有这一行：

```astro
export const prerender = false;
```

首页、归档、标签、主题、文章详情、友链——全都有。只有 404 页面没有。

原因很直接：这些页面都需要在请求时从后端 API 拿数据。

```astro
---
export const prerender = false;

// 请求时从后端获取文章列表
const posts = await postsApi.list({ page, sort });
---
```

如果用静态生成，构建时就要把所有文章、所有标签、所有评论全部拉下来渲染成 HTML。这意味着每次发布新文章都要重新构建整个站点。对于一个内容持续更新的博客来说，这不现实。

`prerender = false` 让这些页面变成 SSR——每次请求时在服务器上实时渲染，拿到最新数据。

---

## 一个迟来的醒悟

在开发初期，我觉得"Astro 静态 + Vue 动态"是完美的组合——静态博客加动态更新，听起来正好符合博客系统的需求。

但开发到后期，我突然意识到一件事：那些所谓的"博客框架"，大多数根本就没有后端。

Hexo、Hugo、Jekyll——它们的博文存在文件里，前端框架读取 Markdown，构建时一次性编译成 HTML，部署到 CDN，完事。Vue 和 React 在这些框架里能做到"动态"，是因为它们本来就是纯前端项目，不需要处理真正的服务端请求。那是前端框架在纯前端环境里的"特权"。

我和这些框架的作者不一样。我从一开始就是全栈开发者，思维方式是前后端分离：前端负责展示，后端负责数据，数据库负责存储，三者通过 API 协作。YukiLog 从设计第一天起就有 Rust 后端、PostgreSQL 数据库、JWT 鉴权——它首先是一个 CMS 系统，然后才是一个"博客"。

这意味着它一定不是"一次性编译出静态页面"的系统。每次访问都需要从数据库拿最新数据，每次提交评论都需要后端处理，每次发布文章都需要通过管理后台写入数据库。这是一个动态 Web 站点，只是碰巧用来写博客。

---

这个认知让我重新审视了一个更大的问题：现代前端框架越来越能做"后端的事"——连接数据库、处理 API 请求、服务端渲染、边缘函数。Next.js 的 Server Actions 可以直接在前端组件里写数据库查询，Nuxt 有 server routes，SvelteKit 有 form actions。

这是前端对后端职责的渗透，还是前端在进化过程中的必然结果？

我倾向于认为两者都有。对于纯前端项目来说，这些能力让他们不需要维护一个独立的后端服务，是合理的简化。但对于真正需要后端的系统——复杂的业务逻辑、严格的权限控制、多服务协作——把这些逻辑塞进前端框架的"server"层，只是换了个地方写后端，并没有真正解决问题。

YukiLog 选择了清晰的边界：Astro 负责渲染，Rust 负责业务，PostgreSQL 负责存储。前端不越界，后端不缺席。

---

## 真正静态的只有 404

404 页面没有 `prerender = false`，它是真正的静态页面：

```astro
---
import BlogLayout from "../layouts/BlogLayout.astro";
import { contentConfig } from "../lib/config";

const e404 = contentConfig.pages.error404;
---
```

没有 API 调用，没有动态数据，构建时生成一次，永远不变。这是合理的——404 页面的内容不依赖任何运行时状态。

---

## `output: 'static'` + `prerender: false` 的组合

这个组合乍看矛盾，但 Astro 的设计是：

* `output: 'static'`：默认行为是静态生成
* `prerender: false`：单个页面的例外，覆盖默认行为，变成 SSR

反过来也可以：`output: 'server'`（默认全部 SSR）+ `export const prerender = true`（单个页面例外，变成静态）。

YukiLog 选择了"静态为默认，动态为例外"的方向，即使实际上大部分页面都是动态的。这个选择的好处是：如果以后有纯静态的页面（比如关于页面、隐私政策），不需要任何额外配置，默认就是静态的。

---

## View Transitions 和它带来的问题

`BaseLayout.astro` 里有一行：

```astro

```

这是 Astro 的 View Transitions 支持。它让页面切换变成类似 SPA 的体验——点击链接时不是完整的页面刷新，而是只替换变化的部分，同时可以添加过渡动画。

但它带来了一个隐藏问题：**脚本的生命周期变了**。

在普通的多页面应用里，每次导航都是完整的页面加载，`` 标签里的代码每次都会重新执行。但 View Transitions 开启后，导航时 DOM 被局部替换，脚本不会自动重新执行。

这意味着，如果一个组件的脚本在页面加载时初始化了某些东西（比如 `IntersectionObserver`、事件监听器），切换页面后这些初始化就消失了。

解决方案是监听 `astro:page-load` 事件：

```javascript
// 错误：只在第一次加载时执行
const cards = document.querySelectorAll('.theme-card');
const observer = new IntersectionObserver(...);
cards.forEach(card => observer.observe(card));

// 正确：每次页面加载（包括 View Transitions 后）都执行
document.addEventListener('astro:page-load', () => {
  const cards = document.querySelectorAll('.theme-card');
  if (!cards.length) return;
  const observer = new IntersectionObserver(...);
  cards.forEach(card => observer.observe(card));
});
```

`astro:page-load` 在每次页面内容更新后触发，无论是首次加载还是 View Transitions 导航。把初始化逻辑包进这个事件，就能保证每次导航后组件都能正常工作。

---

## 小结

YukiLog 的前端架构是：`output: 'static'` 作为基础，`prerender: false` 让需要实时数据的页面变成 SSR，`ClientRouter` 提供 SPA 式的导航体验，`astro:page-load` 解决 View Transitions 带来的脚本生命周期问题。

下一篇讲 CSS 变量系统——以及为什么移除 SCSS 之后，构建速度从 18 秒变成了 1.7 秒。
