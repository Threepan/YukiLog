<script lang="ts">
	import { onMount } from 'svelte';
	import { themesApi, adminApi } from '$lib/api';
	import { generateSlug } from '$lib/slugify';
	import type { Theme } from '$types';

	let themes: Theme[] = $state([]);
	let loading = $state(true);
	let error = $state('');

	// 编辑弹窗
	let showModal = $state(false);
	let editingTheme: Theme | null = $state(null);
	let formName = $state('');
	let formSlug = $state('');
	let formDescription = $state('');
	let slugManuallyEdited = $state(false);
	let submitting = $state(false);

	// 删除确认
	let deleteTarget: Theme | null = $state(null);
	let deleting = $state(false);

	async function loadThemes() {
		loading = true;
		error = '';
		try {
			themes = await themesApi.list();
		} catch (err: any) {
			error = err.message || '加载失败';
		} finally {
			loading = false;
		}
	}

	function openCreateModal() {
		editingTheme = null;
		formName = '';
		formSlug = '';
		formDescription = '';
		slugManuallyEdited = false;
		showModal = true;
	}

	function openEditModal(theme: Theme) {
		editingTheme = theme;
		formName = theme.name;
		formSlug = theme.slug;
		formDescription = theme.description || '';
		slugManuallyEdited = true;
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingTheme = null;
	}

	function handleNameInput() {
		if (!slugManuallyEdited) {
			formSlug = generateSlug(formName);
		}
	}

	function handleSlugInput() {
		slugManuallyEdited = true;
	}

	async function handleSubmit() {
		if (!formName.trim() || !formSlug.trim()) { alert('名称和 Slug 不能为空'); return; }
		submitting = true;
		try {
			const data = {
				name: formName.trim(),
				slug: formSlug.trim(),
				description: formDescription.trim() || undefined,
			};
			if (editingTheme) {
				await adminApi.themes.update(editingTheme.id, data);
			} else {
				await adminApi.themes.create(data);
			}
			closeModal();
			await loadThemes();
		} catch (err: any) {
			alert(err.message || '操作失败');
		} finally {
			submitting = false;
		}
	}

	function confirmDelete(theme: Theme) {
		deleteTarget = theme;
	}

	async function executeDelete() {
		if (!deleteTarget) return;
		deleting = true;
		try {
			await adminApi.themes.delete(deleteTarget.id);
			deleteTarget = null;
			await loadThemes();
		} catch (err: any) {
			alert(err.message || '删除失败');
		} finally {
			deleting = false;
		}
	}

	onMount(() => { loadThemes(); });
</script>

<svelte:head>
	<title>主题管理 - YukiLog</title>
</svelte:head>

<!-- 操作栏 -->
<div class="action-bar">
	<div class="tabs">
		<button class="tab active">全部主题</button>
	</div>
	<button class="btn-create" onclick={openCreateModal}>
		<span>+</span> 新建主题
	</button>
</div>

<!-- 内容区 -->
{#if loading}
	<div class="admin-loading"><div class="spinner"></div><p>加载中...</p></div>
{:else if error}
	<div class="admin-error"><p>{error}</p><button class="btn-retry" onclick={loadThemes}>重新加载</button></div>
{:else if themes.length === 0}
	<div class="admin-empty"><p>暂无主题</p></div>
{:else}
	<div class="admin-card-grid">
		{#each themes as theme (theme.id)}
			<div class="theme-card">
				<div class="theme-header">
					<h3 class="theme-name">{theme.name}</h3>
					<span class="theme-slug">/{theme.slug}</span>
				</div>
				{#if theme.description}
					<p class="theme-desc">{theme.description}</p>
				{:else}
					<p class="theme-no-desc">暂无描述</p>
				{/if}
				<div class="theme-stats">
					<span>📝 {theme.post_count ?? 0} 篇文章</span>
					<span>👁 {theme.view_count ?? 0} 浏览</span>
				</div>
				<div class="theme-actions">
					<button class="action-btn edit" onclick={() => openEditModal(theme)}>编辑</button>
					<button class="action-btn delete" onclick={() => confirmDelete(theme)}>删除</button>
				</div>
			</div>
		{/each}
	</div>
{/if}

<!-- 创建/编辑弹窗 -->
{#if showModal}
	<div class="modal-backdrop" onclick={closeModal} role="presentation"></div>
	<div class="modal-card">
		<div class="modal-header">
			<h3>{editingTheme ? '编辑主题' : '新建主题'}</h3>
			<button class="modal-close" onclick={closeModal}>×</button>
		</div>
		<div class="modal-body">
			<div class="form-group">
				<label for="theme-name">主题名称 *</label>
				<input type="text" id="theme-name" bind:value={formName} oninput={handleNameInput} placeholder="例如：前端开发" />
			</div>
			<div class="form-group">
				<label for="theme-slug">URL Slug *</label>
				<input type="text" id="theme-slug" bind:value={formSlug} oninput={handleSlugInput} placeholder="自动生成，可手动修改" />
			</div>
			<div class="form-group">
				<label for="theme-desc">描述</label>
				<textarea id="theme-desc" bind:value={formDescription} rows="3" placeholder="简单介绍一下这个主题..."></textarea>
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
				<p class="confirm-title">删除主题「{deleteTarget.name}」？</p>
				<p class="confirm-desc">
					{#if (deleteTarget.post_count ?? 0) > 0}
						该主题下有 {deleteTarget.post_count} 篇文章，删除后这些文章将不属于任何主题。
					{:else}
						此操作不可撤销。
					{/if}
				</p>
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
	.theme-card {
		background: var(--color-white);
		border-radius: 20px;
		padding: 1.5rem 1.75rem;
		box-shadow: var(--shadow-sm);
		border: 1px solid var(--color-divider);
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
	}

	.theme-card:hover {
		box-shadow: var(--shadow-blue-md);
		border-color: var(--blue-alpha-30);
		transform: translateY(-2px);
	}

	.theme-header {
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
	}

	.theme-name {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: var(--color-text);
	}

	.theme-slug {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
	}

	.theme-desc {
		margin: 0;
		font-size: 0.8125rem;
		color: var(--color-text-light);
		line-height: 1.6;
	}

	.theme-no-desc {
		margin: 0;
		font-size: 0.8125rem;
		color: var(--color-text-muted);
		font-style: italic;
	}

	.theme-stats {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.theme-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 0.75rem;
		border-top: 1px solid var(--color-divider);
	}
</style>
