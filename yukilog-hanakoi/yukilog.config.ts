// ================================
// YukiLog 全局配置（唯一配置源）
// ================================
// 说明：
// 1) 这个文件放在项目根目录，便于前台/后台/脚本统一读取。
// 2) src/lib/config.ts 仅作为兼容导出层，不再保存业务配置。
// 3) 与样式强相关的 SCSS 变量仍保留在 src/styles/variables.scss，
//    这里保留一份同值色板用于运行时组件（例如 JS 动态着色、管理后台配置编辑）。

import type { NavItem, SiteConfig } from "./src/types";

/**
 * 设计色板（运行时）
 * - 用于组件脚本、后台配置面板等 JS/TS 场景
 * - SCSS 样式仍以 variables.scss 为准
 */
export const designTokens = {
  colors: {
    lianBlue: "#7EB6D9",     // 代码与理性
    lianPink: "#E8A4B4",     // 情绪与温柔
    lianWhite: "#FAFAFA",    // 纸张
    lianBg: "#F6F7F9",       // 页面背景
    lianText: "#2C3E50",     // 主文本
    lianTextLight: "#7F8EA3",// 次要文本
    lianTextMuted: "#A8B3C1",// 弱文本
    lianBorder: "#E1E8ED",   // 边框
    lianDivider: "#F0F3F7",  // 分隔线
  },
} as const;

/**
 * 站点基本配置
 * - 前台 SEO、页脚、个人信息面板、欢迎文案等统一来源
 */
export const siteConfig = {
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
    systemLog: {
      timestamp: "2024-06-09 08:48:29",
      message: "这不是你亲手开启的故事吗？",
    },
  },
  social: [
    {
      name: "GitHub",
      url: "https://github.com/Yueosa",
      icon: "github",
      color: "#6E7F8D",
    },
    {
      name: "QQ Group",
      url: "https://qm.qq.com/cgi-bin/qm/qr?k=O6KD1bt5WDvQw47kzjaDuYIASzar_y-F&jump_from=webapi&authKey=AnF+0ddOwtFY4laf9lDJ9Om7tj5oZE2dfuHJlQfOO2CXaeTOOVdJxlxIg9wSs4WQ",
      icon: "qq",
      color: "#E3A0AE",
    },
    {
      name: "Bilibili",
      url: "https://space.bilibili.com/433677987",
      icon: "bilibili",
      color: "#7EB6D9",
    },
    {
      name: "X (Twitter)",
      url: "https://x.com/Yosa04942475621",
      icon: "twitter",
      color: "#8FAFC4",
    },
    {
      name: "网易云音乐",
      url: "https://music.163.com/#/user/home?id=630887153",
      icon: "netease-music",
      color: "#E8A4B4",
    },
    {
      name: "Gmail",
      url: "mailto:yichengxin7@gmail.com",
      icon: "gmail",
      color: "#D6A1AE",
    },
  ],
  startDate: "2026-02-11",
  seo: {
    keywords: ["博客", "技术", "Rust", "TypeScript", "前端", "后端", "全栈"],
    ogImage: "/images/og-image.jpg",
  },
} satisfies SiteConfig;

/**
 * 顶部导航配置
 * - NavBar 组件统一读取
 * - icon 字段对应 src/assets/icon/nav 下的文件名（不含扩展名）
 */
export const navItems = [
  { label: "主页", href: "/", icon: "home" },
  { label: "主题", href: "/themes", icon: "theme" },
  { label: "归档", href: "/archive", icon: "archive" },
  { label: "标签", href: "/tags", icon: "tag" },
  { label: "友链", href: "/links", icon: "links" },
  { label: "关于", href: "/about", icon: "about" },
] satisfies NavItem[];

/**
 * 文案与资源配置（首批）
 * - 逐步将页面硬编码文案迁移到这里
 */
