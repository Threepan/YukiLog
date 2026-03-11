<script lang="ts">
  import { siteConfig } from '$lib/config';

  const currentYear = new Date().getFullYear();
</script>

<footer class="footer">
  <div class="footer-container">
    <div class="footer-content">
      <p class="footer-copyright">
        © {currentYear} {siteConfig.author.name}. All rights reserved.
      </p>

      <p class="footer-tech">
        <span>Powered by</span>
        <a href="https://svelte.dev" target="_blank" rel="noopener">SvelteKit</a>
        <span>+</span>
        <a href="https://www.rust-lang.org" target="_blank" rel="noopener">Rust</a>
      </p>
    </div>
  </div>
</footer>

<style>
  .footer {
    background: var(--color-white);
    border-top: 1px solid var(--color-border);
    padding: var(--spacing-xs) 0;
    margin-top: var(--spacing-md);
  }

  .footer-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 var(--spacing-lg);
  }

  .footer-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    text-align: center;

    p {
      margin: 0;
      font-size: var(--font-size-xs);
      color: var(--color-text-light);
      line-height: 1.6;
    }

    a {
      color: var(--color-blue);
      text-decoration: none;
      transition: color var(--transition-fast) var(--ease-out);

      &:hover {
        color: var(--color-blue-d10);
      }
    }
  }

  .footer-tech {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
  }

  @media (max-width: 640px) {
    .footer {
      padding: var(--spacing-lg) 0;
    }

    .footer-container {
      padding: 0 var(--spacing-md);
    }
  }
</style>
