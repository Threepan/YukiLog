<script lang="ts">
  import { onMount } from 'svelte';
  import PageHero from '../../components/shared/PageHero.svelte';
  import { contentConfig } from '$lib/config';
  import { navIcons } from '$lib/svg-icons';
  import type { Note } from '$types/api';

  let { data } = $props();

  const notesPageConfig = contentConfig.pages.notes;
  const moodLabels: Record<string, string> = notesPageConfig.moodLabels;

  // 扩展类型：包含服务端渲染的 HTML
  type NoteWithHtml = Note & { renderedContent: string };

  const initialNotes = data.notes;
  const initialTotalPages = data.totalPages;

  let notes: NoteWithHtml[] = $state([...initialNotes]);
  let currentPage = $state(1);
  let isLoading = $state(false);
  let allLoaded = $state(initialTotalPages <= 1);

  function formatDate(iso: string) {
    return new Date(iso).toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  function escapeForPreview(text: string) {
    const div = document.createElement('div');
    div.textContent = text;
    return `<p>${div.innerHTML}</p>`;
  }

  async function loadMore() {
    if (isLoading || allLoaded) return;
    isLoading = true;
    currentPage += 1;

    try {
      const apiBase = window.location.origin;
      const res = await fetch(
        `${apiBase}/api/public/notes?page=${currentPage}&page_size=${data.pageSize}`
      );
      const json = await res.json();

      if (json.success && json.data?.items?.length > 0) {
        const newNotes: NoteWithHtml[] = json.data.items.map((note: Note) => {
          const truncated = note.content.length > 200
            ? note.content.slice(0, 200) + '…'
            : note.content;
          return { ...note, renderedContent: escapeForPreview(truncated) };
        });
        notes = [...notes, ...newNotes];
      }

      if (currentPage >= data.totalPages) {
        allLoaded = true;
      }
    } catch (err) {
      console.error('加载随记失败:', err);
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    if (allLoaded) return;

    const loader = document.getElementById('notes-loader');
    if (!loader) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          loadMore();
        }
      },
      { rootMargin: '200px' }
    );

    observer.observe(loader);
    return () => observer.disconnect();
  });
</script>

<svelte:head>
  <title>随记 - YukiLog</title>
  <meta name="description" content="记录碎片化的想法与日常" />
</svelte:head>

<PageHero title="随记" subtitle={notesPageConfig.heroSubtitle} icon={navIcons.notes} />

<div class="notes-page">
  {#if notes.length === 0}
    <div class="notes-empty">
      <p>{notesPageConfig.emptyText}</p>
    </div>
  {:else}
    <div class="notes-list" id="notes-list">
      {#each notes as note, i}
        <article class="note-card" data-index={i} style="animation-delay: {i * 60}ms">
          <div class="note-meta">
            <time class="note-time">{formatDate(note.created_at)}</time>
            {#if note.mood}
              <span class="note-mood">{moodLabels[note.mood] || note.mood}</span>
            {/if}
          </div>
          <div class="note-body markdown-content">{@html note.renderedContent}</div>
          <a href={`/notes/${note.id}`} class="note-read-more">阅读全文 →</a>
        </article>
      {/each}
    </div>
  {/if}

  {#if !allLoaded}
    <div class="notes-loader" id="notes-loader">
      <div class="loader-spinner"></div>
      <p class="loader-text">{isLoading ? notesPageConfig.loadingText : ''}</p>
    </div>
  {/if}

  {#if allLoaded}
    <div class="notes-end">
      <p>{notesPageConfig.loadMoreText}</p>
    </div>
  {/if}
</div>

<style>
  .notes-page {
    max-width: 720px;
    margin: 0 auto;
    padding: var(--spacing-xl) var(--spacing-lg) var(--spacing-xxl);
  }

  .notes-empty {
    text-align: center;
    padding: var(--spacing-xxl) 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-lg);
  }

  .note-card {
    background: var(--color-white);
    border-radius: var(--radius-lg);
    padding: var(--spacing-lg) var(--spacing-xl);
    margin-bottom: var(--spacing-lg);
    box-shadow: var(--shadow-sm);
    border: 1px solid var(--color-border);
    opacity: 0;
    transform: translateY(12px);
    animation: note-enter 400ms var(--ease-gentle) forwards;
    transition: box-shadow 300ms var(--ease-gentle),
                transform 300ms var(--ease-gentle);

    &:hover {
      box-shadow: var(--shadow-md);
      transform: translateY(-2px);
    }
  }

  @keyframes note-enter {
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .note-meta {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    margin-bottom: var(--spacing-md);
    flex-wrap: wrap;
  }

  .note-time {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .note-mood {
    font-size: var(--font-size-xs);
    padding: 2px 8px;
    background: linear-gradient(135deg, rgba(126, 182, 217, 0.1), rgba(232, 164, 180, 0.1));
    border-radius: var(--radius-full);
    color: var(--color-text-light);
  }

  .note-body {
    font-size: var(--font-size-base);
    color: var(--color-text);
    line-height: 1.75;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 5;
    -webkit-box-orient: vertical;

    :global(p) {
      margin: 0 0 var(--spacing-sm);
    }

    :global(pre) {
      font-size: var(--font-size-sm);
      max-height: 120px;
      overflow: hidden;
    }
  }

  .note-read-more {
    display: inline-block;
    margin-top: var(--spacing-sm);
    font-size: var(--font-size-sm);
    color: var(--color-blue);
    text-decoration: none;
    transition: color 200ms var(--ease-gentle);

    &:hover {
      color: var(--color-pink);
    }
  }

  .notes-loader {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: var(--spacing-xl) 0;
    gap: var(--spacing-sm);
  }

  .loader-spinner {
    width: 24px;
    height: 24px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-blue);
    border-radius: 50%;
    animation: spin 800ms linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .loader-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .notes-end {
    text-align: center;
    padding: var(--spacing-xl) 0;

    p {
      font-size: var(--font-size-sm);
      color: var(--color-text-muted);
      letter-spacing: 0.5px;
    }
  }
</style>
