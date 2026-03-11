<script lang="ts">
  import { onMount } from 'svelte';
  import { svgIcons } from '$lib/svg-icons';
  import { contentConfig } from '$lib/config';

  let hitokotoText = $state('加载中...');
  let hitokotoFrom = $state('');
  let loading = $state(false);
  let cardEl: HTMLDivElement;

  async function fetchHitokoto() {
    loading = true;
    await new Promise(r => setTimeout(r, 200));

    try {
      const res = await fetch('https://v1.hitokoto.cn/?c=a');
      const data = await res.json();

      hitokotoText = data.hitokoto || '...';

      if (data.from_who && data.from) {
        hitokotoFrom = `—— ${data.from_who}「${data.from}」`;
      } else if (data.from) {
        hitokotoFrom = `——「${data.from}」`;
      } else {
        hitokotoFrom = '';
      }
    } catch {
      hitokotoText = '获取失败，点击刷新重试';
      hitokotoFrom = '';
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    fetchHitokoto();
  });
</script>

<div class="hitokoto-card" id="hitokoto-card" bind:this={cardEl}>
  <div class="hitokoto-header">
    <span class="hitokoto-label">{contentConfig.components.hitokotoCard.title}</span>
    <button class="hitokoto-refresh" onclick={fetchHitokoto} aria-label="换一条" title="换一条">
      <span class="hitokoto-refresh-icon">{@html svgIcons.refreshCcw}</span>
    </button>
  </div>

  <div class="hitokoto-body">
    <span class="hitokoto-quote-icon quote-open">{@html svgIcons.openingQuotationMark}</span>
    <p class="hitokoto-text" class:loading>{hitokotoText}</p>
    <span class="hitokoto-quote-icon quote-close">{@html svgIcons.closingQuotationMark}</span>
  </div>

  <div class="hitokoto-source">
    <span class="hitokoto-from" class:loading>{hitokotoFrom}</span>
  </div>
</div>

<style>
  .hitokoto-card {
    background: var(--color-white);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-pink);
    padding: var(--spacing-lg);
    min-height: 180px;
    display: flex;
    flex-direction: column;
    gap: 0;

    opacity: 0;
    transform: translateX(100px);
    transition: opacity 600ms cubic-bezier(0.22, 0.61, 0.36, 1),
                transform 600ms cubic-bezier(0.22, 0.61, 0.36, 1);

    &:global(.visible) {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .hitokoto-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--spacing-md);
  }

  .hitokoto-label {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-light);
    letter-spacing: 0.05em;
  }

  .hitokoto-refresh {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--color-text-muted);
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: color var(--transition-base) var(--ease-gentle),
                background var(--transition-base) var(--ease-gentle);

    &:hover {
      color: var(--color-blue);
      background: var(--color-bg);
    }
  }

  .hitokoto-body {
    display: flex;
    align-items: flex-start;
    gap: var(--spacing-sm);
    padding: 0 var(--spacing-xs);
  }

  .hitokoto-quote-icon {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    margin-top: 4px;
    opacity: 0.3;
    color: var(--color-text);

    :global(svg) {
      width: 100%;
      height: 100%;
      fill: currentColor;

      :global(path) {
        fill: currentColor;
      }
    }

    &.quote-close {
      align-self: flex-end;
      margin-top: 0;
      margin-bottom: 4px;
    }
  }

  .hitokoto-text {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    color: var(--color-text);
    line-height: var(--line-height-relaxed);
    margin: 0;
    flex: 1;
    min-height: 1.6em;
    transition: opacity 200ms var(--ease-gentle);

    &.loading {
      opacity: 0;
    }
  }

  .hitokoto-source {
    margin-top: var(--spacing-md);
    padding-top: var(--spacing-sm);
    border-top: 1px solid var(--color-divider);
    display: flex;
    align-items: center;
    justify-content: flex-end;
    min-height: 1.4em;
  }

  .hitokoto-from {
    font-size: var(--font-size-sm);
    color: var(--color-text-light);
    font-style: italic;
    transition: opacity 200ms var(--ease-gentle);

    &.loading {
      opacity: 0;
    }
  }

  @media (max-width: 768px) {
    .hitokoto-card {
      padding: var(--spacing-md);
    }

    .hitokoto-text {
      font-size: var(--font-size-sm);
    }
  }
</style>