export const contentConfig = {
  hero: {
    headerGif: "mc.gif",
  },
  markdown: {
    headingPrefixes: {
      h1: "✨ ",
      h2: "✦ ",
      h3: "▸ ",
      h4: "• ",
      h5: "· ",
      h6: "— ",
    },
  },
  components: {
    navbar: {
      brand: "YukiLog",
    },
    welcomeCard: {
      quoteText: "这里分享她所热爱的技术、思考，以及情绪、挣扎",
    },
    siteInfoCard: {
      title: "站点信息",
      mainSite: {
        name: "主站",
        value: "yeastar.xin",
        url: "https://yeastar.xin",
      },
      github: {
        name: "GitHub",
        value: "Yueosa/YukiLog",
        url: "https://github.com/Yueosa/YukiLog",
      },
      labels: {
        totalViews: "总浏览量",
        uptime: "已运行",
      },
    },
    applyLinkModal: {
      hint: "也想出现在这里吗？",
      triggerButton: "申请友链",
      title: "写一封信给恋",
      subtitle: "留下你的站点信息，我会认真查看每一份申请",
      submitText: "寄出",
      submitLoadingText: "寄送中...",
    },
  },
  pages: {
    links: {
      greeting: "能走到这里的人，大概都是温柔的吧。",
      friendsTitle: "朋友们",
      bestFriend: {
        name: "Duo 云站",
        description: "MathForest官方🌲|程序及数学可视化✨|屑魔女游世界🔮",
        avatar: "https://www.mduo.cloud/elaina_q.jpg",
        url: "https://www.mduo.cloud/",
        message: "愿你的梦中常有我相伴",
      },
      friends: [
        { name: "一只小透明", description: "记录平凡日子里的不平凡", avatar: null, url: "https://example.com/1" },
        { name: "桜の記録", description: "全栈开发者的技术笔记", avatar: null, url: "https://example.com/2" },
        { name: "云深不知处", description: "诗和远方的数字花园", avatar: null, url: "https://example.com/3" },
        { name: "CodeDreamer", description: "把代码写成诗", avatar: null, url: "https://example.com/4" },
        { name: "月光邮局", description: "收集世界各地的温柔", avatar: null, url: "https://example.com/5" },
        { name: "像素森林", description: "独立游戏与像素艺术", avatar: null, url: "https://example.com/6" },
      ],
    },
    about: {
      opening: "你好，我是",
    },
    themes: {
      heroSubtitle: "按类别浏览文章",
    },
    archive: {
      heroSubtitlePrefix: "共",
      heroSubtitleSuffix: "篇文章，记录着走过的路",
      timelineEndText: "故事从这里开始",
    },
    tags: {
      emptyText: "这个标签下暂时还没有文章",
      colorCycle: [0, 2, 1, 0, 1, 2, 1, 0, 2, 0, 1, 2, 0, 2, 1, 0, 1, 2],
      colorNames: ["pink", "blue", "white"],
    },
    home: {
      articleList: {
        loadMoreText: "—— 暂时只有这些了 ——",
      },
    },
  },
} as const;

// ================================
// 统一文章数据源
// ================================
// 所有页面（首页、归档、标签、主题）的文章数据都从这里派生，
// 保证 slug、theme、tags 完全一致，链接不会 404。

