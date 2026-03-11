<script lang="ts">
  import { onMount } from 'svelte';
  import { contentConfig } from '$lib/config';
  import { svgIcons } from '$lib/svg-icons';
  import { linksApi } from '$lib/api';

  const applyConfig = contentConfig.components.applyLinkModal;

  let isOpen = $state(false);
  let isSubmitting = $state(false);
  let resultMessage = $state('');
  let resultType: 'success' | 'error' | '' = $state('');

  // Form fields
  let title = $state('');
  let url = $state('');
  let avatar = $state('');
  let description = $state('');

  function openModal() {
    isOpen = true;
    document.body.style.overflow = 'hidden';
  }

  function closeModal() {
    isOpen = false;
    document.body.style.overflow = '';
  }

  function handleOverlayClick(e: MouseEvent) {
    if (e.target === e.currentTarget) closeModal();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isOpen) closeModal();
  }

  async function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (isSubmitting) return;

    isSubmitting = true;
    resultMessage = '';
    resultType = '';

    try {
      await linksApi.submit({
        title,
        url,
        avatar: avatar || null,
        description: description || null,
      });
      resultMessage = '✨ 信已寄出，恋收到后会认真查看的';
      resultType = 'success';
      title = '';
      url = '';
      avatar = '';
      description = '';
      setTimeout(closeModal, 2000);
    } catch {
      resultMessage = '网络出了点问题，请稍后再试';
      resultType = 'error';
    } finally {
      isSubmitting = false;
    }
  }

  onMount(() => {
    document.addEventListener('keydown', handleKeydown);
    return () => document.removeEventListener('keydown', handleKeydown);
  });
</script>

<!-- 触发按钮 -->
<div class="apply-section" id="apply-section">
  <p class="apply-hint">{applyConfig.hint}</p>
  <button class="apply-btn" type="button" onclick={openModal}>
    <span>{@html svgIcons.envelope}</span>
    <span>{applyConfig.triggerButton}</span>
  </button>
</div>

