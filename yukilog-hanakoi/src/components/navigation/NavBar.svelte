<script lang="ts">
  import { contentConfig, navItems } from '$lib/config';
  import { svgIcons } from '$lib/svg-icons';
  import NavItem from './NavItem.svelte';
  import { page } from '$app/state';

  interface Props {
    stickyOnly?: boolean;
  }

  let { stickyOnly = false }: Props = $props();

  let navbar: HTMLElement | undefined = $state();
  let navbarFixed: HTMLElement | undefined = $state();
  let menuOpen = $state(false);

  const brandText = contentConfig.components.navbar.brand;
  const searchIcon = svgIcons.search;

  // 为固定元素的 SVG 生成唯一 ID，避免与 navbar 内部 SVG 的 mask ID 冲突
  function deduplicateSvgIds(svg: string, suffix: string): string {
    return svg
      .replace(/id='([^']+)'/g, (_, id: string) => `id='${id}${suffix}'`)
      .replace(/url\(#([^)]+)\)/g, (_, id: string) => `url(#${id}${suffix})`);
  }

  const searchIconFixed = deduplicateSvgIds(searchIcon, '_fixed');

  function toggleMenu() { menuOpen = !menuOpen; }
  function closeMenu() { menuOpen = false; }

  // 路由切换时关闭菜单
  $effect(() => {
    page.url.pathname;
    menuOpen = false;
  });

  // 菜单打开时锁定 body 滚动
  $effect(() => {
    if (typeof document !== 'undefined') {
      document.body.style.overflow = menuOpen ? 'hidden' : '';
    }
  });

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
      <button class="navbar-action-btn navbar-hamburger" aria-label="菜单" onclick={toggleMenu}>
        <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
          {#if menuOpen}
            <path d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" />
          {:else}
            <rect x="2" y="4" width="16" height="2" rx="1" />
            <rect x="2" y="9" width="16" height="2" rx="1" />
            <rect x="2" y="14" width="16" height="2" rx="1" />
          {/if}
        </svg>
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
    <button class="navbar-action-btn navbar-hamburger" aria-label="菜单" onclick={toggleMenu}>
      <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
        {#if menuOpen}
          <path d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" />
        {:else}
          <rect x="2" y="4" width="16" height="2" rx="1" />
          <rect x="2" y="9" width="16" height="2" rx="1" />
          <rect x="2" y="14" width="16" height="2" rx="1" />
        {/if}
      </svg>
    </button>
  </div>
</div>

<!-- 移动端菜单抽屉 -->
{#if menuOpen}
  <div
    class="mobile-menu-overlay"
    role="dialog"
    aria-modal="true"
    aria-label="导航菜单"
    onclick={closeMenu}
  >
    <div class="mobile-menu" onclick={(e) => e.stopPropagation()}>
      <div class="mobile-menu-header">
        <span class="mobile-menu-brand">{brandText}</span>
        <button class="navbar-action-btn" aria-label="关闭菜单" onclick={closeMenu}>
          <svg width="20" height="20" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
            <path d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" />
          </svg>
        </button>
      </div>
      <nav class="mobile-menu-nav">
        {#each navItems as item}
          {@const isActive = page.url.pathname === item.href}
          <a
            href={item.href}
            class="mobile-nav-item"
            class:active={isActive}
            onclick={closeMenu}
          >{item.label}</a>
        {/each}
      </nav>
    </div>
  </div>
{/if}

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

  .navbar-hamburger {
    display: none;
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
      justify-content: space-between;
      gap: var(--spacing-xs);
    }

    .navbar-items {
      display: none;
    }

    .navbar-hamburger {
      display: flex;
    }
  }

  /* ============================== */
  /* 移动端菜单抽屉 */
  /* ============================== */
  :global(.mobile-menu-overlay) {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    z-index: 200;
    display: flex;
    align-items: flex-end;
    backdrop-filter: blur(6px);
    -webkit-backdrop-filter: blur(6px);
  }

  :global(.mobile-menu) {
    background: var(--color-white);
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
    width: 100%;
    padding: var(--spacing-md) var(--spacing-lg) calc(var(--spacing-xl) + env(safe-area-inset-bottom));
    box-shadow: var(--shadow-blue);
    animation: mobile-menu-up 280ms cubic-bezier(0.22, 0.61, 0.36, 1) forwards;
  }

  @keyframes mobile-menu-up {
    from { transform: translateY(100%); }
    to   { transform: translateY(0); }
  }

  :global(.mobile-menu-header) {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--spacing-md);
    padding-bottom: var(--spacing-sm);
    border-bottom: 1px solid var(--color-border);
  }

  :global(.mobile-menu-brand) {
    font-size: 1.25rem;
    font-weight: var(--font-weight-bold);
    color: var(--color-text);
  }

  :global(.mobile-menu-nav) {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  :global(.mobile-nav-item) {
    display: block;
    padding: var(--spacing-sm) var(--spacing-md);
    color: var(--color-text);
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-medium);
    text-decoration: none;
    border-radius: var(--radius-md);
    transition: background var(--transition-fast) var(--ease-gentle),
                color var(--transition-fast) var(--ease-gentle);

    &:hover {
      background: var(--color-bg);
      color: var(--color-blue);
    }

    &.active {
      color: var(--color-pink);
      background: var(--color-bg);
    }
  }
</style>
