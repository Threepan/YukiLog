<script lang="ts">
  import type { CommentNode } from '$types/api';
  import { contentConfig } from '$lib/config';
  import CommentForm from './CommentForm.svelte';
  import CommentItem from './CommentItem.svelte';

  const cc = contentConfig.components.comments;

  let { comments }: { comments: CommentNode[] } = $props();

  function countTotal(nodes: CommentNode[]): number {
    let count = 0;
    for (const node of nodes) {
      count += 1;
      count += countTotal(node.children);
    }
    return count;
  }

  const totalCount = $derived(countTotal(comments));
</script>

<section class="comment-section" id="comments">
  <div class="section-header">
    <h2 class="section-title">{cc.sectionTitle}</h2>
    <p class="section-subtitle">{cc.sectionSubtitle}</p>
  </div>

  <!-- 评论表单 -->
  <CommentForm />

  <!-- 评论列表 -->
  <div class="comment-list">
    {#if totalCount > 0}
      <div class="comment-list-header">
        <h3 class="comment-count">
          <span class="count-number">{totalCount}</span> {cc.countSuffix}
        </h3>
      </div>
      <div class="comment-list-body">
        {#each comments as node}
          <CommentItem {node} />
        {/each}
      </div>
    {:else}
      <div class="empty-state">
        <div class="empty-icon">{cc.emptyIcon}</div>
        <p class="empty-text">{cc.emptyText}</p>
      </div>
    {/if}
  </div>
</section>

<style>
  .comment-section {
    max-width: 800px;
    margin: var(--spacing-xl) auto;
    padding: 0 var(--spacing-md);
  }

  .section-header {
    text-align: center;
    margin-bottom: var(--spacing-md);
  }

  .section-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin: 0 0 4px;
  }

  .section-subtitle {
    font-size: 11px;
    color: var(--color-text-light);
    font-style: italic;
    margin: 0;
  }

  .comment-list {
    margin-top: var(--spacing-md);
  }

  .comment-list-header {
    margin-bottom: var(--spacing-md);
    padding-bottom: var(--spacing-xs);
    border-bottom: 2px solid var(--color-divider);
  }

  .comment-count {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    margin: 0;

    .count-number {
      color: var(--color-pink);
      font-size: var(--font-size-lg);
    }
  }

  .comment-list-body {
    position: relative;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-xl) var(--spacing-lg);
    text-align: center;
  }

  .empty-icon {
    font-size: 48px;
    opacity: 0.3;
    margin-bottom: var(--spacing-sm);
    animation: float 3s ease-in-out infinite;
  }

  @keyframes float {
    0%, 100% { transform: translateY(0); }
    50% { transform: translateY(-10px); }
  }

  .empty-text {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-style: italic;
    margin: 0;
  }

  @media (max-width: 640px) {
    .comment-section {
      padding: 0 var(--spacing-md);
    }
  }
</style>
