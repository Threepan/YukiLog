<script lang="ts">
	import '../styles/global.css';
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import NavBar from '$components/navigation/NavBar.svelte';
	import Footer from '$components/shared/Footer.svelte';
	import ScrollProgress from '$components/shared/ScrollProgress.svelte';
	import SearchOverlay from '$components/shared/SearchOverlay.svelte';
	import MusicPlayer from '$components/shared/MusicPlayer.svelte';
	import { siteConfig, contentConfig, designTokens } from '$lib/config';

	let { children } = $props();

	const isHome = $derived(page.url.pathname === '/');
	const isAdmin = $derived(page.url.pathname.startsWith('/admin'));

	// 页面失去焦点时修改标题
	onMount(() => {
		let originalTitle = document.title;

		const handler = () => {
			if (document.hidden) {
				originalTitle = document.title;
				document.title = contentConfig.ui.tabHiddenTitle;
			} else {
				document.title = originalTitle;
			}
		};

		document.addEventListener('visibilitychange', handler);
		return () => document.removeEventListener('visibilitychange', handler);
	});
</script>

<svelte:head>
	<meta name="author" content={siteConfig.author.name} />
	<meta property="og:locale" content={siteConfig.lang.replace('-', '_')} />
	<meta name="theme-color" content={designTokens.colors.lianPink} />
</svelte:head>

{#if isAdmin}
	{@render children()}
{:else}
	<NavBar stickyOnly={!isHome} />
	<SearchOverlay />
	<ScrollProgress />
	<MusicPlayer />

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
