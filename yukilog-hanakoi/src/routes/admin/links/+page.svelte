<script lang="ts">
	import { onMount } from 'svelte';
	import { adminApi } from '$lib/api';
	import type { Link } from '$types';

	const PAGE_SIZE = 20;

	let allLinks: Link[] = $state([]);
	let pendingCount = $state(0);
	let loading = $state(true);
	let error = $state('');
	let currentStatus: string = $state('all');
	let currentPage = $state(1);

	// 编辑弹窗
	let showModal = $state(false);
	let editingLink: Link | null = $state(null);
	let formTitle = $state('');
	let formUrl = $state('');
	let formDescription = $state('');
	let formAvatar = $state('');
	let submitting = $state(false);

	// 删除确认
	let deleteTarget: Link | null = $state(null);
	let deleting = $state(false);

	const filteredLinks = $derived(
		currentStatus === 'all'
			? allLinks
			: allLinks.filter(l => l.status === currentStatus)
	);

	const totalPages = $derived(Math.max(1, Math.ceil(filteredLinks.length / PAGE_SIZE)));
	const pageLinks = $derived(
		filteredLinks.slice((currentPage - 1) * PAGE_SIZE, currentPage * PAGE_SIZE)
	);

	const statusTabs: { key: string; label: string }[] = [
		{ key: 'all', label: '全部' },
		{ key: 'pending', label: '待审核' },
		{ key: 'active', label: '已通过' },
		{ key: 'broken', label: '已失效' },
	];

	function getStatusText(status: string): string {
		const map: Record<string, string> = { pending: '待审核', active: '已通过', broken: '已失效' };
		return map[status] || status;
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		const now = new Date();
		const days = Math.floor((now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24));
		if (days === 0) return '今天';
		if (days === 1) return '昨天';
		if (days < 7) return `${days}天前`;
		return date.toLocaleDateString('zh-CN', { year: 'numeric', month: '2-digit', day: '2-digit' });
	}

	async function loadLinks() {
		loading = true;
		error = '';
		try {
			const [links, pending] = await Promise.all([
				adminApi.links.list(),
				adminApi.links.pending()
			]);
			allLinks = links;
			pendingCount = pending.length;
		} catch (err: any) {
			error = err.message || '加载失败';
		} finally {
			loading = false;
		}
	}

	function switchTab(status: string) {
		currentStatus = status;
		currentPage = 1;
	}

	async function handleApprove(id: number) {
		const link = allLinks.find(l => l.id === id);
		if (!link || !confirm(`确认通过友链「${link.title}」？`)) return;
		try {
			await adminApi.links.approve(id);
			await loadLinks();
		} catch (err: any) {
			alert(err.message || '操作失败');
		}
	}

	async function handleMarkBroken(id: number) {
		const link = allLinks.find(l => l.id === id);
		if (!link || !confirm(`确认标记友链「${link.title}」为失效？`)) return;
		try {
			await adminApi.links.markBroken(id);
			await loadLinks();
		} catch (err: any) {
			alert(err.message || '操作失败');
		}
	}

	function openCreateModal() {
		editingLink = null;
		formTitle = '';
		formUrl = '';
		formDescription = '';
		formAvatar = '';
		showModal = true;
	}

	function openEditModal(link: Link) {
		editingLink = link;
		formTitle = link.title;
		formUrl = link.url;
		formDescription = link.description || '';
		formAvatar = link.avatar || '';
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingLink = null;
	}

	async function handleSubmit() {
		if (!formTitle.trim() || !formUrl.trim()) { alert('网站名称和地址不能为空'); return; }
		submitting = true;
		try {
			const data = {
				title: formTitle.trim(),
				url: formUrl.trim(),
				description: formDescription.trim() || undefined,
				avatar: formAvatar.trim() || undefined,
			};
			if (editingLink) {
				await adminApi.links.update(editingLink.id, data);
			} else {
				await adminApi.links.create(data);
			}
			closeModal();
			await loadLinks();
		} catch (err: any) {
			alert(err.message || '操作失败');
		} finally {
			submitting = false;
		}
	}

	function confirmDelete(link: Link) {
		deleteTarget = link;
	}

	async function executeDelete() {
		if (!deleteTarget) return;
		deleting = true;
		try {
			await adminApi.links.delete(deleteTarget.id);
			deleteTarget = null;
			await loadLinks();
		} catch (err: any) {
			alert(err.message || '删除失败');
		} finally {
			deleting = false;
		}
	}

	onMount(() => { loadLinks(); });
