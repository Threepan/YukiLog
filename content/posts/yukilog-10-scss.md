---
id: 14
title: "YukiLog - 10 - 删掉 SCSS，构建速度快了十倍"
slug: "yukilog-10-scss"
summary: "移除 SCSS 预处理器之后，本地构建从 18 秒降到 1.7 秒。这篇讲的是为什么能删，以及 CSS 变量系统是怎么替代它的"
cover_image: "https://list.yeastar.xin/d/%E6%81%8B/YukiLog/yukilog-10-scss.jpg"
status: "published"
is_featured: false
theme: "博客搭建 (bkdj)"
tags: ["astro", "blog", "教程"]
view_count: 308
created_at: "2026-02-20 21:28:22"
updated_at: "2026-08-17 15:51:21"
source: "https://blog.yeastar.xin/posts/yukilog-10-scss"
---

## SCSS 在这个项目里做了什么

最开始引入 SCSS 有两个理由：变量和嵌套。

变量用来管理颜色和间距，避免在每个组件里重复写 `#7EB6D9`：

```scss
// variables.scss
$lian-blue: #7EB6D9;
$lian-pink: #E8A4B4;
$radius-md: 20px;
$shadow-hover: 0 8px 20px rgba(0, 0, 0, 0.12);
```

嵌套用来写层级选择器，不用重复写父选择器：

```scss
.card {
  background: $lian-white;

  &:hover {
    box-shadow: $shadow-hover;
  }

  .card-title {
    color: $lian-text;
  }
}
```

还有 `color.adjust()`，用来从基础色派生出深色、浅色变体：

```scss
// 从 $lian-pink 派生出深 8% 的颜色
background: color.adjust($lian-pink, $lightness: -8%);
```

这些功能在当时看起来很合理。但随着项目推进，我发现 SCSS 的这三个理由正在逐一消失。

---

## CSS 已经能做这些了

**变量**：CSS 自定义属性（`var(--xxx)`）早就能做 SCSS 变量做的事，而且更强——它是运行时的，可以被 JavaScript 读写，可以在不同作用域覆盖，可以实现主题切换。SCSS 变量是编译时的，构建完就消失了。

**嵌套**：CSS 原生嵌套在主流浏览器里已经支持。Astro 的 scoped CSS 本来就是组件级隔离，嵌套写法和 SCSS 完全一样：

```css
.card {
  background: var(--color-white);

  &:hover {
    box-shadow: var(--shadow-hover);
  }

  .card-title {
    color: var(--color-text);
  }
}
```

**`color.adjust()`**：这个是最后一道防线。SCSS 可以在编译时动态计算颜色，CSS 原生没有等价的运行时颜色函数（`color-mix()` 支持还不够广）。

但我换了一个思路：既然这些派生色的值是固定的，为什么不直接预计算出来，存成 CSS 变量？

```css
/* tokens.css */
--color-pink-d8:  #DE829A;  /* E8A4B4 -8% lightness */
--color-pink-l8:  #F0C4CE;  /* E8A4B4 +8% lightness */
--color-blue-d10: #569FCD;  /* 7EB6D9 -10% lightness */
```

计算一次，写死，永远不变。SCSS 的最后一个理由也没了。

---

## 删掉五个文件

一次提交，删掉了五个 SCSS 文件，共 1206 行：

```
animations.scss   — 动画关键帧
global.scss       — 全局重置和基础样式
theme.scss        — 颜色变量和主题切换
typography.scss   — 字体排版
variables.scss    — SCSS 变量定义
```

替换成五个 CSS 文件，结构完全对应：

```
animations.css
global.css
typography.css
utilities.css
tokens.css        ← 合并了 theme.scss + variables.scss
```

`tokens.css` 是核心，它把原来分散在 `theme.scss`（CSS 变量）和 `variables.scss`（SCSS 变量）的内容合并成一个文件，全部用 CSS 自定义属性表达：

```css
:root {
  --color-blue: #7EB6D9;
  --color-pink: #E8A4B4;
  --radius-md: 20px;
  --shadow-hover: 0 8px 20px rgba(0, 0, 0, 0.12);

  /* 预计算的派生色 */
  --color-pink-d8: #DE829A;
  --color-pink-l8: #F0C4CE;
}
```

---

## 暗色主题：覆盖同名变量

原来的 `theme.scss` 用 SCSS 的方式处理暗色主题，需要在每个组件里写条件逻辑。现在的做法更简单：

```css
/* tokens.css */
:root {
  --color-bg: #F6F7F9;
  --color-text: #2C3E50;
}

[data-theme="dark"] {
  --color-bg: #0F1419;
  --color-text: #E3E8EF;
}
```

`[data-theme="dark"]` 覆盖了 `:root` 里的同名变量。组件代码里只写 `var(--color-bg)`，不需要知道当前是亮色还是暗色，主题切换对组件完全透明。

---

## 构建速度：18 秒 → 1.7 秒

移除 SCSS 之后，本地构建时间从 18 秒降到 1.7 秒，快了约十倍。

原因不复杂：SCSS 需要一个预处理器（`sass`）在构建时把 `.scss` 文件编译成 CSS。这个编译过程本身有开销，而且 Vite 需要处理每一个引用了 SCSS 的文件。项目里有 39 个组件文件都引用了 SCSS，每次构建都要全部过一遍。

换成原生 CSS 之后，Vite 不需要调用任何预处理器，直接处理 CSS，速度自然快得多。

服务器端的构建时间从 20 多秒降到 13 秒，提升幅度小一些，因为服务器的 I/O 和网络开销占了更大比例，CSS 编译只是其中一部分。

---

## 代价

预计算颜色变量有一个明显的代价：改颜色时要手动更新所有派生色。

原来用 SCSS：

```scss
$lian-pink: #E8A4B4;
// 所有派生色自动跟着变
background: color.adjust($lian-pink, $lightness: -8%);
```

现在用 CSS 变量：

```css
--color-pink: #E8A4B4;
--color-pink-d8: #DE829A;  /* 手动计算，手动更新 */
```

如果要换主题色，需要重新计算所有派生色并逐一更新。这是用构建速度换来的维护成本，在颜色方案稳定之后是可以接受的，但在频繁调整设计的阶段会比较麻烦。

---

## 小结

SCSS 在这个项目里做的三件事——变量、嵌套、颜色计算——原生 CSS 都能替代，只是颜色计算需要预计算成静态值。删掉预处理器之后，构建速度快了十倍，代价是颜色派生变成了手动维护。

下一篇讲 Tailwind 的故事，以及文章卡片动画里那个修了很久的圆角问题。
