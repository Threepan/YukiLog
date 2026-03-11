<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { contentConfig, siteConfig } from '$lib/config';
  import { svgIcons } from '$lib/svg-icons';
  import type { SiteStats } from '$types/api';

  interface Props {
    stats: SiteStats;
  }

  let { stats }: Props = $props();

  const infoConfig = contentConfig.components.siteInfoCard;

  const formattedWords = $derived(stats.total_words >= 1000
    ? `${(stats.total_words / 1000).toFixed(1)}k`
    : stats.total_words.toString());

  let uptimeText: string = $state(infoConfig.labels.calculating);
  let timer: ReturnType<typeof setInterval>;

  function updateUptime() {
    const start = new Date(siteConfig.startDate);
    const now = new Date();
    const diff = now.getTime() - start.getTime();

    if (diff < 0) {
      uptimeText = '0 天 0 时 0 分 0 秒';
      return;
    }

    const days = Math.floor(diff / (1000 * 60 * 60 * 24));
    const hours = Math.floor((diff % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((diff % (1000 * 60)) / 1000);

    uptimeText = `${days} 天 ${hours} 时 ${minutes} 分 ${seconds} 秒`;
  }

  onMount(() => {
    updateUptime();
    timer = setInterval(updateUptime, 1000);
  });

  onDestroy(() => {
    clearInterval(timer);
  });
</script>

<div class="siteinfo-card" id="siteinfo-card">
  <div class="siteinfo-header">
    <span class="siteinfo-label">{infoConfig.title}</span>
  </div>

  <div class="siteinfo-list">
    <div class="siteinfo-item">
      <div class="siteinfo-icon">
        <span>{@html svgIcons.folderOpen}</span>
      </div>
      <div class="siteinfo-detail">
        <span class="siteinfo-name">文章总数</span>
        <span class="siteinfo-value">{stats.total_posts} 篇</span>
      </div>
    </div>

    <div class="siteinfo-item">
      <div class="siteinfo-icon">
        <span>{@html svgIcons.wordCount}</span>
      </div>
      <div class="siteinfo-detail">
        <span class="siteinfo-name">总字数</span>
        <span class="siteinfo-value">{formattedWords} 字</span>
      </div>
    </div>

    <div class="siteinfo-item">
      <div class="siteinfo-icon">
        <span>{@html svgIcons.eye}</span>
      </div>
      <div class="siteinfo-detail">
        <span class="siteinfo-name">总浏览量</span>
        <span class="siteinfo-value">{stats.total_views.toLocaleString()} 次</span>
      </div>
    </div>

    <div class="siteinfo-item">
      <div class="siteinfo-icon">
        <span>{@html svgIcons.clock}</span>
      </div>
      <div class="siteinfo-detail">
        <span class="siteinfo-name">{infoConfig.labels.uptime}</span>
        <span class="siteinfo-value">{uptimeText}</span>
      </div>
    </div>

    <a href={infoConfig.github.url} target="_blank" rel="noopener noreferrer" class="siteinfo-item">
      <div class="siteinfo-icon">
        <span>{@html svgIcons.githubMark}</span>
      </div>
      <div class="siteinfo-detail">
        <span class="siteinfo-name">{infoConfig.github.name}</span>
        <span class="siteinfo-value link-value">{infoConfig.github.value}</span>
      </div>
      <span class="siteinfo-arrow">{@html svgIcons.chevronRight}</span>
    </a>
  </div>
</div>

<style>
  .siteinfo-card {
    background: var(--color-white);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-blue);
    padding: var(--spacing-lg);
    margin-top: var(--spacing-sm);
    display: flex;
    flex-direction: column;

    opacity: 0;
    transform: translateX(100px);
    transition: opacity 600ms cubic-bezier(0.22, 0.61, 0.36, 1),
                transform 600ms cubic-bezier(0.22, 0.61, 0.36, 1);

    &:global(.visible) {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .siteinfo-header {
    margin-bottom: var(--spacing-md);
  }

  .siteinfo-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-light);
    letter-spacing: 0.05em;
  }

  .siteinfo-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .siteinfo-item {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: 10px 12px;
    border-radius: var(--radius-sm);
    color: var(--color-text);
    text-decoration: none;
    transition: background var(--transition-base) var(--ease-gentle);

    &:hover {
      background: var(--color-bg);
    }

    &[href] {
      cursor: pointer;
    }
  }

  .siteinfo-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    color: var(--color-blue);
    background: var(--blue-alpha-08);
    border-radius: 8px;
    transition: all var(--transition-base) var(--ease-gentle);

    .siteinfo-item:hover & {
      background: var(--blue-alpha-14);
    }
  }

  .siteinfo-detail {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .siteinfo-name {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    line-height: 1;
  }

  .siteinfo-value {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
    line-height: 1.3;

    &.link-value {
      color: var(--color-blue);
    }
  }

  .siteinfo-arrow {
    flex-shrink: 0;
    color: var(--color-text-muted);
    transition: transform var(--transition-base) var(--ease-gentle);

    .siteinfo-item:hover & {
      transform: translateX(3px);
      color: var(--color-blue);
    }
  }

  @media (max-width: 768px) {
    .siteinfo-card {
      padding: var(--spacing-md);
    }
  }
</style>
