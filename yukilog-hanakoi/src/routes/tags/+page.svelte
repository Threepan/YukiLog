<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/state';
  import PageHero from '../../components/shared/PageHero.svelte';
  import PostListCard from '../../components/shared/PostListCard.svelte';
  import { contentConfig } from '$lib/config';
  import { navIcons } from '$lib/svg-icons';

  const tagIcon = navIcons.tag;
  const tagsPageConfig = contentConfig.pages.tags;

  let { data } = $props();
  const { tags, tagPosts } = data;

  // 计算标签云大小
  const maxCount = Math.max(...tags.map((t: any) => t.post_count), 1);
  const tagCloud = tags.map((tag: any) => ({
    ...tag,
    size: Math.max(1, Math.min(5, Math.ceil((tag.post_count / maxCount) * 5))),
  }));

  const colorCycle = tagsPageConfig.colorCycle;
  const colorNames = tagsPageConfig.colorNames;

  // 当前选中的标签
  let activeSlug: string | null = $state(null);
  let activeTagName = $state('');
  let showPosts = $state(false);

  // 当前选中标签的文章
  const activePosts = $derived(activeSlug ? (tagPosts[activeSlug] || []) : []);

  // 双栏布局
  const col0 = $derived(() => {
    if (typeof window !== 'undefined' && window.innerWidth <= 768) return activePosts;
    return activePosts.filter((_: any, i: number) => i % 2 === 0);
  });
  const col1 = $derived(() => {
    if (typeof window !== 'undefined' && window.innerWidth <= 768) return [];
    return activePosts.filter((_: any, i: number) => i % 2 === 1);
  });

  function selectTag(slug: string, name: string) {
    if (activeSlug === slug) {
      closePanel();
      return;
    }

    activeSlug = slug;
    activeTagName = name;
    showPosts = true;

    // 增加标签浏览量
    fetch(`/api/public/tags/${slug}/view`, { method: 'POST' }).catch(() => {});

    // 平滑滚动到文章区域
    setTimeout(() => {
      document.getElementById('tagPosts')?.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }, 100);
  }

  function closePanel() {
    activeSlug = null;
    activeTagName = '';
    showPosts = false;
  }

  // 标签云和卡片入场动画
  let cloudVisible = $state(false);

  onMount(() => {
    // 标签云入场
    const cloudSection = document.querySelector('.tag-cloud-section');
    if (cloudSection) {
      const cloudObserver = new IntersectionObserver(
        (entries) => {
          entries.forEach((entry) => {
            if (entry.isIntersecting) {
              cloudVisible = true;
              cloudObserver.unobserve(entry.target);
            }
          });
        },
        { threshold: 0.1 }
      );
      cloudObserver.observe(cloudSection);
    }

    // 卡片入场
    const cardObserver = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('visible');
            cardObserver.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.1 }
    );

    // 监听卡片出现（响应式重新绑定）
    const interval = setInterval(() => {
      document.querySelectorAll('.post-list-card:not(.visible)').forEach((card) => {
        cardObserver.observe(card);
      });
    }, 200);

    // 支持 ?tag=xxx 自动选中
    const autoTag = page.url.searchParams.get('tag');
    if (autoTag) {
      const target = tagCloud.find((t: any) => t.slug === autoTag);
      if (target) {
        setTimeout(() => selectTag(target.slug, target.name), 700);
      }
    }

    return () => {
      clearInterval(interval);
      cardObserver.disconnect();
    };
  });
</script>

<svelte:head>
  <title>标签 - YukiLog</title>
  <meta name="description" content="按标签分类浏览所有文章" />
</svelte:head>

<PageHero title="标签" subtitle="共 {tagCloud.length} 个标签" icon={tagIcon} />

