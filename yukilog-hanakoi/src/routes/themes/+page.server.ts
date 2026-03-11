import type { PageServerLoad } from './$types';
import { themesApi } from '$lib/api';

export const load: PageServerLoad = async () => {
  try {
    const themes = await themesApi.list();
    return { themes };
  } catch (err) {
    console.error('获取主题列表失败：', err);
  }

  return { themes: [] };
};
