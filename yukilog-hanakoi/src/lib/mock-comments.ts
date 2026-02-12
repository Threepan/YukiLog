// ================================
// Mock 评论数据 - 用于开发期间占位展示
// ================================

import type { Comment, CommentNode } from '../types/api';

// Mock 数据：评论树
export const mockComments: CommentNode[] = [
  {
    comment: {
      id: 1,
      post_id: 1,
      content: "写得真好！这篇文章让我对 Astro 有了全新的认识。特别是关于 **静态生成** 和 **按需加载** 的部分，之前一直没理解透彻。\n\n期待更多这样的深度分享 ✨",
      guest_nick: "小透明",
      guest_email: "xiaotouming@example.com",
      guest_website: "https://blog.xiaotouming.dev",
      visitor_info: "首尔 · Mobile Chrome 137.0 · Android 10",
      status: "approved",
      created_at: "2026-02-10T14:30:00+08:00",
    },
    children: [
      {
        comment: {
          id: 2,
          post_id: 1,
          content: "感谢支持！后续会继续写关于 Astro Islands 架构的文章 🌊",
          guest_nick: "恋",
          guest_email: "yichengxin7@gmail.com",
          visitor_info: "上海 · Desktop Edge 131.0 · Windows 11",
          parent_id: 1,
          root_id: 1,
          status: "approved",
          created_at: "2026-02-10T15:00:00+08:00",
        },
        children: [],
      },
    ],
  },
  {
    comment: {
      id: 3,
      post_id: 1,
      content: "有个小建议：能不能加一节关于 [Astro 与 Next.js 的对比](https://docs.astro.build/en/guides/migrate-to-astro/from-nextjs/)？很多人在这两者之间纠结。",
      guest_nick: "CodeDreamer",
      guest_email: "dreamer@example.com",
      guest_website: "https://codedreamer.io",
      visitor_info: "东京 · Desktop Chrome 136.0 · macOS 15",
      status: "approved",
      created_at: "2026-02-11T09:20:00+08:00",
    },
    children: [
      {
        comment: {
          id: 4,
          post_id: 1,
          content: "好建议！下篇文章就写这个对比 👍",
          guest_nick: "恋",
          guest_email: "lian@yukilog.dev",
          visitor_info: "上海 · Desktop Safari 18.0 · macOS 15",
          parent_id: 3,
          root_id: 3,
          status: "approved",
          created_at: "2026-02-11T10:00:00+08:00",
        },
        children: [],
      },
      {
        comment: {
          id: 5,
          post_id: 1,
          content: "我也想看这个对比！尤其是性能和 DX 方面的差异 🤔",
          guest_nick: "桜の記録",
          guest_email: "sakura@example.com",

          visitor_info: "京都 · Mobile Safari 18.0 · iOS 18",
          parent_id: 3,
          root_id: 3,
          status: "approved",
          created_at: "2026-02-11T11:30:00+08:00",
        },
        children: [
          {
            comment: {
              id: 6,
              post_id: 1,
              content: "性能方面 Astro 的零 JS 默认策略确实有优势，不过 Next.js 13+ 的 Server Components 也很强",
              guest_nick: "全栈小白",
              guest_email: "fullstack@example.com",
              visitor_info: "台北 · Desktop Firefox 135.0 · Ubuntu 24.04",
              parent_id: 5,
              root_id: 3,
              status: "approved",
              created_at: "2026-02-11T14:00:00+08:00",
            },
            children: [],
          },
        ],
      },
    ],
  },
  {
    comment: {
      id: 7,
      post_id: 1,
      content: "刚入门前端，这篇文章看得有点吃力 😅 能不能出一个**新手向的 Astro 入门教程**？",
      guest_nick: "前端萌新",
      guest_email: "newbie@example.com",
      visitor_info: "新加坡 · Mobile Chrome 137.0 · Android 13",
      status: "approved",
      created_at: "2026-02-12T08:00:00+08:00",
    },
    children: [],
  },
];

// 辅助函数：计算相对时间
export function getRelativeTime(dateString: string): string {
  const now = new Date();
  const date = new Date(dateString);
  const diffMs = now.getTime() - date.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffSec < 60) return '刚刚';
  if (diffMin < 60) return `${diffMin} 分钟前`;
  if (diffHour < 24) return `${diffHour} 小时前`;
  if (diffDay < 30) return `${diffDay} 天前`;
  if (diffDay < 365) return `${Math.floor(diffDay / 30)} 个月前`;
  return `${Math.floor(diffDay / 365)} 年前`;
}
