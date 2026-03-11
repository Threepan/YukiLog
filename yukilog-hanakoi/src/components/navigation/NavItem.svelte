<script lang="ts">
  import { navIcons } from '$lib/svg-icons';
  import type { NavIconName } from '$types';
  import { page } from '$app/state';

  interface Props {
    label: string;
    href: string;
    icon?: NavIconName;
    index: number;
  }

  let { label, href, icon, index }: Props = $props();

  let isActive = $derived(page.url.pathname === href);
  let iconSvg = $derived(icon ? navIcons[icon] : null);
</script>

<a
  {href}
  class="nav-item"
  class:active={isActive}
  style="--index: {index}"
>
  {#if iconSvg}
    <span class="nav-icon">{@html iconSvg}</span>
  {/if}
  <span class="nav-label">{label}</span>
</a>

<style>
  .nav-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    padding: var(--spacing-sm) var(--spacing-md);
    color: var(--color-text);
    font-weight: var(--font-weight-medium);
    font-size: var(--font-size-base);
    text-decoration: none;
    white-space: nowrap;
    transition: color var(--transition-base) var(--ease-gentle);

    &::after {
      content: '';
      position: absolute;
      bottom: 0;
      left: 15%;
      width: 0;
      height: 2px;
      background: currentColor;
      transition: width var(--transition-base) var(--ease-gentle);
    }

    &:hover {
      color: var(--color-blue);

      &::after {
        width: 70%;
      }
    }

    &.active {
      color: var(--color-pink);

      &::after {
        width: 70%;
      }
    }
  }

  .nav-icon {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;

    :global(svg) {
      width: 100%;
      height: 100%;
      fill: currentColor;
    }
  }

  .nav-label {
    position: relative;
    line-height: 1;
  }

  @media (max-width: 968px) {
    .nav-item {
      padding: var(--spacing-xs);
      gap: 0;

      &::after {
        left: 20%;
        width: 0;
      }

      &:hover::after,
      &.active::after {
        width: 60%;
      }
    }

    .nav-label {
      display: none;
    }
  }
</style>
