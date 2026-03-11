<script lang="ts">
  import { onMount } from 'svelte';
  import PageHero from '../../../components/shared/PageHero.svelte';
  import PostListCard from '../../../components/shared/PostListCard.svelte';
  import { navIcons } from '$lib/svg-icons';
  import { contentConfig } from '$lib/config';

  const themeIcon = navIcons.theme;

  let { data } = $props();
  const { theme, posts, slug } = data;

  // 双栏布局数据
  let col0: typeof posts = $state([]);
  let col1: typeof posts = $state([]);
  let isMobile = $state(false);

  function layoutCards() {
    const mobile = typeof window !== 'undefined' && window.innerWidth <= 768;
    isMobile = mobile;

    if (mobile) {
      col0 = posts;
      col1 = [];
    } else {
      // 简单交替分配（服务端安全，不依赖 DOM 测量）
      const c0: typeof posts = [];
      const c1: typeof posts = [];
      posts.forEach((p: any, i: number) => {
        if (i % 2 === 0) c0.push(p);
        else c1.push(p);
      });
      col0 = c0;
      col1 = c1;
    }
  }

  // 初始化
  layoutCards();

  onMount(() => {
    // 增加主题浏览量
    fetch(`/api/public/themes/${slug}/view`, { method: 'POST' }).catch(() => {});

    // 重新布局
    layoutCards();

    // 卡片入场动画
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('visible');
            observer.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.08 }
    );

    document.querySelectorAll('.post-list-card').forEach((card) => observer.observe(card));

    let resizeTimer: ReturnType<typeof setTimeout>;
    function onResize() {
      clearTimeout(resizeTimer);
      resizeTimer = setTimeout(() => {
        layoutCards();
        // 重新绑定 observer
        document.querySelectorAll('.post-list-card').forEach((card) => observer.observe(card));
      }, 300);
    }

    window.addEventListener('resize', onResize);

    return () => {
      observer.disconnect();
      window.removeEventListener('resize', onResize);
    };
  });
</script>

<svelte:head>
  <title>{theme.name} - YukiLog</title>
  <meta name="description" content={theme.description || `${theme.name} 主题下的所有文章`} />
</svelte:head>

<PageHero
  title={theme.name}
  subtitle={theme.description || `共 ${theme.post_count} 篇文章`}
  icon={themeIcon}
/>

<div class="theme-detail-page">
  <a href="/themes" class="back-link">{contentConfig.pages.themes.backToAll}</a>

  <div class="post-list-container">
    <div class="post-list-column">
      {#each col0 as post, i}
        <PostListCard {post} index={i * 2} />
      {/each}
    </div>
    {#if !isMobile}
      <div class="post-list-column">
        {#each col1 as post, i}
          <PostListCard {post} index={i * 2 + 1} />
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .theme-detail-page {
    max-width: 1000px;
    margin: 0 auto;
    padding: var(--spacing-xl) var(--spacing-lg) calc(var(--spacing-xxl) * 2);
  }

  .post-list-container {
    display: flex;
    gap: var(--spacing-lg);
    align-items: flex-start;
  }

  .post-list-column {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .back-link {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    margin-bottom: var(--spacing-lg);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    text-decoration: none;
    transition: color var(--transition-fast) var(--ease-gentle);

    &:hover { color: var(--color-blue); }
  }

  @media (max-width: 768px) {
    .post-list-container {
      flex-direction: column;
    }
  }

  @media (max-width: 640px) {
    .theme-detail-page {
      padding: var(--spacing-lg) var(--spacing-sm) var(--spacing-xxl);
    }
  }
</style>
