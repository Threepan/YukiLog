<script lang="ts">
  import { contentConfig } from '$lib/config';

  interface Props {
    title: string;
    subtitle?: string;
    icon: string;
  }

  let { title, subtitle, icon }: Props = $props();

  const headerImageName = contentConfig.hero.headerGif;
  const heroBgSrc = `/images/header/${headerImageName}`;
</script>

<div class="page-hero">
  <div class="hero-bg">
    <img src={heroBgSrc} alt="" aria-hidden="true" />
  </div>
  <div class="hero-content">
    <span class="hero-icon">{@html icon}</span>
    <h1 class="hero-title">{title}</h1>
    {#if subtitle}
      <p class="hero-subtitle">{subtitle}</p>
    {/if}
  </div>
</div>

<style>
  .page-hero {
    position: relative;
    height: 40vh;
    min-height: 220px;
    max-height: 360px;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .hero-bg {
    position: absolute;
    inset: 0;
    z-index: 0;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
      display: block;
    }

    &::after {
      content: '';
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      height: 30%;
      background: linear-gradient(to bottom, transparent, var(--color-bg));
      pointer-events: none;
    }
  }

  .hero-content {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--spacing-sm);
    opacity: 0;
    animation: hero-enter 700ms var(--ease-gentle) 150ms forwards;
  }

  @keyframes hero-enter {
    from {
      opacity: 0;
      transform: translateY(12px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .hero-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 56px;
    height: 56px;
    background: var(--white-alpha-85);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-md);
    color: var(--color-blue);
  }

  .hero-icon :global(svg) {
    width: 28px;
    height: 28px;
    display: block;
  }

  .hero-title {
    font-size: var(--font-size-3xl);
    font-weight: var(--font-weight-semibold);
    color: var(--color-on-primary);
    margin: 0;
    text-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
    letter-spacing: 0.08em;
  }

  .hero-subtitle {
    font-size: var(--font-size-base);
    color: var(--white-alpha-85);
    margin: 0;
    text-shadow: 0 1px 4px rgba(0, 0, 0, 0.12);
    letter-spacing: 0.04em;
  }

  @media (max-width: 640px) {
    .page-hero {
      min-height: 180px;
    }

    .hero-icon {
      width: 44px;
      height: 44px;

      :global(svg) {
        width: 22px;
        height: 22px;
      }
    }

    .hero-title {
      font-size: var(--font-size-2xl);
    }
  }
</style>