<!-- Modal 遮罩 + 弹窗 -->
{#if isOpen}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" aria-hidden="false" onclick={handleOverlayClick} onkeydown={handleKeydown}>
    <div class="modal-card" role="dialog" aria-label="申请友链">
      <button class="modal-close" type="button" aria-label="关闭" onclick={closeModal}>
        {@html svgIcons.close}
      </button>

      <h3 class="modal-title">{applyConfig.title}</h3>
      <p class="modal-subtitle">{applyConfig.subtitle}</p>

      <form class="apply-form" onsubmit={handleSubmit}>
        <div class="form-group">
          <label for="link-title">站点名称 <span class="required">*</span></label>
          <input type="text" id="link-title" bind:value={title} required placeholder="你的站点叫什么？" maxlength={50} />
        </div>

        <div class="form-group">
          <label for="link-url">站点地址 <span class="required">*</span></label>
          <input type="url" id="link-url" bind:value={url} required placeholder="https://..." maxlength={200} />
        </div>

        <div class="form-group">
          <label for="link-avatar">头像地址</label>
          <input type="url" id="link-avatar" bind:value={avatar} placeholder="https://...（可选）" maxlength={300} />
        </div>

        <div class="form-group">
          <label for="link-desc">一句话介绍</label>
          <textarea id="link-desc" bind:value={description} placeholder="用一句话描述你的站点吧（可选）" maxlength={100} rows={2}></textarea>
        </div>

        <button type="submit" class="submit-btn" disabled={isSubmitting}>
          {#if isSubmitting}
            {applyConfig.submitLoadingText}
          {:else}
            {applyConfig.submitText}
          {/if}
        </button>

        {#if resultMessage}
          <p class="form-result" class:success={resultType === 'success'} class:error={resultType === 'error'}>
            {resultMessage}
          </p>
        {/if}
      </form>
    </div>
  </div>
{/if}

<style>
  /* 申请区域 */
  .apply-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-xxl) 0 var(--spacing-xl);
    opacity: 0;
    animation: fade-up 500ms var(--ease-gentle) 800ms forwards;
  }

  @keyframes fade-up {
    from {
      opacity: 0;
      transform: translateY(16px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .apply-hint {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin: 0;
  }

  .apply-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 28px;
    background: var(--color-white);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-pink);
    color: var(--color-text);
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-medium);
    cursor: pointer;
    transition:
      box-shadow var(--transition-base) var(--ease-gentle),
      border-color var(--transition-base) var(--ease-gentle),
      transform var(--transition-fast) var(--ease-gentle),
      color var(--transition-fast) var(--ease-gentle);

    :global(svg) {
      color: var(--color-pink);
      transition: transform var(--transition-fast) var(--ease-gentle);
    }

    &:hover {
      box-shadow: var(--shadow-pink-offset-hover);
      border-color: var(--color-pink);
      color: var(--color-pink);

      :global(svg) {
        transform: translateY(-2px);
      }
    }

    &:active {
      transform: scale(0.97);
    }
  }

  /* Modal 遮罩 */
  .modal-overlay {
    position: fixed;
    inset: 0;
    z-index: var(--z-modal);
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--overlay-bg);
    backdrop-filter: blur(4px);
    animation: overlay-in 300ms var(--ease-gentle) forwards;
  }

  @keyframes overlay-in {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  /* Modal 卡片 */
  .modal-card {
    position: relative;
    width: 90%;
    max-width: 440px;
    max-height: 90vh;
    overflow-y: auto;
    background: var(--color-white);
    border-radius: var(--radius-xl);
    box-shadow: var(--shadow-pink), 0 20px 60px rgba(44, 62, 80, 0.1);
    padding: var(--spacing-xl) var(--spacing-xl) var(--spacing-lg);
    animation: card-in 300ms var(--ease-gentle) forwards;
  }

  @keyframes card-in {
    from {
      transform: translateY(20px) scale(0.97);
      opacity: 0;
    }
    to {
      transform: translateY(0) scale(1);
      opacity: 1;
    }
  }

  .modal-close {
    position: absolute;
    top: var(--spacing-sm);
    right: var(--spacing-sm);
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--color-text-muted);
    cursor: pointer;
    border-radius: 50%;
    transition:
      background var(--transition-fast) var(--ease-gentle),
      color var(--transition-fast) var(--ease-gentle);

    &:hover {
      background: var(--color-bg);
      color: var(--color-text);
    }
  }

  .modal-title {
    font-size: var(--font-size-xl);
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    margin: 0;
  }

  .modal-subtitle {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin: var(--spacing-xs) 0 var(--spacing-lg);
  }

  /* 表单 */
  .apply-form {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;

    label {
      font-size: var(--font-size-sm);
      font-weight: var(--font-weight-medium);
      color: var(--color-text);

      .required {
        color: var(--color-pink);
      }
    }

    input,
    textarea {
      width: 100%;
      padding: 10px 14px;
      border: 1px solid var(--color-border);
      border-radius: var(--radius-sm);
      font-size: var(--font-size-sm);
      font-family: var(--font-family-base);
      color: var(--color-text);
      background: var(--color-white);
      transition:
        border-color var(--transition-fast) var(--ease-gentle),
        box-shadow var(--transition-fast) var(--ease-gentle);
      outline: none;
      box-sizing: border-box;

      &::placeholder {
        color: var(--color-text-muted);
      }

      &:focus {
        border-color: var(--color-pink);
        box-shadow: 0 0 0 3px rgba(232, 164, 180, 0.1);
      }
    }

    textarea {
      resize: vertical;
      min-height: 60px;
    }
  }

  .submit-btn {
    width: 100%;
    padding: 12px 0;
    border: none;
    border-radius: var(--radius-sm);
    background: linear-gradient(135deg, var(--color-pink), var(--color-pink-l8));
    color: var(--color-white);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    cursor: pointer;
    transition:
      opacity var(--transition-fast) var(--ease-gentle),
      transform var(--transition-fast) var(--ease-gentle);

    &:hover {
      opacity: 0.9;
    }

    &:active {
      transform: scale(0.98);
    }

    &:disabled {
      opacity: 0.6;
      cursor: not-allowed;
    }
  }

  .form-result {
    font-size: var(--font-size-sm);
    text-align: center;
    margin: 0;
    min-height: 1.4em;

    &.success {
      color: var(--color-success);
    }

    &.error {
      color: var(--color-error);
    }
  }
</style>
