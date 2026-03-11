<script lang="ts">
	import '../styles/global.css';
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import NavBar from '$lib/../components/navigation/NavBar.svelte';
	import Footer from '$lib/../components/shared/Footer.svelte';
	import ScrollProgress from '$lib/../components/shared/ScrollProgress.svelte';
	import SearchOverlay from '$lib/../components/shared/SearchOverlay.svelte';
	import MusicPlayer from '$lib/../components/shared/MusicPlayer.svelte';
	import { siteConfig } from '$lib/config';

	let { children } = $props();

	const isHome = $derived(page.url.pathname === '/');
	const isAdmin = $derived(page.url.pathname.startsWith('/admin'));

	// 页面失去焦点时修改标题
	onMount(() => {
		let originalTitle = document.title;

		const handler = () => {
			if (document.hidden) {
				originalTitle = document.title;
				document.title = '...你 ... 要走了吗?';
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
	<meta property="og:locale" content="zh_CN" />
	<meta name="theme-color" content="#E8A4B4" />
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