</script>

<svelte:head>
	<title>友链管理 - YukiLog</title>
</svelte:head>

<!-- 待审核提示 -->
{#if pendingCount > 0}
	<div class="notice">
		<div class="notice-icon">⏳</div>
		<div class="notice-content">
			<h4>有新的友链等待审核</h4>
			<p>共 <strong>{pendingCount}</strong> 条待审核友链，点击下方「待审核」标签查看</p>
		</div>
	</div>
{/if}

<!-- 操作栏 -->
<div class="action-bar">
	<div class="tabs">
		{#each statusTabs as tab}
			<button class="tab" class:active={currentStatus === tab.key} onclick={() => switchTab(tab.key)}>{tab.label}</button>
		{/each}
	</div>
	<button class="btn-create" onclick={openCreateModal}>
		<span>+</span> 新建友链
	</button>
</div>

<!-- 内容区 -->
{#if loading}
	<div class="admin-loading"><div class="spinner"></div><p>加载中...</p></div>
{:else if error}
	<div class="admin-error"><p>{error}</p><button class="btn-retry" onclick={loadLinks}>重新加载</button></div>
{:else if pageLinks.length === 0}
	<div class="admin-empty"><p>暂无友链</p></div>
{:else}
	<div class="admin-card-grid">
		{#each pageLinks as link (link.id)}
			{@const initial = (link.title || '?').charAt(0).toUpperCase()}
			<div class="link-card">
				<div class="link-top">
					<div class="link-avatar-wrap">
						{#if link.avatar}
							<img
								src={link.avatar}
								alt={link.title}
								class="site-avatar"
								onerror={(e) => { (e.currentTarget as HTMLImageElement).style.display = 'none'; (e.currentTarget as HTMLImageElement).nextElementSibling?.setAttribute('style', 'display:flex'); }}
							/>
							<div class="site-avatar-fallback" style="display:none">{initial}</div>
						{:else}
							<div class="site-avatar-fallback">{initial}</div>
						{/if}
					</div>
					<div class="link-info">
						<span class="link-title">{link.title}</span>
						<a class="link-url-text" href={link.url} target="_blank" rel="noopener noreferrer">{link.url}</a>
					</div>
					<span class="status-dot status-dot-{link.status}" title={getStatusText(link.status)}></span>
				</div>

				{#if link.description}
					<p class="link-desc">{link.description}</p>
				{:else}
					<p class="link-no-desc">暂无描述</p>
				{/if}

				<div class="link-footer">
					<div class="link-footer-left">
						<span class="status-badge status-{link.status}">{getStatusText(link.status)}</span>
						<span class="link-date">{formatDate(link.created_at)}</span>
					</div>
					<div class="link-actions">
						{#if link.status === 'pending'}
							<button class="action-btn approve" onclick={() => handleApprove(link.id)}>通过</button>
						{/if}
						{#if link.status === 'active'}
							<button class="action-btn reject" onclick={() => handleMarkBroken(link.id)}>失效</button>
						{/if}
						<button class="action-btn edit" onclick={() => openEditModal(link)}>编辑</button>
						<button class="action-btn delete" onclick={() => confirmDelete(link)}>删除</button>
					</div>
				</div>
			</div>
		{/each}
	</div>

	{#if filteredLinks.length > PAGE_SIZE}
		<div class="pagination">
			<button class="page-btn" disabled={currentPage === 1} onclick={() => { currentPage--; }}>上一页</button>
			<span class="page-info">第 {currentPage} / {totalPages} 页</span>
			<button class="page-btn" disabled={currentPage === totalPages} onclick={() => { currentPage++; }}>下一页</button>
		</div>
	{/if}
{/if}

<!-- 创建/编辑弹窗 -->
{#if showModal}
	<div class="modal-backdrop" onclick={closeModal} role="presentation"></div>
	<div class="modal-card">
		<div class="modal-header">
			<h3>{editingLink ? '编辑友链' : '新建友链'}</h3>
			<button class="modal-close" onclick={closeModal}>×</button>
		</div>
		<div class="modal-body">
			<div class="form-group">
				<label for="link-name">网站名称 *</label>
				<input type="text" id="link-name" bind:value={formTitle} placeholder="例如：Lian's Tech Blog" />
			</div>
			<div class="form-group">
				<label for="link-url">网站地址 *</label>
				<input type="url" id="link-url" bind:value={formUrl} placeholder="https://example.com" />
			</div>
			<div class="form-group">
				<label for="link-desc">网站描述</label>
				<textarea id="link-desc" bind:value={formDescription} rows="3" placeholder="简单介绍一下这个网站..."></textarea>
			</div>
			<div class="form-group">
				<label for="link-avatar">头像 URL</label>
				<input type="url" id="link-avatar" bind:value={formAvatar} placeholder="https://example.com/avatar.jpg" />
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={closeModal}>取消</button>
			<button class="btn-primary" disabled={submitting} onclick={handleSubmit}>
				{submitting ? '保存中...' : '保存'}
			</button>
		</div>
	</div>
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
				<p class="confirm-title">删除友链「{deleteTarget.title}」？</p>
				<p class="confirm-desc">此操作不可撤销。</p>
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
	/* 友链卡片 */
	.link-card {
		background: var(--color-white);
		border-radius: 20px;
		box-shadow: var(--shadow-sm);
		border: 1px solid var(--color-divider);
		padding: 1.5rem 1.75rem;
		display: flex;
		flex-direction: column;
		gap: 0.875rem;
		transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
	}

	.link-card:hover {
		box-shadow: var(--shadow-blue-md);
		border-color: var(--blue-alpha-30);
		transform: translateY(-2px);
	}

	.link-top {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.link-avatar-wrap {
		flex-shrink: 0;
	}

	.site-avatar {
		width: 52px;
		height: 52px;
		border-radius: 14px;
		object-fit: cover;
		border: 1px solid var(--color-divider);
	}

	.site-avatar-fallback {
		width: 52px;
		height: 52px;
		border-radius: 14px;
		background: linear-gradient(135deg, var(--blue-alpha-15) 0%, var(--pink-alpha-15) 100%);
		border: 1px solid var(--blue-alpha-15);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 1.375rem;
		font-weight: 700;
		color: var(--color-blue);
	}

	.link-info {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 3px;
	}

	.link-title {
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--color-text);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.link-url-text {
		font-size: 0.75rem;
		color: var(--color-blue);
		text-decoration: none;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		opacity: 0.8;
	}

	.link-url-text:hover {
		opacity: 1;
		text-decoration: underline;
	}

	/* 状态指示点 */
	.status-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		flex-shrink: 0;
		margin-left: auto;
	}

	.status-dot-active {
		background: #3ab26e;
		box-shadow: 0 0 0 3px rgba(58, 178, 110, 0.2);
	}

	.status-dot-pending {
		background: #e8984a;
		box-shadow: 0 0 0 3px rgba(232, 152, 74, 0.2);
	}

	.status-dot-broken {
		background: var(--color-error);
		box-shadow: 0 0 0 3px var(--error-alpha-15);
	}

	.link-desc {
		font-size: 0.8125rem;
		color: var(--color-text-light);
		line-height: 1.6;
		margin: 0;
		overflow: hidden;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
	}

	.link-no-desc {
		font-size: 0.8125rem;
		color: var(--color-text-muted);
		margin: 0;
		font-style: italic;
	}

	.link-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		border-top: 1px solid var(--color-divider);
		padding-top: 0.875rem;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.link-footer-left {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.link-date {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
	}

	.link-actions {
		display: flex;
		gap: 0.5rem;
	}
</style>
