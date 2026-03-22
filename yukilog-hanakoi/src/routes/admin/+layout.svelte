<script lang="ts">
	import '../../styles/admin.css';
	import { page } from '$app/state';
	import { onMount } from 'svelte';
	import { guardAdminRoute, logout } from '$lib/admin-guard';
	import { svgIcons } from '$lib/svg-icons';

	let { children } = $props();

	const isLoginPage = $derived(page.url.pathname === '/admin/login');

	const menuItems = [
		{ key: 'home', label: '管理首页', href: '/admin', icon: svgIcons.home },
		{ key: 'posts', label: '文章管理', href: '/admin/posts', icon: svgIcons.folderOpen },
		{ key: 'comments', label: '评论管理', href: '/admin/comments', icon: svgIcons.envelope },
		{ key: 'themes', label: '主题管理', href: '/admin/themes', icon: svgIcons.theme },
		{ key: 'tags', label: '标签管理', href: '/admin/tags', icon: svgIcons.tag },
		{ key: 'links', label: '友链管理', href: '/admin/links', icon: svgIcons.links },
		{ key: 'notes', label: '随记管理', href: '/admin/notes', icon: svgIcons.notes },
	];

	// 根据路径判断当前激活菜单
	const activeMenu = $derived(
		menuItems.find(item =>
			item.href === '/admin'
				? page.url.pathname === '/admin'
				: page.url.pathname.startsWith(item.href)
		)?.key ?? ''
	);

	// 页面标题映射
	const pageTitle = $derived(() => {
		if (page.url.pathname.includes('/posts/new')) return '新建文章';
		if (page.url.pathname.includes('/posts/edit')) return '编辑文章';
		const item = menuItems.find(m =>
			m.href === '/admin'
				? page.url.pathname === '/admin'
				: page.url.pathname.startsWith(m.href)
		);
		return item?.label ?? '管理后台';
	});

	onMount(() => {
		if (!isLoginPage) {
			guardAdminRoute();
		}
	});
</script>

{#if isLoginPage}
	{@render children()}
{:else}
	<div class="admin-layout">
		<!-- 侧边栏 -->
		<aside class="admin-sidebar">
			<div class="sidebar-header">
				<a href="/" class="sidebar-logo">YukiLog</a>
				<div class="sidebar-subtitle">管理后台</div>
			</div>

			<nav class="sidebar-nav">
				{#each menuItems as item}
					<a
						href={item.href}
						class="nav-item"
						class:active={activeMenu === item.key}
					>
						<span class="nav-icon">{@html item.icon}</span>
						<span class="nav-label">{item.label}</span>
					</a>
				{/each}
			</nav>

			<div class="sidebar-footer">
				<a href="/" class="back-to-site">← 返回前台</a>
			</div>
		</aside>

		<!-- 主区域 -->
		<div class="admin-main">
			<!-- 顶栏 -->
			<header class="admin-header">
				<h1 class="header-title">{pageTitle()}</h1>
				<div class="header-actions">
					<span class="admin-name">管理员</span>
					<button class="logout-btn" onclick={logout}>退出登录</button>
				</div>
			</header>

			<!-- 内容区 -->
			<main class="admin-content">
				{@render children()}
			</main>
		</div>
	</div>
{/if}

<style>
	.admin-layout {
		display: flex;
		min-height: 100vh;
		background: var(--color-bg);
	}

	/* ===== 侧边栏 ===== */
	.admin-sidebar {
		width: 260px;
		background: var(--color-white);
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
		box-shadow: var(--shadow-md);
	}

	.sidebar-header {
		padding: var(--spacing-xl) var(--spacing-lg);
		background: var(--gradient-blue-pink);
	}

	.sidebar-logo {
		font-size: var(--font-size-2xl);
		font-weight: var(--font-weight-bold);
		background: linear-gradient(135deg, var(--color-blue) 0%, var(--color-pink) 100%);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		text-decoration: none;
		display: block;
		transition: transform var(--transition-fast) var(--ease-gentle);
	}

	.sidebar-logo:hover {
		transform: scale(1.05);
	}

	.sidebar-subtitle {
		font-size: var(--font-size-xs);
		color: var(--color-text-muted);
		margin-top: var(--spacing-xs);
	}

	.sidebar-nav {
		flex: 1;
		padding: var(--spacing-lg) var(--spacing-md);
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
	}

	.nav-item {
		display: flex;
		align-items: center;
		gap: var(--spacing-sm);
		padding: var(--spacing-sm) var(--spacing-md);
		color: var(--color-text-light);
		text-decoration: none;
		border-radius: var(--radius-sm);
		transition: all var(--transition-base) var(--ease-gentle);
		position: relative;
	}

	.nav-item::before {
		content: '';
		position: absolute;
		left: 0;
		top: 50%;
		transform: translateY(-50%);
		width: 3px;
		height: 0;
		background: linear-gradient(180deg, var(--color-blue), var(--color-pink));
		border-radius: 0 2px 2px 0;
		transition: height var(--transition-base) var(--ease-gentle);
	}

	.nav-item:hover {
		background: var(--gradient-blue-pink-8);
		color: var(--color-blue);
		transform: translateX(4px);
	}

	.nav-item.active {
		background: var(--gradient-blue-pink-12);
		color: var(--color-blue);
		font-weight: var(--font-weight-medium);
		box-shadow: var(--shadow-sm);
	}

	.nav-item.active::before {
		height: 60%;
	}

	.nav-icon {
		width: 20px;
		height: 20px;
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
	}

	.nav-icon :global(svg) {
		width: 18px;
		height: 18px;
		fill: currentColor;
	}

	.nav-label {
		font-size: var(--font-size-sm);
	}

	.sidebar-footer {
		padding: var(--spacing-md);
		border-top: 1px solid var(--color-divider);
	}

	.back-to-site {
		display: block;
		padding: var(--spacing-sm) var(--spacing-md);
		font-size: var(--font-size-sm);
		color: var(--color-text-muted);
		text-align: center;
		text-decoration: none;
		border-radius: var(--radius-sm);
		transition: all var(--transition-fast) var(--ease-gentle);
	}

	.back-to-site:hover {
		background: var(--gradient-blue-pink-8);
		color: var(--color-blue);
		transform: translateX(-4px);
	}

	/* ===== 主区域 ===== */
	.admin-main {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}

	.admin-header {
		height: 72px;
		background: var(--color-white);
		padding: 0 var(--spacing-xl);
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-shrink: 0;
		box-shadow: var(--shadow-sm);
		position: relative;
		z-index: 10;
	}

	.header-title {
		font-size: var(--font-size-xl);
		font-weight: var(--font-weight-semibold);
		background: linear-gradient(135deg, var(--color-blue) 0%, var(--color-pink) 100%);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		margin: 0;
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: var(--spacing-lg);
	}

	.admin-name {
		font-size: var(--font-size-sm);
		color: var(--color-text-light);
		padding: var(--spacing-xs) var(--spacing-md);
		background: var(--gradient-blue-pink);
		border-radius: var(--radius-sm);
	}

	.logout-btn {
		padding: var(--spacing-xs) var(--spacing-md);
		font-size: var(--font-size-sm);
		color: var(--color-on-primary);
		background: linear-gradient(135deg, var(--color-error) 0%, color-mix(in srgb, var(--color-error) 80%, var(--color-pink)) 100%);
		border: none;
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: all var(--transition-fast) var(--ease-gentle);
	}

	.logout-btn:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}

	.admin-content {
		flex: 1;
		padding: var(--spacing-xl);
		overflow-y: auto;
	}
</style>
