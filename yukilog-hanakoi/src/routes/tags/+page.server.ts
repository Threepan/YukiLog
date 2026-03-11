import type { PageServerLoad } from './$types';
import { tagsApi, postsApi } from '$lib/api';

export const load: PageServerLoad = async () => {
  try {
    const tags = await tagsApi.list();

    // 为每个标签获取文章列表（预渲染）
    const tagPosts: Record<string, any[]> = {};
    for (const tag of tags) {
      try {
        const response = await postsApi.list({
          tag_slugs: tag.slug,
          status: 'published',
          page_size: 999,
        });
        tagPosts[tag.slug] = (response.items || []).map((item: any) => ({
          ...item.post,
          theme: item.theme,
          tags: item.tags,
        }));
      } catch {
        tagPosts[tag.slug] = [];
      }
    }

    return { tags, tagPosts };
  } catch (err) {
    console.error('获取标签列表失败：', err);
  }

  return { tags: [], tagPosts: {} };
};
