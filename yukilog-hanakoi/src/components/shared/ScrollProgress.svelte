<script lang="ts">
  import { onMount } from 'svelte';
  import { svgIcons } from '$lib/svg-icons';
  import { contentConfig } from '$lib/config';

  const scrollConfig = contentConfig.components.scrollProgress;

  let barEl: HTMLElement | undefined = $state();
  let btnEl: HTMLElement | undefined = $state();
  let percentNode: HTMLElement | undefined = $state();
  let visible = $state(false);

  onMount(() => {
    if (!barEl || !btnEl || !percentNode) return;

    let ticking = false;

    function updateProgress() {
      const scrollTop = window.scrollY;
      const docHeight = document.documentElement.scrollHeight - window.innerHeight;

      if (docHeight <= 0) {
        barEl!.style.setProperty('--p', '0');
        percentNode!.textContent = '0%';
        visible = false;
        ticking = false;
        return;
      }

      const progress = Math.min(scrollTop / docHeight, 1);
      const percent = Math.round(progress * 100);

      barEl!.style.setProperty('--p', String(progress));
      percentNode!.textContent = `${percent}%`;
      visible = scrollTop > 200;

      ticking = false;
    }

    const onScroll = () => {
      if (!ticking) {
        requestAnimationFrame(updateProgress);
        ticking = true;
      }
    };

    const onClick = () => {
      window.scrollTo({ top: 0, behavior: 'smooth' });
    };

    window.addEventListener('scroll', onScroll, { passive: true });
    btnEl.addEventListener('click', onClick);
    updateProgress();

    return () => {
      window.removeEventListener('scroll', onScroll);
      btnEl!.removeEventListener('click', onClick);
    };
  });
</script>

<!-- 顶部进度条 -->
<div class="scroll-progress-bar" bind:this={barEl} aria-hidden="true"></div>

<!-- 回到顶部按钮 -->
<button
  class="back-to-top"
  class:visible
  bind:this={btnEl}
  aria-label={scrollConfig.backToTop}
  title={scrollConfig.backToTop}
>
  <span class="btt-icon">{@html svgIcons.arrowUp}</span>
  <span class="btt-percent" bind:this={percentNode}>0%</span>
</button>

<style>
  .scroll-progress-bar {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 3px;
    background: transparent;
    z-index: 201;
    pointer-events: none;

    &::after {
      content: "";
      display: block;
      width: 100%;
      height: 100%;
      background: linear-gradient(
        90deg,
        transparent,
        var(--color-pink) 20%,
        var(--color-pink) 80%,
        transparent
      );
      transform: scaleX(var(--p, 0));
      transform-origin: center;
    }
  }

  .back-to-top {
    position: fixed;
    bottom: var(--spacing-xl);
    left: var(--spacing-xl);
    z-index: var(--z-modal);

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 2px;

    width: 48px;
    height: 48px;
    padding: 0;

    background: var(--white-alpha-85);
    backdrop-filter: blur(12px);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    box-shadow: var(--shadow-pink);

    color: var(--color-text-light);
    cursor: pointer;

    opacity: 0;
    transform: translateY(20px);
    pointer-events: none;
    transition:
      opacity 300ms var(--ease-gentle),
      transform 300ms var(--ease-gentle),
      box-shadow 200ms var(--ease-gentle),
      border-color 200ms var(--ease-gentle);

    &.visible {
      opacity: 1;
      transform: translateY(0);
      pointer-events: auto;
    }

    &:hover {
      box-shadow: var(--shadow-pink-offset-hover);
      border-color: var(--color-pink);
      color: var(--color-pink);
    }

    &:active {
      transform: scale(0.95);
    }
  }

  .btt-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    line-height: 0;

    :global(svg) {
      width: 16px;
      height: 16px;
    }
  }

  .btt-percent {
    font-size: 10px;
    font-weight: var(--font-weight-medium);
    line-height: 1;
    letter-spacing: -0.02em;
  }
</style>
