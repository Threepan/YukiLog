<script lang="ts">
	import { page } from '$app/state';
	import { contentConfig } from '$lib/config';

	const cfg404 = contentConfig.pages.error404;
	const cfg500 = contentConfig.pages.error500;

	let status = $derived(page.status);
	let is404 = $derived(status === 404);

	let icon = $derived(is404 ? cfg404.icon : cfg500.icon);
	let title = $derived(is404 ? cfg404.title : cfg500.title);
	let description = $derived(is404 ? cfg404.description : cfg500.description);
</script>

<svelte:head>
	<title>{status} · {title} · 管理后台</title>
</svelte:head>

<div class="admin-error">
	<div class="admin-error-icon">{icon}</div>
	<h2 class="admin-error-title">{title}</h2>
	<p class="admin-error-desc">
		{#each description.split('\n') as line, i}
			{#if i > 0}<br />{/if}{line}
		{/each}
	</p>
	{#if !is404}
		<ul class="admin-error-reasons">
			{#each cfg500.reasons as reason}
				<li>{reason}</li>
			{/each}
		</ul>
	{/if}
	<div class="admin-error-actions">
		<a href="/admin" class="btn-back">← 返回后台首页</a>
		{#if !is404}
			<button class="btn-reload" onclick={() => location.reload()}>刷新页面</button>
		{/if}
	</div>
</div>

<style>
	.admin-error {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		text-align: center;
		padding: var(--spacing-2xl);
	}

	.admin-error-icon {
		font-size: 3.5rem;
		margin-bottom: var(--spacing-lg);
	}

	.admin-error-title {
		font-size: var(--font-size-xl);
		font-weight: var(--font-weight-semibold);
		color: var(--color-text);
		margin: 0 0 var(--spacing-md);
	}

	.admin-error-desc {
		color: var(--color-text-light);
		line-height: 1.7;
		margin: 0 0 var(--spacing-lg);
		max-width: 420px;
	}

	.admin-error-reasons {
		text-align: left;
		list-style: none;
		padding: var(--spacing-md) var(--spacing-lg);
		margin: 0 0 var(--spacing-lg);
		background: var(--color-bg);
		border-radius: var(--radius-md);
		border-left: 3px solid var(--color-pink);
		max-width: 420px;
		width: 100%;
	}

	.admin-error-reasons li {
		padding: var(--spacing-xs) 0;
		color: var(--color-text-light);
		font-size: var(--font-size-sm);
	}

	.admin-error-reasons li::before {
		content: '· ';
		color: var(--color-pink);
	}

	.admin-error-actions {
		display: flex;
		gap: var(--spacing-md);
		flex-wrap: wrap;
		justify-content: center;
	}

	.btn-back {
		padding: var(--spacing-sm) var(--spacing-xl);
		border-radius: var(--radius-md);
		font-size: var(--font-size-sm);
		font-weight: var(--font-weight-medium);
		text-decoration: none;
		background: linear-gradient(135deg, var(--color-blue) 0%, var(--color-pink) 100%);
		color: white;
		transition: all var(--transition-fast) var(--ease-gentle);
	}

	.btn-back:hover {
		transform: translateY(-1px);
		box-shadow: var(--shadow-md);
	}

	.btn-reload {
		padding: var(--spacing-sm) var(--spacing-xl);
		border-radius: var(--radius-md);
		font-size: var(--font-size-sm);
		font-weight: var(--font-weight-medium);
		background: transparent;
		color: var(--color-text-light);
		border: 1px solid var(--color-border);
		cursor: pointer;
		transition: all var(--transition-fast) var(--ease-gentle);
	}

	.btn-reload:hover {
		border-color: var(--color-pink);
		color: var(--color-pink);
	}
</style>
