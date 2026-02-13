<div align="center">

# YukiLog 样式与配置文档

设计色板、SCSS 变量、全局配置、图标系统 —— 所有"拧螺丝"的地方都在这里

</div>

---

## 目录

| 部分 | 说明 |
| --- | --- |
| [配置架构](#config-arch) | 三层配置体系总览 |
| [yukilog.config.ts](#yukilog-config) | 根目录全局配置（唯一配置源） |
| [variables.scss](#variables-scss) | SCSS 设计令牌 |
| [svg-icons.ts](#svg-icons) | SVG 图标管理 |
| [环境变量](#env) | .env 配置 |

---

<a id="config-arch"></a>

## 配置架构

YukiLog 前端采用**三层配置体系**, 职责分明:

```text
┌──────────────────────────────────────────────┐
│  yukilog.config.ts  （根目录）                  │
│  唯一业务配置源                                 │
│  ├─ designTokens    运行时色板                  │
│  ├─ siteConfig      站点/作者/社交/SEO          │
│  ├─ navItems        导航栏配置                  │
│  └─ contentConfig   全站文案（页面/组件/错误）    │
├──────────────────────────────────────────────┤
│  src/styles/variables.scss                    │
│  SCSS 设计令牌（样式层使用）                     │
│  颜色/圆角/阴影/动画/间距/容器/字体/层级          │
├──────────────────────────────────────────────┤
│  src/lib/svg-icons.ts                         │
│  SVG 图标注册表                                │
│  uiIcons / navIcons / socialIcons              │
└──────────────────────────────────────────────┘
```

**为什么色板在两个地方都有？**

* `variables.scss` → 给 SCSS 用（编译时, 在 `.astro` / `.scss` 的 `<style>` 中使用）
* `designTokens` → 给 JS/TS 用（运行时, 在 `<script>` 或动态样式中使用）
* 两者的值保持同步, 但服务于不同场景

**兼容导出层：** `src/lib/config.ts` 仅做 re-export, 不保存业务配置

```typescript
// src/lib/config.ts
export { siteConfig, navItems, designTokens, contentConfig, yukilogConfig } from "../../yukilog.config";
export const API_BASE_URL = import.meta.env.PUBLIC_API_URL || "http://localhost:3000";
```

---

<a id="yukilog-config"></a>

## yukilog.config.ts

源码: [yukilog.config.ts](../yukilog.config.ts)

### designTokens

运行时色板, 与 `variables.scss` 保持同步:

| Token | 值 | 语义 |
| --- | --- | --- |
| `lianBlue` | `#7EB6D9` | 代码与理性 |
| `lianPink` | `#E8A4B4` | 情绪与温柔 |
| `lianWhite` | `#FAFAFA` | 纸张 |
| `lianBg` | `#F6F7F9` | 页面背景 |
| `lianText` | `#2C3E50` | 主文本 |
| `lianTextLight` | `#7F8EA3` | 次要文本 |
| `lianTextMuted` | `#A8B3C1` | 弱文本 |
| `lianBorder` | `#E1E8ED` | 边框 |
| `lianDivider` | `#F0F3F7` | 分隔线 |

### siteConfig

站点基本信息, 类型为 `SiteConfig`:

```typescript
{
  name: "YukiLog",
  title: "恋的博客 - 一个温柔的技术日记本",
  description: "记录技术、思考、情绪与挣扎",
  welcomeText: "欢迎来看恋的博客",
  author: {
    name: "Lian",
    nickname: "恋",
    avatar: "/images/avatar.jpg",
    bio: "我能走到这里，是因为你没有放弃",
    birthday: "2005-05-16",
    genderIdentity: "非二元",
    systemLog: { timestamp: "2024-06-09 08:48:29", message: "..." },
  },
  social: [ /* GitHub, QQ Group, Bilibili, X, 网易云音乐, Gmail */ ],
  startDate: "2026-02-11",
  seo: { keywords: [...], ogImage: "/images/og-image.jpg" },
}
```

### navItems

导航栏配置, 类型为 `NavItem[]`:

| label | href | icon |
| --- | --- | --- |
| 主页 | `/` | `home` |
| 主题 | `/themes` | `theme` |
| 归档 | `/archive` | `archive` |
| 标签 | `/tags` | `tag` |
| 友链 | `/links` | `links` |
| 关于 | `/about` | `about` |

### contentConfig

全站文案配置, 结构如下:

```text
contentConfig
├── hero              首页 Hero 相关（headerGif）
├── markdown          Markdown 渲染配置（标题前缀 emoji）
├── components        各组件文案
│   ├── navbar        品牌名 "YukiLog"
│   ├── welcomeCard   引述文案
│   ├── siteInfoCard  站点信息面板（主站/GitHub/标签文案）
│   ├── applyLinkModal 友链申请弹窗文案
│   ├── comments      评论区所有文案（标题/按钮/表单）
│   ├── tableOfContents 目录标题
│   ├── hitokotoCard  一言标题
│   ├── scrollProgress 回到顶部文案
│   └── search        搜索框/结果/分页文案
└── pages             各页面文案
    ├── error404      404 页面文案
    ├── error500      500 页面文案 + 原因列表
    ├── links         友链页（问候语/最好朋友配置）
    ├── about         关于页（标题/分栏标题）
    ├── themes        主题页（副标题/计数后缀）
    ├── archive       归档页（文案前后缀）
    ├── tags          标签页（空状态/配色循环）
    ├── home          首页（加载更多文案）
    └── admin         管理后台（仪表盘卡片 emoji）
```

---

<a id="variables-scss"></a>

## variables.scss

源码: [src/styles/variables.scss](../src/styles/variables.scss)

通过 Vite 的 `additionalData` 自动注入到所有 SCSS 文件, 无需手动 `@import`:

```javascript
// astro.config.mjs
scss: {
  additionalData: `@use "/src/styles/variables.scss" as *;`,
}
```

### 色板

| 变量 | 值 | 语义 |
| --- | --- | --- |
| `$lian-blue` | `#7EB6D9` | 主题蓝 |
| `$lian-pink` | `#E8A4B4` | 主题粉 |
| `$lian-white` | `#FAFAFA` | 纸张白 |
| `$lian-bg` | `#F6F7F9` | 页面背景 |
| `$lian-text` | `#2C3E50` | 主文本 |
| `$lian-text-light` | `#7F8EA3` | 次要文本 |
| `$lian-text-muted` | `#A8B3C1` | 禁用文本 |
| `$lian-border` | `#E1E8ED` | 边框 |
| `$lian-divider` | `#F0F3F7` | 分隔线 |

**衍生色：** `$lian-blue-light` / `$lian-pink-light` / `$lian-paper-warm` / `$lian-paper-warm-light`

**功能色（低饱和度）：** `$lian-success` / `$lian-warning` / `$lian-error` / `$lian-info`

### 圆角

| 变量 | 值 |
| --- | --- |
| `$radius-sm` | `16px` |
| `$radius-md` | `20px` |
| `$radius-lg` | `24px` |
| `$radius-xl` | `28px` |

### 阴影

| 变量 | 用途 |
| --- | --- |
| `$shadow-sm` | 基础阴影 |
| `$shadow-md` | 卡片阴影 |
| `$shadow-lg` | 弹窗阴影 |
| `$shadow-hover` | 悬浮态阴影 |
| `$shadow-pink` / `$shadow-blue` | 彩色阴影（左下偏移, 描边质感） |

### 动画

| 变量 | 值 | 说明 |
| --- | --- | --- |
| `$transition-fast` | `200ms` | 快速过渡 |
| `$transition-base` | `300ms` | 标准过渡 |
| `$transition-slow` | `400ms` | 慢速过渡 |
| `$ease-gentle` | `cubic-bezier(0.4, 0, 0.2, 1)` | 温柔缓入缓出 |
| `$ease-out` | `cubic-bezier(0, 0, 0.2, 1)` | 缓出 |

### 间距

| 变量 | 值 |
| --- | --- |
| `$spacing-xs` | `0.5rem` (8px) |
| `$spacing-sm` | `1rem` (16px) |
| `$spacing-md` | `1.5rem` (24px) |
| `$spacing-lg` | `2rem` (32px) |
| `$spacing-xl` | `3rem` (48px) |
| `$spacing-xxl` | `4rem` (64px) |

### 容器

| 变量 | 值 |
| --- | --- |
| `$container-sm` | `640px` |
| `$container-md` | `768px` |
| `$container-lg` | `1024px` |
| `$container-xl` | `1200px` |

### 字体

```scss
$font-family-base: 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', ...;
$font-family-code: 'JetBrains Mono', 'Fira Code', 'Consolas', ...;
```

字号: `$font-size-xs` (12px) → `$font-size-4xl` (40px)

行高: `$line-height-tight` (1.25) / `$line-height-base` (1.6) / `$line-height-relaxed` (1.8)

### Z-index 层级

| 变量 | 值 | 用途 |
| --- | --- | --- |
| `$z-nav` | `100` | 导航栏 |
| `$z-dropdown` | `150` | 下拉菜单 |
| `$z-modal` | `200` | 弹窗 |
| `$z-tooltip` | `300` | 提示框 |

---

<a id="svg-icons"></a>

## svg-icons.ts

源码: [src/lib/svg-icons.ts](../src/lib/svg-icons.ts)

通过 Vite 的 `?raw` 导入器将 SVG 文件直接加载为字符串, 分三组注册:

### uiIcons（界面图标）

| key | 说明 |
| --- | --- |
| `arrowUp` | 向上箭头 |
| `arrow` | 通用箭头 |
| `envelope` | 信封 |
| `close` | 关闭 |
| `globe` | 地球 |
| `eye` | 眼睛（浏览量） |
| `clock` | 时钟 |
| `chevronRight` | 右箭头 |
| `githubMark` | GitHub 标记 |
| `arrowRightLine` | 右箭头线条 |
| `folderOpen` | 文件夹 |
| `refreshCcw` | 刷新 |
| `light` | 灯泡 |
| `search` | 搜索 |
| `openingQuotationMark` | 左引号 |
| `closingQuotationMark` | 右引号 |
| `cake` | 蛋糕（生日） |

### navIcons（导航图标）

| key | 文件路径 |
| --- | --- |
| `home` | `assets/icon/nav/home.svg` |
| `theme` | `assets/icon/nav/theme.svg` |
| `archive` | `assets/icon/nav/archive.svg` |
| `tag` | `assets/icon/nav/tag.svg` |
| `links` | `assets/icon/nav/links.svg` |
| `about` | `assets/icon/nav/about.svg` |

### socialIcons（社交图标）

| key | 对应平台 |
| --- | --- |
| `github` | GitHub |
| `qq` | QQ |
| `bilibili` | Bilibili |
| `twitter` | X (Twitter) |
| `netease-music` | 网易云音乐 |
| `gmail` | Gmail |

**类型导出：** `UiIconKey` / `NavIconKey` / `SocialIconKey`

**使用方式：**

```astro
---
import { uiIcons, navIcons, socialIcons, svgIcons } from '../lib/svg-icons';
---
<!-- 直接注入 SVG -->
<span set:html={uiIcons.search}></span>

<!-- 或使用合并对象 -->
<span set:html={svgIcons.home}></span>
```

---

<a id="env"></a>

## 环境变量

配置文件: [.env.example](../.env.example)

| 变量 | 说明 | 默认值 |
| --- | --- | --- |
| `PUBLIC_API_URL` | 后端 API 地址 | `http://localhost:3000` |

**使用方式：**

```typescript
// Astro 前台脚本或服务端代码中
const apiUrl = import.meta.env.PUBLIC_API_URL;
```

> Astro 约定: 以 `PUBLIC_` 开头的环境变量会暴露给客户端代码

---

## 其他样式文件

| 文件 | 说明 |
| --- | --- |
| [src/styles/global.scss](../src/styles/global.scss) | 全局基础样式（CSS Reset + 基础排版） |
| [src/styles/animations.scss](../src/styles/animations.scss) | 全局动画定义 |
| [src/styles/typography.scss](../src/styles/typography.scss) | Markdown 文章排版样式 |
| [src/styles/utilities.scss](../src/styles/utilities.scss) | 工具类样式 |

这些文件在 `BaseLayout` 中通过 `import '../styles/global.scss'` 引入
