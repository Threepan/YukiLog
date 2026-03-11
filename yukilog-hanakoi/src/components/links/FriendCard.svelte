<script lang="ts">
  interface Props {
    name: string;
    description: string | null;
    avatar: string | null;
    url: string;
    index: number;
  }

  let { name, description, avatar, url, index }: Props = $props();

  const rotations = [-2.5, 1.8, -1.2, 3, -1.8, 2.2, -2.8, 1.5, -0.8, 2.6];
  const rotation = rotations[index % rotations.length];
  const isPink = index % 2 === 0;
</script>

<a
  href={url}
  target="_blank"
  rel="noopener"
  class="friend-card"
  class:pink={isPink}
  class:blue={!isPink}
  style="--rotation: {rotation}deg; --i: {index}"
>
  <div class="friend-avatar-wrap">
    {#if avatar}
      <img src={avatar} alt={name} class="friend-avatar" loading="lazy" />
    {:else}
      <div class="friend-avatar friend-avatar-fallback">
        {name.charAt(0)}
      </div>
    {/if}
  </div>

  <div class="friend-info">
    <span class="friend-name">{name}</span>
    {#if description}
      <span class="friend-desc">{description}</span>
    {/if}
  </div>
</a>

<style>
  .friend-card {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    background: var(--color-white);
    border-radius: var(--radius-md);
    text-decoration: none;
    color: var(--color-text);
    transform: rotate(var(--rotation));
    transition:
      transform 300ms var(--ease-gentle),
      box-shadow 300ms var(--ease-gentle);

    /* 入场动画 */
    opacity: 0;
    animation: card-enter 500ms var(--ease-gentle) forwards;
    animation-delay: calc(var(--i, 0) * 80ms + 400ms);

    &.pink {
      box-shadow: var(--shadow-pink);
    }

    &.blue {
      box-shadow: var(--shadow-blue);
    }

    /* 悬停："拿起来" */
    &:hover {
      transform: rotate(0deg) translateY(-6px) scale(1.04);
      z-index: 2;

      &.pink {
        box-shadow: var(--shadow-pink-offset-hover);
      }

      &.blue {
        box-shadow: var(--shadow-blue-offset-hover);
      }

      .friend-avatar {
        transform: scale(1.1);
      }
    }

    &:active {
      transform: rotate(0deg) translateY(-2px) scale(1.01);
    }
  }

  @keyframes card-enter {
    from {
      opacity: 0;
      transform: rotate(var(--rotation)) translateY(20px);
    }
    to {
      opacity: 1;
      transform: rotate(var(--rotation));
    }
  }

  .friend-avatar-wrap {
    flex-shrink: 0;
  }

  .friend-avatar {
    width: 48px;
    height: 48px;
    border-radius: 50%;
    object-fit: cover;
    transition: transform var(--transition-fast) var(--ease-gentle);
  }

  .friend-avatar-fallback {
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--color-pink), var(--color-blue));
    color: var(--color-on-primary);
    font-size: var(--font-size-lg);
    font-weight: var(--font-weight-semibold);
  }

  .friend-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .friend-name {
    font-size: var(--font-size-base);
    font-weight: var(--font-weight-semibold);
    color: var(--color-text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .friend-desc {
    font-size: var(--font-size-xs);
    color: var(--color-text-light);
    line-height: var(--line-height-base);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
