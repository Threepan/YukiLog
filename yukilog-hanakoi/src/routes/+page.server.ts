import type { PageServerLoad } from './$types';
import { postsApi, statsApi } from '$lib/api';
import type { PostCardData } from '$types/blog';
import type { SiteStats } from '$types/api';

export const load: PageServerLoad = async ({ url }) => {
  const currentSort = (url.searchParams.get('sort') || 'created_at') as
    | 'created_at'
    | 'updated_at'
    | 'view_count';

  let recentPosts: PostCardData[] = [];
  let stats: SiteStats = { total_posts: 0, total_views: 0, total_words: 0 };

  try {
    const [postsResponse, statsResponse] = await Promise.all([
      postsApi.list({
        page: 1,
        page_size: 20,
        status: 'published',
        sort: currentSort,
        is_featured: true,
      }),
      statsApi.get().catch((): SiteStats => ({
        total_posts: 0,
        total_views: 0,
        total_words: 0,
      })),
    ]);

    recentPosts = (postsResponse.items || []).map((item) => ({
      ...item.post,
      theme: item.theme,
      tags: item.tags,
    }));

    stats = statsResponse;
  } catch (err) {
    console.error('获取首页数据失败：', err);
    // 后端不可用时返回空数据，页面仍可渲染
  }

  return {
    recentPosts,
    stats,
    currentSort,
  };
};
