---
id: 12
title: "YukiLog - 8 - 一个文件管理所有配置"
slug: "yukilog-8-fileconfig"
summary: "把散落在组件里的硬编码字符串赶到一个地方——这篇讲的是 yukilog.config.ts 的设计思路，以及它现在管的有点太宽的问题"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-8-fileconfig.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["astro", "blog", "教程"]
view_count: 257
created_at: "2026-02-20 19:30:35"
updated_at: "2026-08-17 03:49:24"
source: "https://blog.yeastar.xin/posts/yukilog-8-fileconfig"
---

## 改一个按钮文案要找半天

前端写到一定程度，我发现了一个让人烦躁的问题：想改一个按钮的文案，要先想"这个按钮在哪个组件里"，找到组件，找到那一行，改完还不确定有没有漏掉其他地方用了同样文案的地方。

这不是代码写错了，是信息散落的问题。

评论区的提交按钮写着"✉️ 提交"，回复弹窗里写着"💬 回复"，404 页面的标题写着"页面走失了"——这些字符串分散在十几个组件文件里，没有任何一个地方能让你一眼看到"这个博客所有对外展示的文字"。

解决方案是把它们全部赶到一个地方：`yukilog.config.ts`。

---

## 放在根目录的理由

这个文件放在项目根目录，而不是 `src/` 里面。

原因很简单：前台页面、后台管理面板、构建脚本，都可能需要读取配置。如果放在 `src/` 里，后台或脚本要引用时路径会很奇怪。放在根目录，所有人都能用统一的相对路径找到它。

组件实际上不直接引用根目录的文件——`src/lib/config.ts` 作为一个兼容导出层存在：

```typescript
// src/lib/config.ts
export {
  siteConfig,
  navItems,
  designTokens,
  contentConfig,
  yukilogConfig,
} from "../../yukilog.config";
```

这一层的作用是稳定引用路径。组件统一从 `../../lib/config` 导入，不需要关心真实配置文件在哪里。如果以后配置文件移动了位置，只需要改这一个导出层，不需要改几十个组件。

---

## 文件里有什么

`yukilog.config.ts` 目前分四个部分：

**`designTokens`** — 运行时色板。颜色值给 JS/TS 场景用，比如组件脚本里动态着色、后台配置面板读取颜色。

**`siteConfig`** — 站点基本信息。名称、作者信息、社交链接、SEO 关键词，前台 SEO 组件和个人信息面板都从这里读。

**`navItems`** — 导航栏配置。NavBar 组件直接遍历这个数组渲染导航项。

**`contentConfig`** — 文案配置。这是最大的一块，覆盖了几乎所有组件和页面的展示文字。

最后有一个汇总导出：

```typescript
export const yukilogConfig = {
  designTokens,
  siteConfig,
  navItems,
  contentConfig,
} as const;
```

后台配置面板可以一次性读取整个 `yukilogConfig`，不需要分别导入四个变量。

---

## `satisfies` 的用法

`siteConfig` 和 `navItems` 用了一个值得注意的写法：

```typescript
export const siteConfig = {
  name: "YukiLog",
  // ...
} satisfies SiteConfig;

export const navItems = [
  { label: "主页", href: "/", icon: "home" },
  // ...
] satisfies NavItem[];
```

`satisfies` 是 TypeScript 4.9 引入的关键字，它和 `as SiteConfig` 的区别在于：

* `as SiteConfig` 会把类型"收窄"成 `SiteConfig`，字面量信息丢失
* `satisfies SiteConfig` 在做类型校验的同时，保留字面量类型推断

实际效果是：TypeScript 会检查你的对象是否符合 `SiteConfig` 的结构（写错字段名会报错），但推断出来的类型仍然是具体的字面量，而不是宽泛的 `string`。

`as const` 则是另一层：让所有值变成只读的字面量类型，防止被意外修改。两者组合，既有类型安全，又有精确的类型推断。

---

## CSS 变量和运行时色板的边界

颜色在两个地方都出现了：`designTokens` 里有一份，`src/styles/variables.scss` 里也有一份，而且值是相同的。

这看起来像是重复，但它们服务的是不同的消费场景：

* **CSS 变量**（`variables.scss`）：给样式文件用，浏览器直接解析，组件的 `` 块里用 `var(--color-blue)` 引用
* **运行时色板**（`designTokens`）：给 JS/TS 用，比如用 Canvas 绘图、动态生成内联样式、后台配置面板展示颜色预览

如果只保留 CSS 变量，JS 里要用颜色就得写 `getComputedStyle(el).getPropertyValue('--color-blue')`，既麻烦又依赖 DOM。如果只保留 `designTokens`，样式文件就得用 JS 注入，失去了 CSS 的静态优势。

两份是有意为之的，代价是改颜色时要改两个地方。

---

## 这个方案的问题

`yukilog.config.ts` 解决了"散落"的问题，但引入了另一个问题：**堆积**。

现在这个文件管的太宽了，几个明显的地方：

**页面文案不该全堆在一起。** `contentConfig.pages` 下面有 `error404`、`error500`、`links`、`about`、`themes`、`archive`、`tags`、`home`、`admin`——九个页面的文案全在一个对象里。更合理的做法是按页面拆分，或者让每个页面自己管自己的文案。

**友链数据不是配置。** `bestFriend` 的信息（名称、头像、URL、认识时间）写在配置文件里，但它本质上是数据，应该进数据库或者单独的数据文件，而不是和"提交按钮叫什么"放在一起。

**`colorCycle` 是硬编码的。** 标签页的颜色循环序列 `[0, 2, 1, 0, 1, 2, ...]` 是手写的，目的是让相邻标签颜色不重复。这个逻辑应该自动计算，而不是靠人工维护一个数组。

当前的状态是"第一阶段的权衡"——先把散落的东西收拢，再慢慢分层。知道边界在哪里，比假装没有问题更重要。

---

## 小结

`yukilog.config.ts` 做了一件事：给所有配置一个固定的家。

下一篇讲 Astro 的岛屿架构——为什么大部分页面是静态的，而评论区和管理后台不是。
