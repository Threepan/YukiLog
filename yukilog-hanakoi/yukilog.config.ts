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

/**
 * 汇总导出，便于后台配置面板一次性读取
 */
export const yukilogConfig = {
  designTokens,
  siteConfig,
  navItems,
  contentConfig,
} as const;

