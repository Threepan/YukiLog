// ================================
// YukiLog 站点配置
// ================================

import type { SiteConfig, NavItem } from "../types";

export const siteConfig: SiteConfig = {
  // 基本信息
  name: "YukiLog",
  title: "恋的博客 - 一个温柔的技术日记本",
  description: "记录技术、思考、情绪与挣扎",
  welcomeText: "欢迎来看恋的博客",

  // 作者信息
  author: {
    name: "Lian",
    nickname: "恋",
    avatar: "/images/avatar.jpg",
    bio: "我能走到这里，是因为你没有放弃",
  },

  // 社交链接
  social: [
    {
      name: "GitHub",
      url: "https://github.com/Yueosa",
      icon: "github",
      color: "#6E7F8D", // 柔灰蓝
    },
    {
      name: "QQ Group",
      url: "https://qm.qq.com/cgi-bin/qm/qr?k=O6KD1bt5WDvQw47kzjaDuYIASzar_y-F&jump_from=webapi&authKey=AnF+0ddOwtFY4laf9lDJ9Om7tj5oZE2dfuHJlQfOO2CXaeTOOVdJxlxIg9wSs4WQ",
      icon: "qq",
      color: "#E3A0AE", // 柔粉
    },
    {
      name: "Bilibili",
      url: "https://space.bilibili.com/433677987",
      icon: "bilibili",
      color: "#7EB6D9", // 主蓝
    },
    {
      name: "X (Twitter)",
      url: "https://x.com/Yosa04942475621",
      icon: "twitter",
      color: "#8FAFC4", // 低饱和蓝
    },
    {
      name: "网易云音乐",
      url: "https://music.163.com/#/user/home?id=630887153",
      icon: "netease-music",
      color: "#E8A4B4", // 主粉
    },
    {
      name: "Gmail",
      url: "mailto:yichengxin7@gmail.com",
      icon: "gmail",
      color: "#D6A1AE", // 柔粉偏灰
    },
  ],

  // 站点统计
  startDate: "2026-02-12",

  // SEO
  seo: {
    keywords: ["博客", "技术", "Rust", "TypeScript", "前端", "后端", "全栈"],
    ogImage: "/images/og-image.jpg",
  },
};

// API 基础 URL（从环境变量获取）
export const API_BASE_URL =
  import.meta.env.PUBLIC_API_URL || "http://localhost:3000";

// 导航项配置
export const navItems: NavItem[] = [
  {
    label: "主页",
    href: "/",
    icon: "home",
  },
  {
    label: "主题",
    href: "/themes",
    icon: "theme",
  },
  {
    label: "归档",
    href: "/archive",
    icon: "archive",
  },
  {
    label: "标签",
    href: "/tags",
    icon: "tag",
  },
  {
    label: "友链",
    href: "/links",
    icon: "links",
  },
  {
    label: "关于",
    href: "/about",
    icon: "about",
  },
];
