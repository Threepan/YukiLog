<script lang="ts">
  import { onMount } from 'svelte';
  import { contentConfig } from '$lib/config';
  import ProfileCard from '../../../components/home/ProfileCard.svelte';

  onMount(async () => {
    const mermaid = (await import('mermaid')).default;
    mermaid.initialize({ startOnLoad: false, theme: 'neutral' });
    await mermaid.run();

    const { default: mediumZoom } = await import('medium-zoom');
    mediumZoom('.markdown-body img', { margin: 24, background: 'rgba(0,0,0,0.7)' });

    document.querySelectorAll<HTMLButtonElement>('.code-block .copy-btn').forEach((btn) => {
      btn.addEventListener('click', async () => {
        const code = btn.closest('.code-block')?.querySelector('code')?.innerText ?? '';
        await navigator.clipboard.writeText(code);
        btn.textContent = '已复制';
        btn.classList.add('copied');
        setTimeout(() => {
          btn.textContent = '复制';
          btn.classList.remove('copied');
        }, 2000);
      });
    });
  });

  let { data } = $props();

  const notesPageConfig = contentConfig.pages.notes;
  const hp = contentConfig.markdown.headingPrefixes;

  const moodLabel = $derived(data.note.mood
    ? (notesPageConfig.moodLabels as Record<string, string>)[data.note.mood] || data.note.mood
    : null);

  const formattedDate = $derived(new Date(data.note.created_at).toLocaleDateString('zh-CN', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    weekday: 'long',
    hour: '2-digit',
    minute: '2-digit',
  }));

  const showUpdated = $derived(data.note.updated_at !== data.note.created_at);
  const formattedUpdated = $derived(showUpdated
    ? new Date(data.note.updated_at).toLocaleDateString('zh-CN', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      })
    : '');
</script>

<svelte:head>
  <title>随记 - YukiLog</title>
  <meta name="description" content="一条随记" />
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.28/dist/katex.min.css" />
</svelte:head>

<section class="note-page">
  <!-- 左侧个人卡片 -->
  <aside class="note-author-fixed">
    <ProfileCard />
  </aside>

  <!-- 正文区域 -->
  <div class="note-main">
    <nav class="note-back-nav">
      <a href="/notes" class="note-back-link">← 返回随记</a>
    </nav>

    <article
      class="note-detail"
      style="--h1p:'{hp.h1}';--h2p:'{hp.h2}';--h3p:'{hp.h3}';--h4p:'{hp.h4}';--h5p:'{hp.h5}';--h6p:'{hp.h6}';"
    >
      <header class="note-header">
        <time class="note-date">{formattedDate}</time>
        {#if moodLabel}
          <span class="note-mood">{moodLabel}</span>
        {/if}
      </header>

      <div class="note-content markdown-body">{@html data.markdownHtml}</div>

      {#if showUpdated}
        <footer class="note-footer">
          <span class="note-updated">最后编辑于 {formattedUpdated}</span>
        </footer>
      {/if}
    </article>
  </div>
</section>

<style>
  .note-page {
    padding-top: calc(var(--spacing-xl) + 44px);
    padding-bottom: var(--spacing-xxl);
    min-height: 100vh;
  }

  .note-author-fixed {
    position: fixed;
    left: calc(var(--spacing-xl) * 2);
    top: 88px;
    width: 280px;
    z-index: 99;
  }

  :global(.note-author-fixed .profile-card) {
    opacity: 1 !important;
    transform: translateX(0) !important;
  }

  .note-main {
    max-width: 780px;
    margin: 0 auto;
    padding: 0 var(--spacing-md);
    transform: translateX(130px);
  }

  .note-back-nav {
    margin-bottom: var(--spacing-xl);
    opacity: 0;
    animation: fade-in 400ms var(--ease-gentle) 100ms forwards;
  }

  .note-back-link {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    text-decoration: none;
    transition: color 200ms var(--ease-gentle);

    &:hover {
      color: var(--color-blue);
    }
  }

  .note-detail {
    background: var(--color-white);
    border-radius: var(--radius-lg);
    padding: var(--spacing-xl) var(--spacing-xxl);
    box-shadow: var(--shadow-md);
    border: 1px solid var(--color-border);
    opacity: 0;
    animation: note-in 500ms var(--ease-gentle) 200ms forwards;
  }

  @keyframes note-in {
    from {
      opacity: 0;
      transform: translateY(16px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @keyframes fade-in {
    to { opacity: 1; }
  }

  .note-header {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    margin-bottom: var(--spacing-xl);
    padding-bottom: var(--spacing-md);
    border-bottom: 1px solid var(--color-divider);
    flex-wrap: wrap;
  }

  .note-date {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .note-mood {
    font-size: var(--font-size-xs);
    padding: 2px 10px;
    background: linear-gradient(135deg, rgba(126, 182, 217, 0.12), rgba(232, 164, 180, 0.12));
    border-radius: var(--radius-full);
    color: var(--color-text-light);
  }

  .note-footer {
    margin-top: var(--spacing-xl);
    padding-top: var(--spacing-md);
    border-top: 1px solid var(--color-divider);
  }

  .note-updated {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  @media (max-width: 1400px) {
    .note-author-fixed {
      display: none;
    }

    .note-main {
      max-width: 720px;
      padding: 0 var(--spacing-lg);
      transform: none;
    }
  }

  @media (max-width: 768px) {
    .note-page {
      padding-top: 70px;
    }

    .note-main {
      padding: 0 var(--spacing-md);
    }

    .note-detail {
      padding: var(--spacing-lg) var(--spacing-lg);
    }
  }
</style>
