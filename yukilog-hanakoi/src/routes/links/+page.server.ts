import type { PageServerLoad } from './$types';
import { linksApi } from '$lib/api';

export const load: PageServerLoad = async () => {
  try {
    const allLinks = await linksApi.list();
    const friends = allLinks
      .filter((link) => link.status === 'active')
      .map((link) => ({
        name: link.title,
        description: link.description || '',
        avatar: link.avatar,
        url: link.url,
      }));

    return { friends };
  } catch (err) {
    console.error('获取友链列表失败：', err);
  }

  return { friends: [] };
};
