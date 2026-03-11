import type { PageServerLoad } from './$types';
import { notesApi } from '$lib/api';
import { renderMarkdown } from '$lib/markdown';

const PAGE_SIZE = 10;

export const load: PageServerLoad = async () => {
  try {
    const response = await notesApi.list({ page: 1, page_size: PAGE_SIZE });

    // 服务端渲染 Markdown 摘要（取前 200 字符）
    const notesWithHtml = await Promise.all(
      response.items.map(async (note) => {
        const truncated = note.content.length > 200
          ? note.content.slice(0, 200) + '…'
          : note.content;
        const { html } = await renderMarkdown(truncated);
        return { ...note, renderedContent: html };
      })
    );

    return {
      notes: notesWithHtml,
      totalPages: response.total_pages,
      pageSize: PAGE_SIZE,
    };
  } catch (err) {
    console.error('获取随记列表失败：', err);
  }

  return {
    notes: [],
    totalPages: 1,
    pageSize: PAGE_SIZE,
  };
};
