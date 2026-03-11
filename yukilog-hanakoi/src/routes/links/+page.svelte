<script lang="ts">
  import FriendSpotlight from '$components/links/FriendSpotlight.svelte';
  import FriendCard from '$components/links/FriendCard.svelte';
  import ApplyLinkModal from '$components/links/ApplyLinkModal.svelte';
  import { contentConfig } from '$lib/config';

  let { data } = $props();

  const linksPageConfig = contentConfig.pages.links;
  const bestFriend = linksPageConfig.bestFriend;
</script>

<svelte:head>
  <title>友链 | YukiLog</title>
  <meta name="description" content="恋的朋友们" />
</svelte:head>

<div class="links-page">
  <!-- 页头独白 -->
  <header class="links-header">
    <p class="links-greeting">{linksPageConfig.greeting}</p>
    <div class="links-divider"></div>
  </header>

  <!-- 最好的朋友 Spotlight -->
  <FriendSpotlight {...bestFriend} />

  <!-- 朋友们 - 散落便签 -->
  <section class="friends-section">
    <h2 class="friends-title">{linksPageConfig.friendsTitle}</h2>
    <div class="friends-grid">
      {#each data.friends as friend, i}
        <FriendCard {...friend} index={i} />
      {/each}
    </div>
  </section>

  <!-- 申请友链 -->
  <ApplyLinkModal />
</div>

<style>
  .links-page {
    max-width: 800px;
    margin: 0 auto;
    padding: 0 var(--spacing-lg);
    min-height: 100vh;
  }

  /* 页头 */
  .links-header {
    padding-top: 80px;
    text-align: center;
    opacity: 0;
    animation: header-enter 700ms var(--ease-gentle) 100ms forwards;
  }

  @keyframes header-enter {
    from {
      opacity: 0;
      transform: translateY(-10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .links-greeting {
    font-size: var(--font-size-lg);
    color: var(--color-text-light);
    font-style: italic;
    margin: 0;
    line-height: var(--line-height-relaxed);
  }

  .links-divider {
    width: 60px;
    height: 2px;
    background: linear-gradient(90deg, var(--color-pink), var(--color-blue));
    margin: var(--spacing-lg) auto 0;
    border-radius: 1px;
  }

  /* 朋友们区域 */
  .friends-section {
    padding-top: var(--spacing-xl);
  }

  .friends-title {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text-muted);
    text-align: center;
    margin: 0 0 var(--spacing-xl);
    letter-spacing: 0.15em;
  }

  .friends-grid {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: var(--spacing-lg);
    padding: 0 var(--spacing-md);
  }

  /* 响应式 */
  @media (max-width: 640px) {
    .links-page {
      padding: 0 var(--spacing-sm);
    }

    .links-header {
      padding-top: 70px;
    }

    .friends-grid {
      gap: var(--spacing-md);
      padding: 0;
    }
  }
</style>
