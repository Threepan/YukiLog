import type { PageServerLoad } from './$types';
import { postsApi } from '$lib/api';

export const load: PageServerLoad = async () => {
  try {
    const response = await postsApi.list({
      page_size: 999,
      status: 'published',
    });

    const allPosts = (response.items || []).map((item: any) => ({
      ...item.post,
      theme: item.theme,
      tags: item.tags,
    }));

    // 按年份分组
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

    const archiveData = Array.from(yearMap.entries())
      .sort((a, b) => b[0] - a[0])
      .map(([year, posts]) => ({
        year,
        posts: posts.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime()),
      }));

    const totalPosts = archiveData.reduce((sum, y) => sum + y.posts.length, 0);

    return { archiveData, totalPosts };
  } catch (err) {
    console.error('获取文章列表失败：', err);
  }

  return { archiveData: [], totalPosts: 0 };
};
