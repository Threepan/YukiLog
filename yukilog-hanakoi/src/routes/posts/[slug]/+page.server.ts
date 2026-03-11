import type { PageServerLoad } from './$types';
import { postsApi, commentsApi } from '$lib/api';
import { renderMarkdown } from '$lib/markdown';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
  const { slug } = params;

  // 获取文章详情
  let currentPost;
  try {
    currentPost = await postsApi.getBySlug(slug);
  } catch (err) {
    console.error('获取文章详情失败：', err);
    error(404, '文章不存在');
  }

  const post = currentPost.post;
  const postTheme = currentPost.theme;
  const postTags = currentPost.tags;

  // 增加浏览数（fire-and-forget）
  postsApi.incrementView(slug).catch((err) => {
    console.error('增加浏览数失败：', err);
  });

  // 获取评论列表
  let comments: any[] = [];
  try {
    comments = await commentsApi.getPostComments(slug);
  } catch (err) {
    console.error('获取评论失败：', err);
  }

  // 渲染 Markdown
  const { html: markdownHtml, headings } = await renderMarkdown(post.content);

  return {
    post,
    theme: postTheme,
    tags: postTags,
    markdownHtml,
    headings,
    comments,
    slug,
  };
};
