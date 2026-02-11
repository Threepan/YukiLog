// ================================
// 前端专属类型定义
// ================================

/**
 * 导航项配置
 */
export interface NavItem {
  label: string;
  href: string;
  icon?: string; // 图标名称或 SVG 路径
}

/**
 * 社交链接配置
 */
export interface SocialLink {
  name: string;
  url: string;
  icon: string; // 图标名称
  color?: string; // 品牌色
}

/**
 * 站点配置
 */
export interface SiteConfig {
  // 基本信息
  name: string;
  title: string;
  description: string;
  welcomeText: string;
  
  // 作者信息
  author: {
    name: string;
    nickname: string;
    avatar: string;
    bio: string;
  };
  
  // 社交链接
  social: SocialLink[];
  
  // 站点统计
  startDate: string; // 启动日期，格式：2026-02-12
  
  // SEO
  seo: {
    keywords: string[];
    ogImage: string;
  };
}

/**
 * 导航栏状态
 */
export type NavState = 'hidden' | 'visible' | 'sticky';

/**
 * 页面主题（亮/暗色模式）
 */
export type ThemeMode = 'light' | 'dark' | 'auto';

/**
 * 归档数据结构（按年份分组）
 */
export interface ArchiveYear {
  year: number;
  posts: ArchivePost[];
}

export interface ArchivePost {
  title: string;
  slug: string;
  created_at: string;
  month: number;
  day: number;
}

/**
 * 标签云数据
 */
export interface TagCloud {
  name: string;
  slug: string;
  count: number;
  size: number; // 字号大小（相对值，1-5）
}

/**
 * 文章卡片数据（列表展示用）
 */
export interface PostCardData {
  title: string;
  slug: string;
  summary: string | null;
  cover_image: string | null;
  created_at: string;
  view_count: number;
  // 扩展字段（前端组装）
  theme?: {
    name: string;
    slug: string;
  } | null;
  tags?: Array<{
    name: string;
    slug: string;
  }>;
}

/**
 * 面包屑导航项
 */
export interface BreadcrumbItem {
  label: string;
  href?: string; // 无 href 表示当前页
}

/**
 * Toast 通知类型
 */
export type ToastType = 'info' | 'success' | 'warning' | 'error';

export interface ToastMessage {
  id: string;
  type: ToastType;
  message: string;
  duration?: number; // 毫秒，0 表示不自动关闭
}

/**
 * 分页器配置
 */
export interface PaginationConfig {
  currentPage: number;
  totalPages: number;
  baseUrl: string; // 基础 URL，如 /posts
  showFirstLast?: boolean; // 是否显示首尾页按钮
  showPrevNext?: boolean;  // 是否显示上下页按钮
  maxVisible?: number;     // 最多显示多少页码
}
