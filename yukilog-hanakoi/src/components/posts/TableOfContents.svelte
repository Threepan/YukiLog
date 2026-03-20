<script lang="ts">
  import { onMount } from 'svelte';
  import { contentConfig } from '$lib/config';

  interface Heading {
    id: string;
    text: string;
    level: number;
  }

  let { headings }: { headings: Heading[] } = $props();

  const tocConfig = contentConfig.components.tableOfContents;
  const minLevel = Math.min(...headings.map((h) => h.level));

  let activeIndex = $state(-1);
  let indicatorStyle = $state('opacity: 0');
  let scrollAreaEl: HTMLDivElement | undefined = $state();

  onMount(() => {
    const tocItems = document.querySelectorAll<HTMLLIElement>('.toc-item');
    const headingEls: (HTMLElement | null)[] = headings.map(
      (h) => document.getElementById(h.id)
    );

    function setActive(index: number) {
      if (index === activeIndex) return;
      activeIndex = index;

      if (index >= 0 && index < tocItems.length && scrollAreaEl) {
        const activeItem = tocItems[index];
        const link = activeItem.querySelector('.toc-link') as HTMLElement;
        if (link) {
          const scrollRect = scrollAreaEl.getBoundingClientRect();
          const linkRect = link.getBoundingClientRect();
          const top = linkRect.top - scrollRect.top + scrollAreaEl.scrollTop;
          const height = link.offsetHeight;
          indicatorStyle = `transform: translateY(${top}px); height: ${height}px; opacity: 1`;

          // 自动滚动 TOC 让高亮项保持可见
          const areaHeight = scrollAreaEl.clientHeight;
          const itemTop = activeItem.offsetTop - scrollAreaEl.offsetTop;
          const itemBottom = itemTop + activeItem.offsetHeight;
          const scrollTop = scrollAreaEl.scrollTop;
          const visibleTop = scrollTop + areaHeight * 0.15;
          const visibleBottom = scrollTop + areaHeight * 0.85;

          if (itemTop < visibleTop) {
            scrollAreaEl.scrollTo({ top: Math.max(0, itemTop - areaHeight * 0.2), behavior: 'smooth' });
          } else if (itemBottom > visibleBottom) {
            scrollAreaEl.scrollTo({ top: itemBottom - areaHeight * 0.8, behavior: 'smooth' });
          }
        }
      } else {
        indicatorStyle = 'opacity: 0';
      }
    }

    // IntersectionObserver
    const observer = new IntersectionObserver(
      (entries) => {
        let bestIndex = activeIndex;
        let bestTop = Infinity;

        for (const entry of entries) {
          if (entry.isIntersecting) {
            const idx = headingEls.indexOf(entry.target as HTMLElement);
            if (idx !== -1) {
              const top = entry.boundingClientRect.top;
              if (top < bestTop) {
                bestTop = top;
                bestIndex = idx;
              }
            }
          }
        }

        if (bestIndex === activeIndex) {
          const scrollY = window.scrollY + window.innerHeight * 0.25;
          for (let i = headingEls.length - 1; i >= 0; i--) {
            const el = headingEls[i];
            if (el && el.offsetTop <= scrollY) {
              bestIndex = i;
              break;
            }
          }
        }

        setActive(bestIndex);
      },
      { rootMargin: '-80px 0px -40% 0px', threshold: 0 }
    );

    headingEls.forEach((el) => { if (el) observer.observe(el); });

    // 滚动事件补充
    let ticking = false;
    function onScroll() {
      if (ticking) return;
      ticking = true;
      requestAnimationFrame(() => {
        const scrollY = window.scrollY + window.innerHeight * 0.25;
        let best = 0;
        for (let i = headingEls.length - 1; i >= 0; i--) {
          const el = headingEls[i];
          if (el && el.offsetTop <= scrollY) {
            best = i;
            break;
          }
        }
        setActive(best);
        ticking = false;
      });
    }

    window.addEventListener('scroll', onScroll, { passive: true });

    // 初始化
    requestAnimationFrame(() => {
      const scrollY = window.scrollY + window.innerHeight * 0.25;
      let best = 0;
      for (let i = headingEls.length - 1; i >= 0; i--) {
        const el = headingEls[i];
        if (el && el.offsetTop <= scrollY) {
          best = i;
          break;
        }
      }
      setActive(best);
    });

    return () => {
      observer.disconnect();
      window.removeEventListener('scroll', onScroll);
    };
  });

  function handleClick(e: MouseEvent, index: number) {
    e.preventDefault();
    const el = document.getElementById(headings[index].id);
    if (el) {
      const top = el.offsetTop - 96;
      window.scrollTo({ top, behavior: 'smooth' });
    }
  }
</script>

<nav class="toc-container" aria-label="文章目录">
  <div class="toc-title">{tocConfig.title}</div>
  <div class="toc-scroll-area" bind:this={scrollAreaEl}>
    <ul class="toc-list">
      {#each headings as h, i}
        <li
          class="toc-item"
          class:active={activeIndex === i}
          style="--indent: {h.level - minLevel}"
        >
          <a
            href="#{h.id}"
            class="toc-link"
            onclick={(e) => handleClick(e, i)}
          >
            {h.text}
          </a>
        </li>
      {/each}
    </ul>
    <div class="toc-indicator" aria-hidden="true" style={indicatorStyle}></div>
  </div>
</nav>

<style>
  .toc-container {
    font-size: var(--font-size-sm);
  }

  .toc-title {
    font-size: var(--font-size-sm);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text-light);
    margin-bottom: var(--spacing-sm);
    padding-left: 14px;
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }

  .toc-scroll-area {
    position: relative;
    max-height: calc(100vh - 200px);
    overflow-y: auto;
    scrollbar-width: none;
    padding: var(--spacing-xs) 0;

    &::-webkit-scrollbar { display: none; }
  }

  .toc-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .toc-item {
    padding-left: calc(14px + var(--indent, 0) * 12px);
    transition: all 0.5s cubic-bezier(0.22, 0.61, 0.36, 1);

    &.active .toc-link {
      color: color-mix(in srgb, var(--color-blue) 85%, var(--color-black, #000));
      font-weight: var(--font-weight-semibold);
      transform: scale(1.2) translateX(-6px);
      transform-origin: left center;
    }

    &:not(.active) .toc-link {
      color: var(--color-text-muted);
    }
  }

  .toc-link {
    display: block;
    padding: 4px 0;
    line-height: 1.5;
    text-decoration: none;
    transition: color 0.5s cubic-bezier(0.22, 0.61, 0.36, 1), transform 0.5s cubic-bezier(0.22, 0.61, 0.36, 1);
    white-space: normal;
    overflow-wrap: break-word;

    &:hover {
      color: color-mix(in srgb, var(--color-blue) 80%, var(--color-black, #000)) !important;
    }

    &::after {
      content: none !important;
    }
  }

  .toc-indicator {
    position: absolute;
    left: 0;
    top: 0;
    width: 2.5px;
    height: 20px;
    border-radius: 2px;
    background: linear-gradient(to bottom, var(--color-pink), var(--color-blue));
    opacity: 0;
    transition: transform 0.5s cubic-bezier(0.22, 0.61, 0.36, 1), height 0.5s cubic-bezier(0.22, 0.61, 0.36, 1), opacity 0.5s cubic-bezier(0.22, 0.61, 0.36, 1);
    pointer-events: none;
  }
</style>
