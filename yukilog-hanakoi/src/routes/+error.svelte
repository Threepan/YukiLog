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
	let primaryButton = $derived(is404 ? cfg404.primaryButton : cfg500.primaryButton);
</script>

<svelte:head>
	<title>{status} · {title}</title>
</svelte:head>

<div class="error-page">
	<div class="error-card">
		<div class="error-icon">{icon}</div>
		<h1 class="error-title">{title}</h1>
		<p class="error-description">
			{#each description.split('\n') as line, i}
				{#if i > 0}<br />{/if}{line}
			{/each}
		</p>

		{#if !is404}
			<ul class="error-reasons">
				{#each cfg500.reasons as reason}
					<li>{reason}</li>
				{/each}
			</ul>
			<p class="error-hint">{cfg500.hint}</p>
		{/if}

		<div class="error-actions">
			<a href="/" class="btn btn-primary">{primaryButton}</a>
			{#if is404}
				<button class="btn btn-secondary" onclick={() => history.back()}>
					{cfg404.secondaryButton}
				</button>
			{:else}
				<button class="btn btn-secondary" onclick={() => location.reload()}>
					{cfg500.secondaryButton}
				</button>
			{/if}
		</div>
	</div>
</div>

<style>
	.error-page {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 60vh;
		padding: var(--spacing-2xl) var(--spacing-lg);
	}

	.error-card {
		text-align: center;
		max-width: 520px;
		width: 100%;
		padding: var(--spacing-2xl);
		background: var(--color-surface);
		border-radius: var(--radius-lg);
		border: 1px solid var(--color-border);
		box-shadow: var(--shadow-md);
	}

	.error-icon {
		font-size: 4rem;
		line-height: 1;
		margin-bottom: var(--spacing-lg);
	}

	.error-title {
		font-size: var(--font-size-2xl);
		font-weight: var(--font-weight-semibold);
		color: var(--color-text);
		margin: 0 0 var(--spacing-md);
	}

	.error-description {
		color: var(--color-text-light);
		line-height: 1.7;
		margin: 0 0 var(--spacing-lg);
	}

	.error-reasons {
		text-align: left;
		list-style: none;
		padding: var(--spacing-md);
		margin: 0 0 var(--spacing-md);
		background: var(--color-bg);
		border-radius: var(--radius-md);
		border-left: 3px solid var(--color-pink);
	}

	.error-reasons li {
		padding: var(--spacing-xs) 0;
		color: var(--color-text-light);
		font-size: var(--font-size-sm);
	}

	.error-reasons li::before {
		content: '· ';
		color: var(--color-pink);
	}

	.error-hint {
		font-size: var(--font-size-sm);
		color: var(--color-text-muted);
		margin: 0 0 var(--spacing-lg);
	}

	.error-actions {
		display: flex;
		gap: var(--spacing-md);
		justify-content: center;
		flex-wrap: wrap;
	}

	.btn {
		padding: var(--spacing-sm) var(--spacing-xl);
		border-radius: var(--radius-md);
		font-size: var(--font-size-sm);
		font-weight: var(--font-weight-medium);
		cursor: pointer;
		transition: all var(--transition-fast) var(--ease-gentle);
		text-decoration: none;
		display: inline-flex;
		align-items: center;
	}

	.btn-primary {
		background: linear-gradient(135deg, var(--color-pink) 0%, var(--color-blue) 100%);
		color: white;
		border: none;
	}

	.btn-primary:hover {
		transform: translateY(-1px);
		box-shadow: var(--shadow-md);
	}

	.btn-secondary {
		background: transparent;
		color: var(--color-text-light);
		border: 1px solid var(--color-border);
	}

	.btn-secondary:hover {
		border-color: var(--color-pink);
		color: var(--color-pink);
	}
</style>
