<script lang="ts">
	import { onMount } from 'svelte';
	import { adminApi } from '$lib/api';
	import { formatDate } from '$lib/date';
	import type { PostWithRelations, Post, Theme, Tag } from '$types';

	const PAGE_SIZE = 20;

	type StatusFilter = 'all' | 'published' | 'draft';

	let allPosts: PostWithRelations[] = $state([]);
	let loading = $state(true);
	let error = $state('');
	let currentStatus: StatusFilter = $state('all');
	let currentPage = $state(1);

	// 删除确认
	let deleteTarget: { slug: string; title: string } | null = $state(null);
	let deleting = $state(false);

	interface FlatPost {
		id: number;
		title: string;
		slug: string;
		status: string;
		view_count: number;
		created_at: string;
		theme: Theme | null;
		tags: Tag[];
	}

	function flattenPost(pw: PostWithRelations): FlatPost {
		return {
			id: pw.post.id,
			title: pw.post.title,
			slug: pw.post.slug,
			status: pw.post.status,
			view_count: pw.post.view_count,
			created_at: pw.post.created_at,
			theme: pw.theme ?? null,
			tags: pw.tags ?? [],
		};
	}

	const filteredPosts = $derived(
		currentStatus === 'all'
			? allPosts.map(flattenPost)
			: allPosts.map(flattenPost).filter(p => p.status === currentStatus)
	);

	const totalPages = $derived(Math.max(1, Math.ceil(filteredPosts.length / PAGE_SIZE)));
	const pagePosts = $derived(
		filteredPosts.slice((currentPage - 1) * PAGE_SIZE, currentPage * PAGE_SIZE)
	);

	const statusTabs: { key: StatusFilter; label: string }[] = [
		{ key: 'all', label: '全部' },
		{ key: 'published', label: '已发布' },
		{ key: 'draft', label: '草稿' },
	];

	function switchTab(status: StatusFilter) {
		currentStatus = status;
		currentPage = 1;
	}

	async function loadPosts() {
		loading = true;
		error = '';
		try {
			const res = await adminApi.posts.list({ page: 1, page_size: 9999 });
			allPosts = res.items;
		} catch (err: any) {
			error = err.message || '加载失败';
		} finally {
			loading = false;
		}
	}

	function confirmDelete(slug: string, title: string) {
		deleteTarget = { slug, title };
	}

	async function executeDelete() {
		if (!deleteTarget) return;
		deleting = true;
		try {
			await adminApi.posts.delete(deleteTarget.slug);
			deleteTarget = null;
			await loadPosts();
		} catch (err: any) {
			alert(err.message || '删除失败');
		} finally {
			deleting = false;
		}
	}

	onMount(() => {
		loadPosts();
	});
</script>

<svelte:head>
	<title>文章管理 - YukiLog</title>
</svelte:head>

