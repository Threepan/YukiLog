# YukiLog 前端项目结构说明

## 📁 目录结构

```
yukilog-hanakoi/
├── src/
│   ├── styles/                  # 样式文件
│   │   ├── variables.scss       # 恋的配色方案 + 变量定义
│   │   ├── animations.scss      # 动画定义（渐入/滑入/悬停）
│   │   ├── typography.scss      # 字体排版样式
│   │   ├── utilities.scss       # 工具类（间距/圆角/阴影）
│   │   └── global.scss          # 全局样式（整合上述所有）
│   │
│   ├── types/                   # TypeScript 类型定义
│   │   ├── api.ts              # 后端 API 响应类型
│   │   ├── blog.ts             # 前端专属类型
│   │   └── index.ts            # 类型统一导出
│   │
│   ├── lib/                     # 工具库
│   │   ├── api.ts              # API 调用封装（对接后端）
│   │   ├── auth.ts             # JWT 认证管理
│   │   ├── utils.ts            # 工具函数集
│   │   └── config.ts           # 站点配置
│   │
│   ├── layouts/                 # 布局组件
│   │   ├── BaseLayout.astro    # 基础布局（SEO + 全局样式）
│   │   └── BlogLayout.astro    # 博客布局（导航栏 + 页脚）
│   │
│   ├── components/              # UI 组件
│   │   ├── navigation/         # 导航相关
│   │   │   ├── NavBar.astro    # 导航栏（支持滑入动画）
│   │   │   └── NavItem.astro   # 导航项
│   │   ├── shared/             # 共享组件
│   │   │   ├── SEO.astro       # SEO Meta 标签
│   │   │   └── Footer.astro    # 页脚
│   │   ├── home/               # 主页组件（待实现）
│   │   └── blog/               # 博客组件（待实现）
│   │
│   └── pages/                   # 路由页面
│       └── index.astro         # 首页（临时测试页）
│
├── public/                      # 静态资源
│   └── favicon.svg
│
├── .env                         # 环境变量
├── .env.example                # 环境变量模板
├── astro.config.mjs            # Astro 配置
├── package.json
└── tsconfig.json
```

---

## 🎨 样式系统

### 核心配色（恋的方案）

```scss
$lian-blue: #7EB6D9;    // 代码与理性
$lian-pink: #E8A4B4;    // 情绪与温柔
$lian-white: #FAFAFA;   // 纸张
$lian-bg: #F6F7F9;      // 页面背景
```

### 设计原则

- 柔软圆角（16px ~ 24px）
- 轻微阴影（低对比度）
- 温柔动画（200ms ~ 300ms，ease-gentle）
- 无毛玻璃效果、无炫光特效

### 使用方式

**全局样式**：已在 `BaseLayout.astro` 中自动引入

**组件样式**：
```astro
<style lang="scss">
// 变量已自动注入，直接使用
.my-component {
  background: $lian-blue;
  border-radius: $radius-md;
  box-shadow: $shadow-sm;
}
</style>
```

---

## 📡 API 调用

### 使用示例

```typescript
import { postsApi, themesApi, tagsApi } from '../lib/api';

// 获取文章列表（支持分页、筛选）
const posts = await postsApi.list({
  page: 1,
  page_size: 10,
  sort: 'created_at',
  theme_slugs: 'tech,life',    // 逗号分隔
  tag_slugs: 'rust,backend',   // AND 关系
});

// 获取主题列表（按文章数排序）
const themes = await themesApi.list('post_count');

// 获取标签列表
const tags = await tagsApi.list('view_count');

// 获取文章详情
const post = await postsApi.getBySlug('my-article');

// 增加浏览计数（无需等待返回）
postsApi.incrementView('my-article');
```

### API 基础地址

通过环境变量配置：`.env` 中的 `PUBLIC_API_URL`

---

## 🛠️ 工具函数

### 日期格式化

```typescript
import { formatDate, getSiteUptime } from '../lib/utils';

formatDate('2026-01-01T00:00:00+08:00', 'short');     // "2026/01/01"
formatDate('2026-01-01T00:00:00+08:00', 'full');      // "2026年1月1日 00:00"
formatDate('2026-01-01T00:00:00+08:00', 'relative');  // "2 天前"

getSiteUptime('2026-02-12');  // "15 天"
```

### 其他工具

```typescript
import {
  truncate,
  buildUrl,
  debounce,
  throttle,
  getLocalStorage,
  setLocalStorage,
  scrollToElement,
  copyToClipboard,
} from '../lib/utils';
```

---

## 🧩 组件使用

### 布局组件

```astro
---
import BlogLayout from '../layouts/BlogLayout.astro';
---

<BlogLayout title="页面标题" description="页面描述">
  <!-- 你的页面内容 -->
</BlogLayout>
```

### 导航栏

已在 `BlogLayout` 中自动引入，无需手动添加

行为：
- 默认隐藏（opacity: 0）
- 鼠标靠近顶部时渐入
- 滚动超过 1080px 后变为 sticky（常驻显示，带阴影）

---

## ⚙️ 配置

### 站点配置（`src/lib/config.ts`）

修改以下配置自定义站点信息：

```typescript
export const siteConfig = {
  name: 'YukiLog',
  title: '恋的博客 - 一个温柔的技术日记本',
  author: {
    name: 'Lian',
    avatar: '/images/avatar.jpg',
    // ...
  },
  social: [
    { name: 'GitHub', url: '...', icon: 'github' },
    // ...
  ],
  startDate: '2026-02-12',
};
```

### 导航项配置

```typescript
export const navItems = [
  { label: 'YukiLog', href: '/', icon: 'home' },
  { label: '主题', href: '/themes', icon: 'folder' },
  // ...
];
```

---

## 🚀 运行项目

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm dev

# 访问 http://localhost:4321
```

---

## ✅ 已完成的基础设施

- ✅ SCSS 样式系统（恋的配色方案）
- ✅ TypeScript 类型定义（完整对接后端 API）
- ✅ API 调用封装（前台所有接口）
- ✅ 工具函数库（日期、本地存储、DOM 操作等）
- ✅ 基础布局组件（SEO + 导航栏 + 页脚）
- ✅ 站点配置系统
- ✅ JWT 认证管理（管理端备用）

---

## 📝 下一步

开始实现具体页面：

1. **主页**：欢迎屏 + 个人信息 + 文章列表
2. **文章详情页**：Markdown 渲染 + 评论系统
3. **主题/标签/归档页**
4. **友链页**
5. **关于页**

所有基础设施已就绪，可以直接开始构建页面组件！
