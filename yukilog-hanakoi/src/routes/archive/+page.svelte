<script lang="ts">
  import { onMount } from 'svelte';
  import PageHero from '../../components/shared/PageHero.svelte';
  import { contentConfig } from '$lib/config';
  import { navIcons } from '$lib/svg-icons';

  const archiveIcon = navIcons.archive;
  const archivePageConfig = contentConfig.pages.archive;

  let { data } = $props();
  let archiveData = $derived(data.archiveData);
  let totalPosts = $derived(data.totalPosts);

  onMount(() => {
    const yearGroups = document.querySelectorAll<HTMLElement>('.year-group');
    const timelineEnd = document.querySelector<HTMLElement>('.timeline-end');

    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('visible');
            observer.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.15 }
    );

    yearGroups.forEach((g) => observer.observe(g));
    if (timelineEnd) observer.observe(timelineEnd);

    return () => observer.disconnect();
  });
</script>

<svelte:head>
  <title>归档 - YukiLog</title>
  <meta name="description" content="按时间线浏览所有文章" />
</svelte:head>

<PageHero
  title="归档"
  subtitle="{archivePageConfig.heroSubtitlePrefix} {totalPosts} {archivePageConfig.heroSubtitleSuffix}"
  icon={archiveIcon}
/>

<div class="archive-page">
  <div class="timeline">
    {#each archiveData as yearGroup, yi}
      <section class="year-group" style="--yi: {yi}">
        <div class="year-marker">
          <span class="year-dot"></span>
          <h2 class="year-label">{yearGroup.year}</h2>
          <span class="year-count">{yearGroup.posts.length} 篇</span>
        </div>

        <div class="posts-container">
          {#each yearGroup.posts as post, pi}
            <div
              class="post-item"
              class:left={post.month % 2 === 1}
              class:right={post.month % 2 === 0}
              style="--pi: {pi}"
            >
              <div class="post-card">
                <time class="post-date">
                  {String(post.month).padStart(2, '0')}-{String(post.day).padStart(2, '0')}
                </time>
                <a href="/posts/{post.slug}" class="post-title">
                  {post.title}
                </a>
              </div>
              <span class="post-dot"></span>
            </div>
          {/each}
        </div>
      </section>
    {/each}

    <div class="timeline-end">
      <span class="end-dot"></span>
      <span class="end-text">{archivePageConfig.timelineEndText}</span>
    </div>
  </div>
</div>

<style>
  .archive-page {
    max-width: 860px;
    margin: 0 auto;
    padding: var(--spacing-xl) var(--spacing-lg) calc(var(--spacing-xxl) * 2);
  }

  .timeline {
    position: relative;

    &::before {
      content: '';
      position: absolute;
      left: 50%;
      transform: translateX(-50%);
      top: 0;
      bottom: 0;
      width: 2px;
      background: linear-gradient(
        to bottom,
        var(--color-blue),
        var(--color-pink) 60%,
        transparent
      );
      border-radius: 1px;
    }
  }

  .year-group {
    position: relative;
    margin-bottom: var(--spacing-xl);
    opacity: 0;
    transform: translateY(20px);
    transition:
      opacity 600ms var(--ease-gentle),
      transform 600ms var(--ease-gentle);
  }

  :global(.year-group.visible) {
    opacity: 1;
    transform: translateY(0);

    .post-item {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .year-marker {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-sm);
    margin-bottom: var(--spacing-lg);
    position: relative;
  }

  .year-dot {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: var(--color-blue);
    border: 3px solid var(--color-surface, #fff);
    box-shadow: var(--shadow-sm);
    z-index: 2;
  }

  .year-label {
    font-size: var(--font-size-2xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin: 0;
    letter-spacing: 0.04em;
    background: var(--color-bg);
    padding: 0 var(--spacing-md);
    position: relative;
    z-index: 3;
  }

  .year-count {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    background: rgba(126, 182, 217, 0.08);
    padding: 2px 10px;
    border-radius: 12px;
    position: relative;
    z-index: 3;
  }

  .posts-container {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .post-item {
    display: flex;
    align-items: center;
    position: relative;
    width: 100%;
    opacity: 0;
    transition:
      opacity 500ms var(--ease-gentle),
      transform 500ms var(--ease-gentle);
    transition-delay: calc(var(--pi) * 60ms + 200ms);

    .post-dot {
      position: absolute;
      left: 50%;
      top: 50%;
      transform: translate(-50%, -50%);
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: var(--color-pink);
      z-index: 2;
      flex-shrink: 0;
    }

    &.left {
      justify-content: flex-end;
      padding-right: calc(50% + 24px);
      transform: translateX(-16px);

      .post-card {
        text-align: right;
        flex-direction: row-reverse;
      }
    }

    &.right {
      justify-content: flex-start;
      padding-left: calc(50% + 24px);
      transform: translateX(16px);

      .post-card {
        text-align: left;
      }
    }
  }

  .post-card {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-xs) var(--spacing-md);
    border-radius: var(--radius-sm);
    transition: background var(--transition-fast) var(--ease-gentle);

    &:hover {
      background: rgba(126, 182, 217, 0.04);
    }
  }

  .post-date {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-family: var(--font-family-code);
    flex-shrink: 0;
    white-space: nowrap;
  }

  .post-title {
    font-size: var(--font-size-base);
    color: var(--color-text);
    text-decoration: none;
    line-height: var(--line-height-base);
    transition: color var(--transition-fast) var(--ease-gentle);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;

    &:hover { color: var(--color-blue); }
  }

  /* 时间轴终点 */
  .timeline-end {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-sm);
    position: relative;
    padding-top: var(--spacing-md);
    opacity: 0;
    transform: translateY(12px);
    transition:
      opacity 600ms var(--ease-gentle) 200ms,
      transform 600ms var(--ease-gentle) 200ms;
  }

  :global(.timeline-end.visible) {
    opacity: 1;
    transform: translateY(0);
  }

  .end-dot {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: var(--color-pink);
    border: 2px solid var(--color-surface, #fff);
    box-shadow: var(--shadow-sm);
    z-index: 2;
  }

  .end-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-style: italic;
    background: var(--color-bg);
    padding: 0 var(--spacing-md);
    position: relative;
    z-index: 3;
  }

  /* 响应式 —— 小屏单列 */
  @media (max-width: 768px) {
    .timeline::before {
      left: 20px;
      transform: none;
    }

    .year-marker {
      justify-content: flex-start;
      padding-left: 40px;
    }

    .year-dot {
      left: 20px;
      transform: translate(-50%, -50%);
    }

    .year-label {
      background: transparent;
      padding: 0;
    }

    .post-item {
      .post-dot {
        left: 20px;
        transform: translate(-50%, -50%);
      }

      &.left,
      &.right {
        justify-content: flex-start;
        padding-left: 48px;
        padding-right: 0;
        transform: translateX(-12px);

        .post-card {
          text-align: left;
          flex-direction: row;
        }
      }
    }

    .timeline-end {
      justify-content: flex-start;
      padding-left: 40px;
    }

    .end-dot {
      left: 20px;
      transform: translate(-50%, -50%);
    }

    .end-text {
      background: transparent;
      padding: 0;
    }
  }

  @media (max-width: 640px) {
    .archive-page {
      padding: var(--spacing-lg) var(--spacing-sm) var(--spacing-xxl);
    }

    .year-label {
      font-size: var(--font-size-xl);
    }

    .post-card {
      max-width: 100%;
      overflow: hidden;
    }

    .post-date {
      font-size: var(--font-size-xs);
      white-space: nowrap;
      flex-shrink: 0;
    }

    .post-title {
      font-size: var(--font-size-sm);
      white-space: normal;
      word-break: break-all;
    }
  }
</style>
