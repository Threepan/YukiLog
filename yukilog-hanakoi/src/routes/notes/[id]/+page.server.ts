import type { PageServerLoad } from './$types';
import { notesApi } from '$lib/api';
import { renderMarkdown } from '$lib/markdown';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
  const id = Number(params.id);

  if (isNaN(id)) {
    error(404, '随记不存在');
  }

  try {
    const note = await notesApi.getById(id);
    const { html: markdownHtml } = await renderMarkdown(note.content);

    return {
      note,
      markdownHtml,
    };
  } catch (err) {
    console.error('获取随记详情失败：', err);
    error(404, '随记不存在');
  }
};