<div class="tags-page">
  <!-- 标签云 -->
  <section class="tag-cloud-section" class:visible={cloudVisible}>
    <div class="tag-cloud">
      {#each tagCloud as tag, i}
        <button
          class="tag-bubble"
          class:active={activeSlug === tag.slug}
          data-size={tag.size}
          data-color={colorNames[colorCycle[i % colorCycle.length]]}
          style="--i: {i}"
          onclick={() => selectTag(tag.slug, tag.name)}
        >
          <span class="tag-name"># {tag.name}</span>
          <span class="tag-count">{tag.post_count}</span>
        </button>
      {/each}
    </div>
  </section>

  <!-- 文章列表 -->
  <section class="tag-posts-section" class:show={showPosts} id="tagPosts">
    <div class="tag-posts-header">
      <h2 class="tag-posts-title">{activeTagName}</h2>
      <button class="tag-posts-close" onclick={closePanel} aria-label="关闭">×</button>
    </div>

    {#if activePosts.length > 0}
      <div class="tag-group">
        <div class="tag-group-column">
          {#each col0() as post, i}
            <PostListCard {post} index={i * 2} />
          {/each}
        </div>
        {#if col1().length > 0}
          <div class="tag-group-column">
            {#each col1() as post, i}
              <PostListCard {post} index={i * 2 + 1} />
            {/each}
          </div>
        {/if}
      </div>
    {:else if showPosts}
      <p class="tag-posts-empty">{tagsPageConfig.emptyText}</p>
    {/if}
  </section>
</div>

<style>
  .tags-page {
    max-width: 1000px;
    margin: 0 auto;
    padding: var(--spacing-xl) var(--spacing-lg) calc(var(--spacing-xxl) * 2);
  }

  /* ================================ */
  /* 标签云 */
  /* ================================ */
  .tag-cloud-section {
    opacity: 0;
    transform: translateY(20px);
    transition:
      opacity 600ms var(--ease-gentle),
      transform 600ms var(--ease-gentle);

    &.visible {
      opacity: 1;
      transform: translateY(0);

      .tag-bubble {
        opacity: 1;
        transform: translateY(0);
      }
    }
  }

  .tag-cloud {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-md) 0;
  }

  .tag-bubble {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 7px 14px;
    border: 1.5px solid transparent;
    border-radius: 24px;
    background: var(--color-surface, #fff);
    color: var(--color-text-light);
    cursor: pointer;
    font-family: inherit;
    transition:
      border-color var(--transition-fast) var(--ease-gentle),
      background var(--transition-fast) var(--ease-gentle),
      box-shadow var(--transition-fast) var(--ease-gentle),
      transform var(--transition-fast) var(--ease-gentle),
      opacity 400ms var(--ease-gentle),
      color var(--transition-fast) var(--ease-gentle);

    opacity: 0;
    transform: translateY(12px);
    transition-delay: calc(var(--i) * 40ms);

    &[data-size="1"] { font-size: 11px; padding: 5px 10px; }
    &[data-size="2"] { font-size: 12px; padding: 6px 12px; }
    &[data-size="3"] { font-size: 13px; padding: 7px 14px; }
    &[data-size="4"] { font-size: 14px; padding: 8px 16px; }
    &[data-size="5"] { font-size: 15px; padding: 9px 18px; }

    &[data-color="pink"] {
      background: rgba(232, 164, 180, 0.12);
      border-color: rgba(232, 164, 180, 0.2);
      .tag-name { color: var(--color-pink-d18); }
      .tag-count { background: rgba(232, 164, 180, 0.15); color: var(--color-pink); }
      &:hover { background: rgba(232, 164, 180, 0.2); box-shadow: var(--shadow-pink); transform: translateY(-2px); }
    }

    &[data-color="blue"] {
      background: rgba(126, 182, 217, 0.12);
      border-color: rgba(126, 182, 217, 0.2);
      .tag-name { color: var(--color-blue-d20); }
      .tag-count { background: rgba(126, 182, 217, 0.15); color: var(--color-blue); }
      &:hover { background: rgba(126, 182, 217, 0.2); box-shadow: var(--shadow-blue); transform: translateY(-2px); }
    }

    &[data-color="white"] {
      background: var(--color-surface, #fff);
      border-color: var(--color-border);
      .tag-name { color: var(--color-text); }
      .tag-count { background: var(--muted-alpha-10); color: var(--color-text-muted); }
      &:hover { background: var(--color-white); box-shadow: var(--shadow-sm); transform: translateY(-2px); }
    }

    &.active {
      background: var(--color-blue);
      color: var(--color-on-primary, #fff);
      border-color: var(--color-blue);
      box-shadow: var(--shadow-blue-offset-hover);
      .tag-name { color: var(--color-on-primary, #fff); }
      .tag-count { background: rgba(255, 255, 255, 0.25); color: var(--color-on-primary, #fff); }
    }
  }

  .tag-name {
    font-weight: var(--font-weight-semibold);
    letter-spacing: 0.01em;
  }

  .tag-count {
    font-size: var(--font-size-xs);
    padding: 1px 8px;
    border-radius: 10px;
    transition:
      background var(--transition-fast) var(--ease-gentle),
      color var(--transition-fast) var(--ease-gentle);
  }

  /* ================================ */
  /* 文章列表面板 */
  /* ================================ */
  .tag-posts-section {
    margin-top: var(--spacing-xl);
    max-height: 0;
    overflow: hidden;
    opacity: 0;
    transition:
      max-height 500ms var(--ease-gentle),
      opacity 400ms var(--ease-gentle);

    &.show {
      max-height: 5000px;
      opacity: 1;
    }
  }

  .tag-posts-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--spacing-md);
    padding-bottom: var(--spacing-sm);
    border-bottom: 1.5px solid var(--color-divider);
  }

  .tag-posts-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-blue);
    margin: 0;
    letter-spacing: 0.02em;
  }

  .tag-posts-close {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: var(--muted-alpha-08);
    color: var(--color-text-muted);
    border-radius: 50%;
    cursor: pointer;
    font-size: var(--font-size-lg);
    transition:
      background var(--transition-fast) var(--ease-gentle),
      color var(--transition-fast) var(--ease-gentle);

    &:hover {
      background: rgba(232, 164, 180, 0.12);
      color: var(--color-pink);
    }
  }

  .tag-group {
    display: flex;
    gap: var(--spacing-lg);
    align-items: flex-start;
  }

  .tag-group-column {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .tag-posts-empty {
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-style: italic;
    padding: var(--spacing-xl) 0;
  }

  @media (max-width: 768px) {
    .tag-group {
      flex-direction: column;
    }

    .tag-group-column:not(:first-child) {
      display: none;
    }
  }

  @media (max-width: 640px) {
    .tags-page {
      padding: var(--spacing-lg) var(--spacing-sm) var(--spacing-xxl);
    }

    .tag-cloud {
      gap: var(--spacing-xs);
    }
  }
</style>
