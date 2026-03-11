<script lang="ts">
  import { contentConfig } from '$lib/config';
  import { commentsApi } from '$lib/api';
  import { page } from '$app/state';

  const cc = contentConfig.components.comments;

  let nickname = $state('');
  let email = $state('');
  let website = $state('');
  let content = $state('');
  let isSubmitting = $state(false);
  let isExpanded = $state(false);

  const charCount = $derived(content.length);
  const storageKey = 'comment-form-main';

  // 从 localStorage 恢复
  if (typeof window !== 'undefined') {
    try {
      const saved = localStorage.getItem(storageKey);
      if (saved) {
        const data = JSON.parse(saved);
        nickname = data.nickname || '';
        email = data.email || '';
        website = data.website || '';
        content = data.content || '';
        if (data.content) isExpanded = true;
      }
    } catch { /* ignore */ }
  }

  function saveToStorage() {
    if (typeof window !== 'undefined') {
      localStorage.setItem(storageKey, JSON.stringify({ nickname, email, website, content }));
    }
  }

  function expandForm() {
    isExpanded = true;
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (isSubmitting) return;
    isSubmitting = true;

    try {
      const postSlug = page.url.pathname.split('/').pop() || '';
      await commentsApi.submit(postSlug, {
        nickname,
        email,
        content,
        website: website || undefined,
      });

      nickname = '';
      email = '';
      website = '';
      content = '';
      isExpanded = false;
      if (typeof window !== 'undefined') localStorage.removeItem(storageKey);
      alert('✅ 评论已提交！\n\n审核通过后将显示在评论区。感谢你的参与！');
    } catch (err: any) {
      console.error('提交评论失败：', err);
      alert(`❌ 提交失败\n\n${err.message || '网络错误，请稍后重试'}`);
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="comment-form main-form" class:expanded={isExpanded}>
  <form class="form-content" onsubmit={handleSubmit} oninput={saveToStorage}>
    <!-- 主输入区 - 始终可见 -->
    <div class="form-field main-input">
      <textarea
        name="content"
        bind:value={content}
        placeholder={cc.form.placeholder}
        required
        rows={4}
        maxlength={500}
        onfocus={expandForm}
        oninput={expandForm}
      ></textarea>
      <div class="char-count"><span class="current">{charCount}</span> / 500</div>
    </div>

    <!-- 扩展字段区 -->
    <div class="expandable-fields" class:expanded={isExpanded}>
      <div class="form-row">
        <div class="form-field">
          <label>{cc.form.nickLabel}</label>
          <input type="text" bind:value={nickname} placeholder={cc.form.nickPlaceholder} required maxlength={20} />
        </div>
        <div class="form-field">
          <label>{cc.form.emailLabel}</label>
          <input type="email" bind:value={email} placeholder={cc.form.emailPlaceholder} required />
        </div>
      </div>

      <div class="form-field">
        <label>{cc.form.websiteLabel}</label>
        <input type="url" bind:value={website} placeholder={cc.form.websitePlaceholder} />
      </div>

      <div class="form-actions">
        <button type="submit" class="btn-submit" disabled={isSubmitting}>
          {#if isSubmitting}
            {cc.submitLoadingText}
          {:else}
            {cc.submitText}
          {/if}
        </button>
      </div>

      <div class="form-hint">{cc.form.hint}</div>
    </div>
  </form>
</div>

<style>
  .comment-form {
    background: var(--color-white);
    border: 1px solid rgba(232, 164, 180, 0.25);
    border-radius: var(--radius-md);
    padding: var(--spacing-md) var(--spacing-lg);
    box-shadow: 0 1px 3px rgba(44, 62, 80, 0.04);
    transition: box-shadow 300ms var(--ease-gentle);

    &.main-form { margin-bottom: var(--spacing-lg); }
    &.expanded { box-shadow: 0 2px 8px rgba(126, 182, 217, 0.08); }
  }

  .main-input {
    margin-bottom: 0 !important;

    textarea {
      border-radius: 8px;
      min-height: 64px;
      font-size: var(--font-size-sm);
      &::placeholder { color: var(--color-text-muted); font-size: var(--font-size-sm); }
    }
  }

  .expandable-fields {
    max-height: 0;
    opacity: 0;
    overflow: hidden;
    transition:
      max-height 300ms var(--ease-gentle),
      opacity 250ms var(--ease-gentle),
      margin-top 300ms var(--ease-gentle);

    &.expanded {
      max-height: 500px;
      opacity: 1;
      margin-top: var(--spacing-md);
    }
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--spacing-sm);
    margin-bottom: var(--spacing-sm);
  }

  .form-field {
    position: relative;
    margin-bottom: var(--spacing-sm);

    label {
      display: block;
      margin-bottom: 4px;
      font-size: var(--font-size-xs);
      font-weight: var(--font-weight-medium);
      color: var(--color-text-light);
    }

    input, textarea {
      width: 100%;
      padding: 8px var(--spacing-sm);
      background: var(--color-bg);
      border: 1.5px solid var(--color-pink-l28);
      border-radius: 6px;
      font-size: var(--font-size-sm);
      font-family: inherit;
      color: var(--color-text);
      transition:
        border-color var(--transition-fast) var(--ease-gentle),
        background var(--transition-fast) var(--ease-gentle);

      &::placeholder { color: var(--color-text-muted); }
      &:focus { outline: none; border-color: var(--color-pink); background: var(--color-white); }
    }

    textarea {
      resize: vertical;
      line-height: var(--line-height-base);
      min-height: 70px;
    }
  }

  .char-count {
    position: absolute;
    right: 8px;
    bottom: -18px;
    font-size: 11px;
    color: var(--color-text-muted);

    .current {
      color: var(--color-text-light);
      font-weight: var(--font-weight-medium);
    }
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--spacing-xs);
    margin-top: var(--spacing-sm);
  }

  .btn-submit {
    padding: 7px var(--spacing-md);
    border: none;
    border-radius: 6px;
    font-size: var(--font-size-xs);
    font-weight: var(--font-weight-medium);
    font-family: inherit;
    cursor: pointer;
    background: linear-gradient(135deg, var(--color-pink), var(--color-pink-l8));
    color: white;
    box-shadow: 0 1px 3px rgba(232, 164, 180, 0.3);
    transition:
      background var(--transition-fast) var(--ease-gentle),
      box-shadow var(--transition-fast) var(--ease-gentle),
      transform var(--transition-fast) var(--ease-gentle);

    &:hover { box-shadow: 0 2px 6px rgba(232, 164, 180, 0.4); transform: translateY(-1px); }
    &:active { transform: translateY(0); }
    &:disabled { opacity: 0.6; cursor: not-allowed; transform: none; }
  }

  .form-hint {
    margin-top: var(--spacing-sm);
    font-size: 11px;
    color: var(--color-text-muted);
    font-style: italic;
    text-align: center;
  }

  @media (max-width: 640px) {
    .form-row { grid-template-columns: 1fr; }
  }
</style>