export const allPosts = [
  {
    title: "Markdown 渲染占位测试",
    slug: "markdown-style-placeholder",
    summary: "用于验证标题前缀、动态下划线、表格样式与代码块样式的测试文章。",
    cover_image: "https://list.yeastar.xin/d/%E6%81%8B/fastfetch/1761672341793.jpeg",
    created_at: "2026-02-12T20:00:00+08:00",
    view_count: 1,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "Markdown", slug: "markdown" },
      { name: "CSS", slug: "css" },
    ],
  },
  {
    title: "Astro + Vue 3：打造轻盈优雅的博客前端",
    slug: "astro-vue3-blog-frontend",
    summary: "选择 Astro 作为 SSG 框架，搭配 Vue 3 实现交互组件。从双屏首页设计到响应式布局，分享这套轻量而富有表现力的前端方案。",
    cover_image: "https://list.yeastar.xin/d/%E6%81%8B/fastfetch/20250809_033215.jpg",
    created_at: "2026-02-12T14:30:00+08:00",
    view_count: 96,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "Astro", slug: "astro" },
      { name: "Vue", slug: "vue" },
      { name: "TypeScript", slug: "typescript" },
    ],
  },
  {
    title: "用 Rust 和 Axum 从零搭建博客后端",
    slug: "build-blog-backend-with-rust-axum",
    summary: "从项目初始化到 RESTful API 设计，完整记录用 Rust 实现博客后端的全过程。数据库选型、ORM 搭建、错误处理与中间件设计等核心环节一一解析。",
    cover_image: "https://list.yeastar.xin/d/%E6%81%8B/fastfetch/1f57b6c355024848e0ccfa0f864273328c22fa76130ae3183bd15453f06d3a17.jpg",
    created_at: "2026-02-11T10:00:00+08:00",
    view_count: 128,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "Rust", slug: "rust" },
      { name: "Axum", slug: "axum" },
    ],
  },
  {
    title: "YukiLog 前端架构复盘",
    slug: "yukilog-frontend-review",
    summary: "回顾一个月的前端开发经历，从技术选型到组件设计的全程反思。",
    cover_image: "https://list.yeastar.xin/d/%E6%81%8B/fastfetch/56934dfd210ab238a1ce39ad8adbd7dedc9c744ef910455de9d774f1a2a373b6.jpg",
    created_at: "2026-02-10T22:15:00+08:00",
    view_count: 64,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "Astro", slug: "astro" },
      { name: "架构设计", slug: "architecture" },
    ],
  },
  {
    title: "写给自己的一封信",
    slug: "letter-to-myself",
    summary: "在某个安静的夜晚写下的独白。关于选择、关于坚持、关于那些不太被看见的角落。",
    cover_image: "https://list.yeastar.xin/d/%E6%81%8B/fastfetch/bf4c8fd473f4b2e3cb7163d14a8182ed2ea203261f29e9fbc5c1f93e5fcb60be.jpg",
    created_at: "2026-02-10T02:00:00+08:00",
    view_count: 256,
    theme: { name: "随笔", slug: "essay" },
    tags: [
      { name: "情绪", slug: "emotion" },
      { name: "思考", slug: "thought" },
    ],
  },
  {
    title: "SeaORM 实战：优雅地管理数据库实体与关系",
    slug: "seaorm-entity-relations",
    summary: "深入 SeaORM 的实体定义、关系映射与查询构建。比较 Active Record 与 Query Builder 两种模式，探索 Rust 生态中数据库操作的最佳实践。",
    cover_image: "https://list.yeastar.xin/d/%E6%81%8B/fastfetch/8755c1590031b234f650056457e8b851ab52d4f523667f25303b9c9f1b6f1242.jpg",
    created_at: "2026-02-09T16:00:00+08:00",
    view_count: 87,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "Rust", slug: "rust" },
      { name: "Sea-ORM", slug: "sea-orm" },
      { name: "PostgreSQL", slug: "postgresql" },
    ],
  },
  {
    title: "深夜的代码与猫",
    slug: "late-night-code-and-cat",
    summary: "凌晨两点，屏幕发出微弱的光。窗外是安静的城市，身边是蜷缩的猫。这大概就是我最喜欢的编程时间。",
    cover_image: "https://list.yeastar.xin/d/%E6%81%8B/fastfetch/Image_1762911570825.jpg",
    created_at: "2026-02-08T02:30:00+08:00",
    view_count: 194,
    theme: { name: "随笔", slug: "essay" },
    tags: [
      { name: "生活", slug: "life" },
      { name: "思考", slug: "thought" },
    ],
  },
  {
    title: "SCSS 变量系统设计：构建一致性的视觉语言",
    slug: "scss-variable-system-design",
    summary: "如何利用 SCSS 变量、Mixins 与设计 Token 打造一套温柔而统一的设计系统。",
    cover_image: null,
    created_at: "2026-02-07T11:00:00+08:00",
    view_count: 73,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "SCSS", slug: "scss" },
      { name: "CSS", slug: "css" },
    ],
  },
  {
    title: "Rust Axum 中间件实践",
    slug: "rust-axum-middleware",
    summary: "探索 Axum 的中间件机制和错误处理模式，从鉴权到日志全链路实战。",
    cover_image: null,
    created_at: "2026-02-03T09:00:00+08:00",
    view_count: 55,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "Rust", slug: "rust" },
      { name: "Axum", slug: "axum" },
    ],
  },
  {
    title: "如何从零搭建一个博客系统",
    slug: "build-blog-from-scratch",
    summary: "从需求分析到技术选型，记录 YukiLog 博客系统的诞生过程。",
    cover_image: null,
    created_at: "2026-01-08T10:00:00+08:00",
    view_count: 42,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "全栈", slug: "fullstack" },
      { name: "架构设计", slug: "architecture" },
    ],
  },
  {
    title: "年终总结：代码与生活",
    slug: "year-end-2025",
    summary: "2025 年发生了很多事，关于代码、关于成长、关于那些不被看见的努力。",
    cover_image: null,
    created_at: "2025-12-28T20:00:00+08:00",
    view_count: 312,
    theme: { name: "随笔", slug: "essay" },
    tags: [
      { name: "年终总结", slug: "year-review" },
      { name: "生活", slug: "life" },
      { name: "思考", slug: "thought" },
    ],
  },
  {
    title: "深夜的胡思乱想",
    slug: "midnight-thoughts",
    summary: "凌晨三点睡不着写的东西，关于未来、关于迷茫。",
    cover_image: null,
    created_at: "2025-12-15T03:00:00+08:00",
    view_count: 189,
    theme: { name: "随笔", slug: "essay" },
    tags: [
      { name: "情绪", slug: "emotion" },
      { name: "生活", slug: "life" },
    ],
  },
  {
    title: "Sea-ORM 踩坑日记",
    slug: "sea-orm-notes",
    summary: "在 Rust 生态里找一个顺手的 ORM，踩过的坑和最终选择。",
    cover_image: null,
    created_at: "2025-11-20T14:00:00+08:00",
    view_count: 76,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "Rust", slug: "rust" },
      { name: "Sea-ORM", slug: "sea-orm" },
    ],
  },
  {
    title: "关于「造轮子」这件事",
    slug: "about-reinventing-wheel",
    summary: "为什么明知有现成方案还要自己写？也许造轮子本身就是学习。",
    cover_image: null,
    created_at: "2025-10-18T16:00:00+08:00",
    view_count: 145,
    theme: { name: "随笔", slug: "essay" },
    tags: [
      { name: "思考", slug: "thought" },
      { name: "开源", slug: "open-source" },
    ],
  },
  {
    title: "Astro 初体验",
    slug: "astro-first-look",
    summary: "用 Astro 搭一个静态博客，到底有多舒服？从安装到第一篇文章上线。",
    cover_image: null,
    created_at: "2025-09-22T10:00:00+08:00",
    view_count: 203,
    theme: { name: "技术", slug: "tech" },
    tags: [
      { name: "Astro", slug: "astro" },
      { name: "TypeScript", slug: "typescript" },
    ],
  },
  {
    title: "学习全栈的第一个月",
    slug: "first-month-fullstack",
    summary: "从前端入门到尝试写后端，记录全栈学习路上的困惑与收获。",
    cover_image: null,
    created_at: "2025-09-01T08:00:00+08:00",
    view_count: 168,
    theme: { name: "随笔", slug: "essay" },
    tags: [
      { name: "全栈", slug: "fullstack" },
      { name: "生活", slug: "life" },
    ],
  },
] as const;

