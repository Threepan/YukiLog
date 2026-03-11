import type { PageServerLoad } from './$types';
import { themesApi, postsApi } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
  const { slug } = params;

  try {
    const theme = await themesApi.getBySlug(slug);
    const response = await postsApi.list({
      theme_slugs: slug,
      status: 'published',
      page_size: 999,
    });
    const posts = (response.items || []).map((item: any) => ({
      ...item.post,
      theme: item.theme,
      tags: item.tags,
    }));

    return { theme, posts, slug };
  } catch (err) {
    console.error('获取主题详情失败：', err);
    error(404, '主题不存在');
  }
};
