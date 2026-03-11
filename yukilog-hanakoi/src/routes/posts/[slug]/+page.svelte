<script lang="ts">
  import { goto } from '$app/navigation';
  import { contentConfig } from '$lib/config';
  import ProfileCard from '../../../components/home/ProfileCard.svelte';
  import TableOfContents from '../../../components/posts/TableOfContents.svelte';
  import CommentSection from '../../../components/comments/CommentSection.svelte';

  const hp = contentConfig.markdown.headingPrefixes;

  let { data } = $props();
  const { post, tags: postTags, markdownHtml, headings, comments, slug } = data;

  function goBack() {
    if (typeof document !== 'undefined' && document.referrer && new URL(document.referrer).origin === window.location.origin) {
      history.back();
    } else {
      goto('/');
    }
  }
</script>

<svelte:head>
  <title>{post.title} - YukiLog</title>
  {#if post.summary}
    <meta name="description" content={post.summary} />
  {/if}
  {#if postTags.length > 0}
    <meta name="keywords" content={postTags.map((t: any) => t.name).join(', ')} />
  {/if}
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.28/dist/katex.min.css" />
</svelte:head>

<section class="post-page">
  <!-- 左侧个人卡片 -->
  <aside class="post-author-fixed">
    <ProfileCard />
  </aside>

  <!-- 文章 + 目录 -->
  <div class="post-main">
    <article
      class="post-content"
      style="--h1p:'{hp.h1}';--h2p:'{hp.h2}';--h3p:'{hp.h3}';--h4p:'{hp.h4}';--h5p:'{hp.h5}';--h6p:'{hp.h6}';"
    >
      <nav class="post-back-nav">
        <button class="post-back-link" onclick={goBack}>← 返回</button>
      </nav>

      <header class="post-header">
        <h1 class="post-title">{post.title}</h1>
        {#if post.summary}
          <p class="post-summary">{post.summary}</p>
        {/if}
        <time datetime={post.created_at}>
          {new Date(post.created_at).toLocaleDateString('zh-CN', {
            year: 'numeric',
            month: 'long',
            day: 'numeric',
          })}
        </time>
      </header>

      <div class="post-divider" aria-hidden="true"></div>

      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      <div class="markdown-body">{@html markdownHtml}</div>

      <!-- 评论区 -->
      <CommentSection {comments} />
    </article>

    <aside class="post-toc">
      {#if headings.length > 0}
        <TableOfContents {headings} />
      {/if}
    </aside>
  </div>
</section>

<style>
  /* ================================ */
  /* 文章页布局 */
  /* ================================ */
  .post-page {
    padding-top: calc(var(--spacing-xl) + 44px);
    padding-bottom: var(--spacing-xxl);
    min-height: 100vh;
  }

  .post-author-fixed {
    position: fixed;
    left: calc(var(--spacing-xl) * 2);
    top: 88px;
    width: 280px;
    z-index: 99;
  }

  :global(.post-author-fixed .profile-card) {
    opacity: 1 !important;
    transform: translateX(0) !important;
  }

  .post-main {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 220px;
    gap: var(--spacing-xl);
    align-items: start;
    max-width: 1160px;
    margin: 0 auto;
    padding: 0 var(--spacing-md);
    transform: translateX(130px);
  }

  .post-content {
    background: var(--color-white);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-blue);
    padding: var(--spacing-xl);
    width: 100%;
  }

  .post-back-nav {
    margin-bottom: var(--spacing-md);
  }

  .post-back-link {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    text-decoration: none;
    background: none;
    border: none;
    cursor: pointer;
    font-family: inherit;
    padding: 0;
    transition: color var(--transition-fast) var(--ease-gentle);

    &:hover { color: var(--color-blue); }
    &::after { content: none !important; }
  }

  .post-header {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
    margin-bottom: var(--spacing-lg);

    .post-title {
      font-size: var(--font-size-3xl);
      font-weight: var(--font-weight-bold);
      color: var(--color-text);
      line-height: var(--line-height-tight);
      &::before { content: none !important; }
    }

    .post-summary {
      color: var(--color-text-light);
      line-height: var(--line-height-relaxed);
      font-size: var(--font-size-base);
    }

    time {
      color: var(--color-text-muted);
      font-size: var(--font-size-sm);
    }
  }

  .post-divider {
    height: 1px;
    background: linear-gradient(to right, var(--color-pink), var(--color-blue));
    opacity: 0.35;
    margin-bottom: var(--spacing-xl);
  }

  /* ================================ */
  /* 右侧目录 */
  /* ================================ */
  .post-toc {
    position: sticky;
    top: 88px;
    max-height: calc(100vh - 120px);
    overflow: visible;
    padding-right: var(--spacing-lg);
  }

  /* ================================ */
  /* 响应式 */
  /* ================================ */
  @media (max-width: 1400px) {
    .post-author-fixed {
      display: none;
    }

    .post-main {
      grid-template-columns: minmax(0, 1fr);
      max-width: 780px;
      padding: 0 var(--spacing-sm);
      transform: none;
    }

    .post-toc {
      display: block;
      position: static;
      top: auto;
      max-height: none;
      padding-right: 0;
      margin-bottom: var(--spacing-md);
      order: -1;
    }

    :global(.post-toc .toc-container) {
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      padding: var(--spacing-sm);
      background: var(--color-white);
    }

    :global(.post-toc .toc-title) {
      margin-bottom: var(--spacing-xs);
      padding-left: 8px;
    }

    :global(.post-toc .toc-scroll-area) {
      max-height: 220px;
      padding: 0;
    }
  }

  @media (max-width: 768px) {
    .post-content {
      padding: var(--spacing-lg) var(--spacing-md);
    }
  }
</style>