// ================================
// 派生数据（由 allPosts 自动计算）
// ================================

/** 按年份分组的归档数据 */
function deriveArchiveYears() {
  const yearMap = new Map<number, Array<{ title: string; slug: string; created_at: string; month: number; day: number }>>();
  for (const post of allPosts) {
    const d = new Date(post.created_at);
    const year = d.getFullYear();
    if (!yearMap.has(year)) yearMap.set(year, []);
    yearMap.get(year)!.push({
      title: post.title,
      slug: post.slug,
      created_at: post.created_at,
      month: d.getMonth() + 1,
      day: d.getDate(),
    });
  }
  // 按年份降序
  return Array.from(yearMap.entries())
    .sort((a, b) => b[0] - a[0])
    .map(([year, posts]) => ({
      year,
      posts: posts.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()),
    }));
}

/** 标签云（从 allPosts 统计） */
function deriveTagCloud() {
  const tagCount = new Map<string, { name: string; slug: string; count: number }>();
  for (const post of allPosts) {
    for (const tag of post.tags) {
      const existing = tagCount.get(tag.slug);
      if (existing) {
        existing.count++;
      } else {
        tagCount.set(tag.slug, { name: tag.name, slug: tag.slug, count: 1 });
      }
    }
  }
  const tags = Array.from(tagCount.values()).sort((a, b) => b.count - a.count);
  const maxCount = Math.max(...tags.map((t) => t.count));
  return tags.map((t) => ({
    ...t,
    size: Math.max(1, Math.min(5, Math.ceil((t.count / maxCount) * 5))),
  }));
}

/** 按标签分组的文章列表 */
function deriveTagPosts() {
  const map: Record<string, Array<{ title: string; slug: string; created_at: string; summary: string; cover_image: string | null }>> = {};
  for (const post of allPosts) {
    for (const tag of post.tags) {
      if (!map[tag.slug]) map[tag.slug] = [];
      map[tag.slug].push({
        title: post.title,
        slug: post.slug,
        created_at: post.created_at,
        summary: post.summary ?? "",
        cover_image: post.cover_image ?? null,
      });
    }
  }
  // 每组按日期降序
  for (const key of Object.keys(map)) {
    map[key].sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime());
  }
  return map;
}

/** 主题分类（从 allPosts 统计） */
function deriveThemeItems() {
  const themeMap = new Map<string, { name: string; slug: string; post_count: number; description: string }>();
  const descriptions: Record<string, string> = {
    tech: "HTML、CSS、Rust、TypeScript 以及各种框架的探索与实践",
    essay: "日常的观察、感受和写给自己的话",
  };
  for (const post of allPosts) {
    if (!post.theme) continue;
    const existing = themeMap.get(post.theme.slug);
    if (existing) {
      existing.post_count++;
    } else {
      themeMap.set(post.theme.slug, {
        name: post.theme.name,
        slug: post.theme.slug,
        post_count: 1,
        description: descriptions[post.theme.slug] ?? "",
      });
    }
  }
  return Array.from(themeMap.values()).sort((a, b) => b.post_count - a.post_count);
}

export const derivedData = {
  archiveYears: deriveArchiveYears(),
  tagCloud: deriveTagCloud(),
  tagPosts: deriveTagPosts(),
  themeItems: deriveThemeItems(),
} as const;

/**
 * 汇总导出，便于后台配置面板一次性读取
 */
export const yukilogConfig = {
  designTokens,
  siteConfig,
  navItems,
  contentConfig,
  allPosts,
  derivedData,
} as const;