<!-- 操作栏 -->
<div class="action-bar">
	<div class="tabs">
		{#each statusTabs as tab}
			<button
				class="tab"
				class:active={currentStatus === tab.key}
				onclick={() => switchTab(tab.key)}
			>{tab.label}</button>
		{/each}
	</div>
	<a href="/admin/posts/new" class="btn-create">
		<span>+</span> 新建文章
	</a>
</div>

<!-- 内容区 -->
{#if loading}
	<div class="admin-loading">
		<div class="spinner"></div>
		<p>加载中...</p>
	</div>
{:else if error}
	<div class="admin-error">
		<p>{error}</p>
		<button class="btn-retry" onclick={loadPosts}>重新加载</button>
	</div>
{:else if pagePosts.length === 0}
	<div class="admin-empty">
		<p>暂无文章</p>
	</div>
{:else}
	<div class="posts-grid">
		{#each pagePosts as post (post.id)}
			<div class="post-card">
				<div class="card-status-strip status-{post.status}"></div>
				<div class="card-body">
					<div class="card-header">
						<a href="/posts/{post.slug}" class="post-title" target="_blank">{post.title}</a>
						<span class="status-badge status-{post.status}">
							{post.status === 'published' ? '已发布' : '草稿'}
						</span>
					</div>

					<div class="card-meta">
						{#if post.theme}
							<span class="theme-badge">{post.theme.name}</span>
						{/if}
						{#each post.tags as tag}
							<span class="tag-chip">#{tag.name}</span>
						{/each}
					</div>

					<div class="card-footer">
						<div class="card-info">
							<span class="view-count">👁 {post.view_count}</span>
							<span class="post-date">{formatDate(post.created_at)}</span>
						</div>
						<div class="card-actions">
							<a href="/admin/posts/edit/{post.slug}" class="action-btn edit">编辑</a>
							<button class="action-btn delete" onclick={() => confirmDelete(post.slug, post.title)}>删除</button>
						</div>
					</div>
				</div>
			</div>
		{/each}
	</div>

	<!-- 分页 -->
	{#if filteredPosts.length > PAGE_SIZE}
		<div class="pagination">
			<button class="page-btn" disabled={currentPage === 1} onclick={() => currentPage--}>上一页</button>
			<span class="page-info">第 {currentPage} / {totalPages} 页</span>
			<button class="page-btn" disabled={currentPage === totalPages} onclick={() => currentPage++}>下一页</button>
		</div>
	{/if}
{/if}

<!-- 删除确认弹窗 -->
{#if deleteTarget}
	<div class="modal-backdrop" onclick={() => (deleteTarget = null)} role="presentation"></div>
	<div class="modal-card">
		<div class="modal-header">
			<h3>确认删除</h3>
			<button class="modal-close" onclick={() => (deleteTarget = null)}>×</button>
		</div>
		<div class="modal-body">
			<div class="confirm-body">
				<div class="confirm-icon">⚠️</div>
				<p class="confirm-title">删除文章「{deleteTarget.title}」？</p>
				<p class="confirm-desc">此操作不可撤销，关联的评论和标签关系也将被清除。</p>
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={() => (deleteTarget = null)}>取消</button>
			<button class="btn-danger" disabled={deleting} onclick={executeDelete}>
				{deleting ? '删除中...' : '确认删除'}
			</button>
		</div>
	</div>
{/if}

<style>
	.posts-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
		gap: 1.5rem;
		margin-bottom: 1.5rem;
	}

	.post-card {
		background: var(--color-white);
		border-radius: 20px;
		box-shadow: var(--shadow-sm);
		border: 1px solid var(--color-divider);
		overflow: hidden;
		transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
	}

	.post-card:hover {
		box-shadow: var(--shadow-blue-md);
		border-color: var(--blue-alpha-30);
		transform: translateY(-2px);
	}

	.card-status-strip {
		height: 3px;
	}

	.card-status-strip.status-published {
		background: linear-gradient(90deg, #3ab26e, #5cd690);
	}

	.card-status-strip.status-draft {
		background: linear-gradient(90deg, #e8984a, #f0b070);
	}

	.card-body {
		padding: 1.25rem 1.5rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.card-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 0.75rem;
	}

	.post-title {
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--color-text);
		text-decoration: none;
		line-height: 1.4;
		flex: 1;
		overflow: hidden;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
	}

	.post-title:hover {
		color: var(--color-blue);
	}

	.card-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.theme-badge {
		padding: 0.125rem 0.625rem;
		font-size: 0.6875rem;
		font-weight: 500;
		color: var(--color-blue);
		background: var(--blue-alpha-08);
		border-radius: 12px;
	}

	.tag-chip {
		padding: 0.125rem 0.5rem;
		font-size: 0.6875rem;
		color: var(--color-pink);
		background: var(--pink-alpha-08);
		border-radius: 12px;
	}

	.card-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-top: 0.75rem;
		border-top: 1px solid var(--color-divider);
	}

	.card-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.card-actions {
		display: flex;
		gap: 0.5rem;
	}
</style>
