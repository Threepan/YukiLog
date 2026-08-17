---
id: 15
title: "YukiLog - 11 - Tailwind 的故事，以及那个圆角问题"
slug: "yukilog-11-tailwind"
summary: "移除 Tailwind 只花了两个提交，但文章卡片的圆角问题修了很久——这篇讲的是为什么删，以及 GPU 合成层和 overflow:hidden 之间的冲突"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-11-tailwind.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["astro", "blog", "教程"]
view_count: 341
created_at: "2026-02-20 21:38:04"
updated_at: "2026-08-16 15:30:10"
source: "https://blog.yeastar.xin/posts/yukilog-11-tailwind"
---

## Tailwind 是怎么进来的

Tailwind 进入这个项目的理由很简单：我在学它。

YukiLog 开发初期，我想顺便把 Tailwind 用熟。它的工具类系统确实有吸引力——不用起类名，不用在 CSS 文件和 HTML 之间来回跳，直接在标签上写样式。

但用了一段时间之后，我发现它和我的写法不合。

我习惯给每个组件写独立的 `` 块，用语义化的类名描述结构，用 CSS 变量管理设计 token。Tailwind 的工具类和这套写法是冲突的——你不能一边用 `class="flex items-center gap-4"` 一边用 `var(--color-blue)`，两套系统混在一起，代码反而更难读。

最终的结果是：Tailwind 被引入了，但几乎没有被用。`astro.config.mjs` 里有它的插件配置，`package.json` 里有它的依赖，但组件里找不到几个 Tailwind 类名。

---

## 移除只花了两个提交

删掉一个没有被用的依赖，代价很小：

```
fix: 移除未使用的 tailwindcss 插件配置
  astro.config.mjs  — 删掉 tailwindcss() 插件

fix: 移除未使用的 tailwindcss 依赖项
  package.json      — 删掉 tailwindcss 依赖
  pnpm-lock.yaml    — 净减少 192 行
```

`pnpm-lock.yaml` 减少了 192 行，这是 Tailwind 及其依赖树的锁文件记录。服务器构建时间从 20 多秒降到 13 秒，这部分提升和 SCSS 移除的效果叠加在一起——两者都减少了构建时需要处理的依赖和转换步骤。

---

## GPU 合成和圆角的冲突

文章卡片（`PostListCard`）有一个入场动画：卡片从 `translateY(16px)` 滑入，同时 `opacity` 从 0 到 1。封面图片在 hover 时有 `scale` 放大效果。

这两个动画都是 CSS `transition`，理论上很简单。但实际上，封面图片的圆角在动画过程中会消失。

原因在于 GPU 合成层的工作方式。

浏览器在渲染时，会把某些元素提升到独立的 GPU 合成层（compositor layer）。提升到合成层的元素，它的动画由 GPU 直接处理，不需要触发主线程的重绘——这是性能优化的核心。

但合成层有一个副作用：**`overflow: hidden` 对合成层的子元素失效**。

卡片的结构是这样的：

```
.plc-card（border-radius: 18px; overflow: hidden）
  └── img（scale 动画）
```

正常情况下，`overflow: hidden` 会裁剪超出圆角的部分，图片的圆角由父元素控制。但当 `img` 被提升到合成层之后，它脱离了父元素的裁剪上下文，`overflow: hidden` 不再对它生效，圆角消失了。

---

## 第一次尝试：`clip-path` 加在卡片上

第一个思路是给卡片加 `clip-path`：

```css
.plc-card {
  border-radius: 18px;
  overflow: hidden;
  clip-path: inset(0 round 18px); /* 强制 GPU 合成时裁剪圆角 */
}
```

`clip-path` 是在合成阶段之后应用的，理论上能裁剪合成层的内容。但这个方案有问题：`clip-path` 加在卡片上，会裁剪卡片本身的所有内容，包括卡片的 `box-shadow`——阴影被裁掉了。

---

## 真正的解法：给 img 套一个 div

问题的根源是 `img` 直接作为合成层，脱离了父元素的裁剪上下文。解法是在 `img` 外面加一层包装容器，让裁剪发生在这一层：

```css
/* 图片包装层 */
.plc-cover-wrapper {
  width: 100%;
  overflow: hidden;
  border-radius: inherit;          /* 继承卡片圆角 */
  clip-path: inset(0 round 18px);  /* GPU 合成时强制裁剪 */
}

/* 图片本身不需要 border-radius */
.plc-cover-wrapper img {
  width: 100%;
  height: auto;
  object-fit: cover;
  transition: transform 500ms cubic-bezier(0.4, 0, 0.2, 1);
}
```

`.plc-cover-wrapper` 不参与动画，不会被提升到合成层，它的 `clip-path` 能正常裁剪子元素。`img` 在这个容器内做 `scale`，圆角由容器控制，不再依赖 `overflow: hidden` 对合成层的裁剪。

这个问题我和 Agent 联合调试了很久，试了各种 CSS 属性组合。最后我一句话解决了：「给 img 套一个 div 容器」。

---

## `will-change: transform` 的作用

在加入包装层的同时，卡片本身也加了 `will-change: transform`：

```css
.plc-card {
  will-change: transform;
}
```

`will-change` 告诉浏览器这个元素即将发生 `transform` 变化，提前把它提升到合成层。这样做有两个效果：

一是让入场动画（`translateY` + `opacity`）由 GPU 处理，避免主线程重绘，动画更流畅。

二是让合成层的边界在动画开始前就确定下来，而不是在动画触发时临时提升——临时提升会导致一帧的闪烁。

---

## 小结

Tailwind 进来是因为想学，删掉是因为和写法不合，移除成本几乎为零。GPU 合成层和 `overflow: hidden` 的冲突是一个经典的渲染陷阱，解法不是找到正确的 CSS 属性组合，而是调整 DOM 结构——加一层容器，让裁剪和动画分离。
