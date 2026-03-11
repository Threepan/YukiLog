<script lang="ts">
  import { contentConfig, navItems } from '$lib/config';
  import { svgIcons } from '$lib/svg-icons';
  import NavItem from './NavItem.svelte';

  interface Props {
    stickyOnly?: boolean;
  }

  let { stickyOnly = false }: Props = $props();

  let navbar: HTMLElement | undefined = $state();
  let navbarFixed: HTMLElement | undefined = $state();

  const brandText = contentConfig.components.navbar.brand;
  const searchIcon = svgIcons.search;

  // 为固定元素的 SVG 生成唯一 ID，避免与 navbar 内部 SVG 的 mask ID 冲突
  function deduplicateSvgIds(svg: string, suffix: string): string {
    return svg
      .replace(/id='([^']+)'/g, (_, id: string) => `id='${id}${suffix}'`)
      .replace(/url\(#([^)]+)\)/g, (_, id: string) => `url(#${id}${suffix})`);
  }

  const searchIconFixed = deduplicateSvgIds(searchIcon, '_fixed');

  // 使用 $effect 替代 onMount，当 stickyOnly 变化时自动重新绑定事件
  // 这样从非首页导航回首页时，navbar 能正确切换行为模式
  $effect(() => {
    if (!navbar || !navbarFixed) return;

    // 先重置所有状态
    navbar.classList.remove('sticky', 'active');
    navbarFixed.classList.remove('hidden');

    if (stickyOnly) {
      navbar.classList.add('sticky');
      navbarFixed.classList.add('hidden');
      return;
    }

    const getThreshold = () => window.innerHeight;

    const onScroll = () => {
      const threshold = getThreshold();
      if (window.scrollY >= threshold - 50) {
        navbar!.classList.add('sticky');
        navbarFixed!.classList.add('hidden');
        navbar!.classList.remove('active');
      } else {
        navbar!.classList.remove('sticky');
        navbarFixed!.classList.remove('hidden');
      }
    };

    const onMouseMove = (e: MouseEvent) => {
      const threshold = getThreshold();
      if (e.clientY < 80 && window.scrollY < threshold) {
        navbar!.classList.add('active');
      } else if (window.scrollY < threshold) {
        navbar!.classList.remove('active');
      }
    };

    window.addEventListener('scroll', onScroll, { passive: true });
    document.addEventListener('mousemove', onMouseMove);
    onScroll();

    return () => {
      window.removeEventListener('scroll', onScroll);
      document.removeEventListener('mousemove', onMouseMove);
    };
  });
</script>

<nav class="navbar" bind:this={navbar}>
  <div class="navbar-container">
    <a href="/" class="navbar-inner-logo">{brandText}</a>

    <div class="navbar-items">
      {#each navItems as item, index}
        <NavItem {...item} {index} />
      {/each}
    </div>

    <div class="navbar-inner-actions">
      <button class="navbar-action-btn" aria-label="搜索">
        <span>{@html searchIcon}</span>
      </button>
    </div>
  </div>
</nav>

<div class="navbar-fixed-elements" bind:this={navbarFixed}>
  <a href="/" class="navbar-logo">{brandText}</a>

  <div class="navbar-actions">
    <button class="navbar-action-btn" id="search-toggle" aria-label="搜索">
      <span>{@html searchIconFixed}</span>
    </button>
  </div>
</div>

<style>
  .navbar {
    position: fixed;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 100%;
    height: 50px;
    background: var(--color-white);
    z-index: var(--z-nav);
    opacity: 0;
    pointer-events: none;
    transition: opacity 400ms var(--ease-gentle),
                width 500ms cubic-bezier(0.22, 0.61, 0.36, 1),
                border-radius 500ms cubic-bezier(0.22, 0.61, 0.36, 1),
                top 500ms cubic-bezier(0.22, 0.61, 0.36, 1),
                box-shadow 500ms var(--ease-gentle);

    &:global(.active) {
      opacity: 1;
      pointer-events: auto;
    }

    &:global(.sticky) {
      opacity: 1;
      pointer-events: auto;
      width: auto;
      top: 10px;
      border-radius: var(--radius-xl);
      box-shadow: var(--shadow-blue), var(--shadow-xs);
    }
  }

  .navbar-container {
    height: 100%;
    margin: 0 auto;
    padding: 0 var(--spacing-lg);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
  }

  .navbar-fixed-elements {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 50px;
    pointer-events: none;
    z-index: 101;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 var(--spacing-xl);
    transition: opacity 400ms var(--ease-gentle);

    &:global(.hidden) {
      opacity: 0;
      pointer-events: none;
    }
  }

  .navbar-logo {
    font-size: 1.5rem;
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    text-decoration: none;
    flex-shrink: 0;
    pointer-events: auto;
    white-space: nowrap;

    &:hover {
      color: var(--color-blue);
    }
  }

  .navbar-inner-logo {
    font-size: 1.5rem;
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
    text-decoration: none;
    flex-shrink: 0;
    white-space: nowrap;
    display: none;

    &:hover {
      color: var(--color-blue);
    }

    :global(.sticky) & {
      display: block;
    }
  }

  .navbar-inner-actions {
    display: none;
    align-items: center;
    gap: var(--spacing-sm);
    flex-shrink: 0;

    :global(.sticky) & {
      display: flex;
    }
  }

  .navbar-items {
    display: flex;
    align-items: center;
    gap: 4px;
    white-space: nowrap;
    flex-shrink: 0;
  }

  /* Nav item animation (children are NavItem components) */
  .navbar :global(.nav-item) {
    opacity: 0;
    transform: translateX(40px);
    transition: color 200ms cubic-bezier(0.22, 0.61, 0.36, 1);
  }

  :global(.active) .navbar-items :global(.nav-item),
  :global(.sticky) .navbar-items :global(.nav-item) {
    animation: nav-item-enter 600ms cubic-bezier(0.22, 0.61, 0.36, 1) forwards;
    transition: color 200ms cubic-bezier(0.22, 0.61, 0.36, 1);
  }

  :global(.active) .navbar-items :global(.nav-item:nth-child(1)),
  :global(.sticky) .navbar-items :global(.nav-item:nth-child(1)) { animation-delay: 0ms; }
  :global(.active) .navbar-items :global(.nav-item:nth-child(2)),
  :global(.sticky) .navbar-items :global(.nav-item:nth-child(2)) { animation-delay: 150ms; }
  :global(.active) .navbar-items :global(.nav-item:nth-child(3)),
  :global(.sticky) .navbar-items :global(.nav-item:nth-child(3)) { animation-delay: 300ms; }
  :global(.active) .navbar-items :global(.nav-item:nth-child(4)),
  :global(.sticky) .navbar-items :global(.nav-item:nth-child(4)) { animation-delay: 450ms; }
  :global(.active) .navbar-items :global(.nav-item:nth-child(5)),
  :global(.sticky) .navbar-items :global(.nav-item:nth-child(5)) { animation-delay: 600ms; }
  :global(.active) .navbar-items :global(.nav-item:nth-child(6)),
  :global(.sticky) .navbar-items :global(.nav-item:nth-child(6)) { animation-delay: 750ms; }
  :global(.active) .navbar-items :global(.nav-item:nth-child(7)),
  :global(.sticky) .navbar-items :global(.nav-item:nth-child(7)) { animation-delay: 900ms; }

  @keyframes nav-item-enter {
    from {
      opacity: 0;
      transform: translateX(40px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }

  .navbar-actions {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    flex-shrink: 0;
    pointer-events: auto;

    .navbar-action-btn {
      color: var(--color-text);

      &:hover {
        background: var(--white-alpha-15);
        color: var(--color-blue);
      }
    }
  }

  .navbar-action-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: none;
    color: var(--color-text);
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition: all var(--transition-fast) var(--ease-out);

    :global(svg) {
      width: 20px;
      height: 20px;
      fill: currentColor;
    }

    &:hover {
      background: var(--color-bg);
      color: var(--color-blue);
    }
  }

  @media (max-width: 968px) {
    .navbar-container,
    .navbar-fixed-elements {
      padding: 0 var(--spacing-md);
    }

    .navbar-items {
      gap: 2px;
      max-width: calc(100vw - 180px);
      overflow-x: auto;
      scrollbar-width: none;

      &::-webkit-scrollbar {
        display: none;
      }
    }

    .navbar-inner-logo {
      font-size: 1.2rem;
    }
  }

  @media (max-width: 640px) {
    .navbar-logo,
    .navbar-inner-logo {
      display: none;
    }

    .navbar-container {
      justify-content: center;
      gap: var(--spacing-xs);
    }

    .navbar-items {
      max-width: calc(100vw - 96px);
    }
  }
</style>
