<script lang="ts">
  import { contentConfig, siteConfig } from '$lib/config';
  import { socialIcons, svgIcons } from '$lib/svg-icons';

  const resolvedSocialIcons = siteConfig.social.map((s) => ({
    ...s,
    svg: socialIcons[s.icon as keyof typeof socialIcons] ?? '',
  }));

  const chars = [...siteConfig.welcomeText];
  const quoteText = contentConfig.components.welcomeCard.quoteText;
</script>

<div class="welcome-card">
  <div class="welcome-title">
    {#each chars as char, i}
      <span
        class="welcome-char"
        style="--char-index: {i}; --char-delay: {i * 0.08}s"
      >
        {char}
      </span>
    {/each}
  </div>

  <div class="welcome-sub">
    <div class="welcome-spacer"></div>

    <div class="welcome-info-card">
      <div class="welcome-quote">
        <span class="quote-icon quote-open">{@html svgIcons.openingQuotationMark}</span>
        <span class="quote-text">{quoteText}</span>
        <span class="quote-icon quote-close">{@html svgIcons.closingQuotationMark}</span>
      </div>

      <div class="welcome-social">
        {#each resolvedSocialIcons as s}
          {#if s.icon === 'gmail'}
            <span
              class="social-icon-btn"
              title={s.name}
              style="--icon-color: {s.color}"
            >
              {@html s.svg}
            </span>
          {:else}
            <a
              href={s.url}
              target="_blank"
              rel="noopener noreferrer"
              class="social-icon-btn"
              title={s.name}
              style="--icon-color: {s.color}"
            >
              {@html s.svg}
            </a>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  .welcome-card {
    width: min(580px, 70vw);
    display: flex;
    flex-direction: column;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 10;
  }

  .welcome-title {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
    padding: var(--spacing-md) 0;
  }

  .welcome-char {
    display: inline-block;
    font-size: 3.0rem;
    font-weight: 900;
    color: var(--color-white);
    opacity: 0;
    transform: translateY(12px);
    animation: char-appear 0.5s cubic-bezier(0.22, 0.61, 0.36, 1) var(--char-delay) forwards;
    cursor: default;
    text-shadow: 0 2px 16px rgba(0, 0, 0, 0.5), 0 0 4px rgba(0, 0, 0, 0.3);
    -webkit-text-stroke: 0.5px rgba(255, 255, 255, 0.9);
  }

  @keyframes char-appear {
    from {
      opacity: 0;
      transform: translateY(12px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .welcome-sub {
    display: flex;
    flex-direction: column;
  }

  .welcome-spacer {
    height: 20%;
    min-height: 12px;
  }

  .welcome-info-card {
    background: rgba(0, 0, 0, 0.6);
    border-radius: var(--radius-lg);
    padding: var(--spacing-xs) var(--spacing-lg);
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .welcome-quote {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-sm);
    padding-top: var(--spacing-md);
    padding-bottom: var(--spacing-xs);
  }

  .quote-icon {
    width: 18px;
    height: 18px;
    flex-shrink: 0;

    :global(svg) {
      width: 100%;
      height: 100%;
    }
  }

  .quote-text {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-bold);
    color: rgba(255, 255, 255, 0.85);
    line-height: 1.6;
    text-align: center;
  }

  .welcome-social {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--spacing-md);
    padding-top: var(--spacing-sm);
  }

  .social-icon-btn {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    color: var(--icon-color);
    text-decoration: none;
    transition: all 0.3s cubic-bezier(0.22, 0.61, 0.36, 1);
    cursor: pointer;

    :global(svg) {
      width: 24px;
      height: 24px;
      fill: currentColor;
      transition: transform 0.3s cubic-bezier(0.22, 0.61, 0.36, 1);
    }

    &:hover {
      filter: brightness(1.3);
      transform: translateY(-3px);

      :global(svg) {
        transform: scale(1.15);
      }
    }
  }

  @media (max-width: 768px) {
    .welcome-title {
      padding: var(--spacing-md) 0;
    }

    .welcome-char {
      font-size: 2.2rem;
    }

    .welcome-info-card {
      padding: var(--spacing-xs) var(--spacing-md);
    }

    .quote-text {
      font-size: var(--font-size-sm);
    }

    .social-icon-btn {
      width: 32px;
      height: 32px;

      :global(svg) {
        width: 20px;
        height: 20px;
      }
    }
  }
</style>
