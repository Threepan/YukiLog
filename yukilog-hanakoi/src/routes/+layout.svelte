<script lang="ts">
	import '../styles/global.css';
	import { page } from '$app/state';
	import NavBar from '$lib/../components/navigation/NavBar.svelte';
	import Footer from '$lib/../components/shared/Footer.svelte';
	import ScrollProgress from '$lib/../components/shared/ScrollProgress.svelte';
	import SearchOverlay from '$lib/../components/shared/SearchOverlay.svelte';
	import { siteConfig } from '$lib/config';

	let { children } = $props();

	const isHome = $derived(page.url.pathname === '/');
	const isAdmin = $derived(page.url.pathname.startsWith('/admin'));
</script>

<svelte:head>
	<meta name="author" content={siteConfig.author.name} />
	<meta property="og:locale" content="zh_CN" />
	<meta name="theme-color" content="#E8A4B4" />
</svelte:head>

{#if isAdmin}
	{@render children()}
{:else}
	<NavBar stickyOnly={!isHome} />
	<SearchOverlay />
	<ScrollProgress />

	<main>
		{@render children()}
	</main>

	<Footer />
{/if}

<style>
	main {
		flex: 1;
		min-height: 0;
	}
</style>
