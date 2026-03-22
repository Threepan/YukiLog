<script lang="ts">
  import { contentConfig } from '$lib/config';
  import { commentsApi } from '$lib/api';
  import { page } from '$app/state';

  const cc = contentConfig.components.comments;

  let {
    commentId,
    commentNick,
    onCancel,
    onSubmitted,
  }: {
    commentId: number;
    commentNick: string;
    onCancel: () => void;
    onSubmitted: () => void;
  } = $props();

  let nickname = $state('');
  let email = $state('');
  let website = $state('');
  let content = $state('');
  let isSubmitting = $state(false);

  const charCount = $derived(content.length);
  let storageKey = $derived(`comment-reply-${commentId}`);

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
      }
    } catch { /* ignore */ }
  }

  function saveToStorage() {
    if (typeof window !== 'undefined') {
      localStorage.setItem(storageKey, JSON.stringify({ nickname, email, website, content }));
    }
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
        parent_id: commentId,
        website: website || undefined,
      });

      // 清理
      nickname = '';
      email = '';
      website = '';
      content = '';
      if (typeof window !== 'undefined') localStorage.removeItem(storageKey);
      alert('✅ 回复已提交！\n\n审核通过后将显示在评论区。感谢你的参与！');
      onSubmitted();
    } catch (err: any) {
      console.error('提交回复失败：', err);
      alert(`❌ 提交失败\n\n${err.message || '网络错误，请稍后重试'}`);
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="comment-form reply-form">
  <div class="reply-target">
    <span class="reply-icon">{cc.replyIcon}</span>
    回复 <strong>@{commentNick}</strong>
  </div>

  <form class="form-content" onsubmit={handleSubmit} oninput={saveToStorage}>
    <div class="form-row">
      <div class="form-field">
        <label for="reply-nickname">昵称 *</label>
        <input id="reply-nickname" type="text" bind:value={nickname} placeholder={cc.form.nickPlaceholder} required maxlength={20} />
      </div>
      <div class="form-field">
        <label for="reply-email">邮箱 *</label>
        <input id="reply-email" type="email" bind:value={email} placeholder={cc.form.emailPlaceholder} required />
      </div>
    </div>

    <div class="form-field">
      <label for="reply-website">{cc.form.websiteLabel}</label>
      <input id="reply-website" type="url" bind:value={website} placeholder={cc.form.websitePlaceholder} />
    </div>

    <div class="form-field">
      <label for="reply-content">回复内容 *</label>
      <textarea id="reply-content" bind:value={content} placeholder="支持 Markdown 格式：**粗体** *斜体* [链接](url)..." required rows={4} maxlength={500}></textarea>
      <div class="char-count"><span class="current">{charCount}</span> / 500</div>
    </div>

    <div class="form-actions">
      <button type="button" class="btn-cancel" onclick={onCancel}>{cc.cancelText}</button>
      <button type="submit" class="btn-submit" disabled={isSubmitting}>
        {#if isSubmitting}
          {cc.submitLoadingText}
        {:else}
          {cc.replyLabel}
        {/if}
      </button>
    </div>

    <div class="form-hint">{cc.form.hint}</div>
  </form>
</div>

<style>
  .comment-form.reply-form {
    background: linear-gradient(135deg, rgba(126, 182, 217, 0.05), rgba(232, 164, 180, 0.03));
    border: 1px solid var(--color-blue-l24);
    border-radius: 8px;
    padding: var(--spacing-sm) var(--spacing-md);
    box-shadow: 0 2px 8px rgba(126, 182, 217, 0.12);
    margin-top: var(--spacing-sm);
    animation: slideDown 0.2s var(--ease-gentle);
  }

  @keyframes slideDown {
    from { opacity: 0; transform: translateY(-8px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .reply-target {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    margin-bottom: var(--spacing-xs);
    padding: 5px var(--spacing-xs);
    background: rgba(126, 182, 217, 0.1);
    border-left: 3px solid var(--color-blue);
    border-radius: 4px;
    font-size: 11px;
    font-weight: var(--font-weight-medium);
    color: var(--color-text-light);

    strong {
      color: var(--color-blue);
      font-weight: var(--font-weight-semibold);
    }
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--spacing-xs);
    margin-bottom: var(--spacing-xs);
  }

  .form-field {
    position: relative;
    margin-bottom: var(--spacing-xs);

    label {
      display: block;
      margin-bottom: 4px;
      font-size: 11px;
      font-weight: var(--font-weight-semibold);
      color: var(--color-text);
    }

    input, textarea {
      width: 100%;
      padding: 7px var(--spacing-sm);
      background: var(--color-white);
      border: 1.5px solid var(--color-blue-l30);
      border-radius: 5px;
      font-size: 12px;
      font-family: inherit;
      color: var(--color-text);
      transition:
        border-color var(--transition-fast) var(--ease-gentle),
        box-shadow var(--transition-fast) var(--ease-gentle);

      &::placeholder { color: var(--color-text-muted-l10); font-size: 11px; }

      &:focus {
        outline: none;
        border-color: var(--color-blue);
        box-shadow: 0 0 0 2px rgba(126, 182, 217, 0.12);
        background: #FFFFFF;
      }
    }

    textarea {
      resize: vertical;
      line-height: 1.45;
      min-height: 64px;
    }
  }

  .char-count {
    text-align: right;
    margin-top: 4px;
    font-size: 10px;
    color: var(--color-text-muted);

    .current {
      color: var(--color-text-light);
      font-weight: var(--font-weight-medium);
    }
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 6px;
    margin-top: var(--spacing-sm);
  }

  .btn-cancel, .btn-submit {
    padding: 6px var(--spacing-md);
    border: none;
    border-radius: 5px;
    font-size: 12px;
    font-weight: var(--font-weight-semibold);
    font-family: inherit;
    cursor: pointer;
    transition:
      background var(--transition-fast) var(--ease-gentle),
      box-shadow var(--transition-fast) var(--ease-gentle),
      transform var(--transition-fast) var(--ease-gentle);
  }

  .btn-cancel {
    background: var(--color-white);
    color: var(--color-text-light);
    border: 1.5px solid var(--color-text-muted-l20);

    &:hover { background: var(--muted-alpha-05); border-color: var(--color-text-muted); }
  }

  .btn-submit {
    background: linear-gradient(135deg, var(--color-blue), var(--color-blue-d8));
    color: var(--color-on-primary);
    box-shadow: 0 2px 4px rgba(126, 182, 217, 0.3);

    &:hover { box-shadow: 0 3px 8px rgba(126, 182, 217, 0.4); transform: translateY(-1px); }
    &:active { transform: translateY(0); }
    &:disabled { opacity: 0.6; cursor: not-allowed; transform: none; }
  }

  .form-hint {
    margin-top: 6px;
    font-size: 10px;
    color: var(--color-text-muted);
    font-style: italic;
    text-align: center;
  }

  @media (max-width: 640px) {
    .form-row { grid-template-columns: 1fr; }
    .comment-form.reply-form { padding: var(--spacing-sm); }
  }
</style>
