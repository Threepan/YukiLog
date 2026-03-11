<script lang="ts">
	import { onMount } from 'svelte';
	import { tagsApi, adminApi } from '$lib/api';
	import { generateSlug } from '$lib/slugify';
	import type { Tag } from '$types';

	let tags: Tag[] = $state([]);
	let loading = $state(true);
	let error = $state('');

	// 编辑弹窗
	let showModal = $state(false);
	let editingTag: Tag | null = $state(null);
	let formName = $state('');
	let formSlug = $state('');
	let slugManuallyEdited = $state(false);
	let submitting = $state(false);

	// 删除确认
	let deleteTarget: Tag | null = $state(null);
	let deleting = $state(false);

	// 合并功能
	let mergeMode = $state(false);
	let selectedForMerge: Set<number> = $state(new Set());
	let showMergeModal = $state(false);
	let mergeTargetId = $state<number | ''>('');
	let merging = $state(false);

	const selectedTags = $derived(tags.filter(t => selectedForMerge.has(t.id)));
	const mergeTargetOptions = $derived(tags.filter(t => !selectedForMerge.has(t.id)));

	async function loadTags() {
		loading = true;
		error = '';
		try {
			tags = await tagsApi.list();
		} catch (err: any) {
			error = err.message || '加载失败';
		} finally {
			loading = false;
		}
	}

	function openCreateModal() {
		editingTag = null;
		formName = '';
		formSlug = '';
		slugManuallyEdited = false;
		showModal = true;
	}

	function openEditModal(tag: Tag) {
		editingTag = tag;
		formName = tag.name;
		formSlug = tag.slug;
		slugManuallyEdited = true;
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingTag = null;
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
			const data = { name: formName.trim(), slug: formSlug.trim() };
			if (editingTag) {
				await adminApi.tags.update(editingTag.id, data);
			} else {
				await adminApi.tags.create(data);
			}
			closeModal();
			await loadTags();
		} catch (err: any) {
			alert(err.message || '操作失败');
		} finally {
			submitting = false;
		}
	}

	function confirmDelete(tag: Tag) {
		deleteTarget = tag;
	}

	async function executeDelete() {
		if (!deleteTarget) return;
		deleting = true;
		try {
			await adminApi.tags.delete(deleteTarget.id);
			deleteTarget = null;
			await loadTags();
		} catch (err: any) {
			alert(err.message || '删除失败');
		} finally {
			deleting = false;
		}
	}

	// 合并相关
	function toggleMergeMode() {
		mergeMode = !mergeMode;
		if (!mergeMode) {
			selectedForMerge = new Set();
		}
	}

	function toggleMergeSelection(id: number) {
		const next = new Set(selectedForMerge);
		if (next.has(id)) next.delete(id);
		else next.add(id);
		selectedForMerge = next;
	}

	function openMergeModal() {
		if (selectedForMerge.size < 2) { alert('请至少选择 2 个标签进行合并'); return; }
		mergeTargetId = '';
		showMergeModal = true;
	}

	function closeMergeModal() {
		showMergeModal = false;
	}

	async function executeMerge() {
		if (!mergeTargetId) { alert('请选择目标标签'); return; }
		merging = true;
		try {
			const sourceIds = Array.from(selectedForMerge).filter(id => id !== mergeTargetId);
			// 逐个合并到目标标签
			for (const sourceId of sourceIds) {
				await adminApi.tags.merge({
					source_id: Number(sourceId),
					target_id: Number(mergeTargetId),
				});
			}
			closeMergeModal();
			mergeMode = false;
			selectedForMerge = new Set();
			await loadTags();
		} catch (err: any) {
			alert(err.message || '合并失败');
		} finally {
			merging = false;
		}
	}

	onMount(() => { loadTags(); });
</script>

<svelte:head>
	<title>标签管理 - YukiLog</title>
</svelte:head>

<!-- 操作栏 -->
<div class="action-bar">
	<div class="tabs">
		<button class="tab active">全部标签</button>
		<button class="tab" class:active={mergeMode} onclick={toggleMergeMode}>
			{mergeMode ? '取消合并' : '合并模式'}
		</button>
	</div>
	<div class="action-btns">
		{#if mergeMode && selectedForMerge.size >= 2}
			<button class="btn-merge" onclick={openMergeModal}>
				合并 ({selectedForMerge.size}) 个标签
			</button>
		{/if}
		<button class="btn-create" onclick={openCreateModal}>
			<span>+</span> 新建标签
		</button>
	</div>
</div>

<!-- 内容区 -->
{#if loading}
	<div class="admin-loading"><div class="spinner"></div><p>加载中...</p></div>
{:else if error}
	<div class="admin-error"><p>{error}</p><button class="btn-retry" onclick={loadTags}>重新加载</button></div>
{:else if tags.length === 0}
	<div class="admin-empty"><p>暂无标签</p></div>
{:else}
	<div class="admin-card-grid">
		{#each tags as tag (tag.id)}
			<div class="tag-card" class:selected={selectedForMerge.has(tag.id)}>
				{#if mergeMode}
					<label class="merge-checkbox">
						<input type="checkbox" checked={selectedForMerge.has(tag.id)} onchange={() => toggleMergeSelection(tag.id)} />
					</label>
				{/if}
				<div class="tag-header">
					<h3 class="tag-name">#{tag.name}</h3>
					<span class="tag-slug">/{tag.slug}</span>
				</div>
				<div class="tag-stats">
					<span>📝 {tag.post_count ?? 0} 篇文章</span>
					<span>👁 {tag.view_count ?? 0} 浏览</span>
				</div>
				<div class="tag-actions">
					<button class="action-btn edit" onclick={() => openEditModal(tag)}>编辑</button>
					<button class="action-btn delete" onclick={() => confirmDelete(tag)}>删除</button>
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
			<h3>{editingTag ? '编辑标签' : '新建标签'}</h3>
			<button class="modal-close" onclick={closeModal}>×</button>
		</div>
		<div class="modal-body">
			<div class="form-group">
				<label for="tag-name">标签名称 *</label>
				<input type="text" id="tag-name" bind:value={formName} oninput={handleNameInput} placeholder="例如：Rust" />
			</div>
			<div class="form-group">
				<label for="tag-slug">URL Slug *</label>
				<input type="text" id="tag-slug" bind:value={formSlug} oninput={handleSlugInput} placeholder="自动生成，可手动修改" />
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
				<p class="confirm-title">删除标签「#{deleteTarget.name}」？</p>
				<p class="confirm-desc">
					{#if (deleteTarget.post_count ?? 0) > 0}
						该标签关联了 {deleteTarget.post_count} 篇文章，删除后这些文章将不再包含此标签。
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

<!-- 合并弹窗 -->
{#if showMergeModal}
	<div class="modal-backdrop" onclick={closeMergeModal} role="presentation"></div>
	<div class="modal-card">
		<div class="modal-header">
			<h3>合并标签</h3>
			<button class="modal-close" onclick={closeMergeModal}>×</button>
		</div>
		<div class="modal-body">
			<div class="merge-info">
				<p class="merge-label">将以下标签合并：</p>
				<div class="merge-sources">
					{#each selectedTags as tag}
						<span class="merge-tag">#{tag.name}</span>
					{/each}
				</div>
			</div>
			<div class="form-group">
				<label for="merge-target">合并到目标标签 *</label>
				<select id="merge-target" bind:value={mergeTargetId}>
					<option value="">请选择目标标签</option>
					{#each mergeTargetOptions as tag}
						<option value={tag.id}>#{tag.name} ({tag.post_count ?? 0} 篇文章)</option>
					{/each}
				</select>
				<span class="form-hint">所选标签的文章关联将合并到目标标签，源标签将被删除</span>
			</div>
		</div>
		<div class="modal-footer">
			<button class="btn-secondary" onclick={closeMergeModal}>取消</button>
			<button class="btn-primary" disabled={merging} onclick={executeMerge}>
				{merging ? '合并中...' : '确认合并'}
			</button>
		</div>
	</div>
{/if}

<style>
	.action-btns {
		display: flex;
		gap: 0.75rem;
	}

	.btn-merge {
		padding: 1rem 2rem;
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--color-on-primary);
		background: linear-gradient(135deg, #e8984a 0%, #f0b070 100%);
		border: none;
		border-radius: 16px;
		cursor: pointer;
		transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
	}

	.btn-merge:hover {
		transform: translateY(-1px);
		box-shadow: 0 4px 12px rgba(232, 152, 74, 0.3);
	}

	.tag-card {
		background: var(--color-white);
		border-radius: 20px;
		padding: 1.5rem 1.75rem;
		box-shadow: var(--shadow-sm);
		border: 1px solid var(--color-divider);
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
		position: relative;
	}

	.tag-card:hover {
		box-shadow: var(--shadow-blue-md);
		border-color: var(--blue-alpha-30);
		transform: translateY(-2px);
	}

	.tag-card.selected {
		border-color: var(--color-pink);
		box-shadow: 0 0 0 2px var(--pink-alpha-25);
	}

	.merge-checkbox {
		position: absolute;
		top: 1rem;
		right: 1rem;
		cursor: pointer;
	}

	.merge-checkbox input {
		width: 18px;
		height: 18px;
		accent-color: var(--color-pink);
		cursor: pointer;
	}

	.tag-header {
		display: flex;
		align-items: baseline;
		gap: 0.5rem;
	}

	.tag-name {
		margin: 0;
		font-size: 1rem;
		font-weight: 600;
		color: var(--color-pink);
	}

	.tag-slug {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
	}

	.tag-stats {
		display: flex;
		gap: 1rem;
		font-size: 0.75rem;
		color: var(--color-text-muted);
	}

	.tag-actions {
		display: flex;
		gap: 0.5rem;
		padding-top: 0.75rem;
		border-top: 1px solid var(--color-divider);
	}

	/* 合并弹窗 */
	.merge-info {
		margin-bottom: 1.25rem;
	}

	.merge-label {
		font-size: 0.875rem;
		color: var(--color-text);
		margin: 0 0 0.5rem;
	}

	.merge-sources {
		display: flex;
		flex-wrap: wrap;
		gap: 0.375rem;
	}

	.merge-tag {
		padding: 0.25rem 0.75rem;
		font-size: 0.8125rem;
		color: var(--color-pink);
		background: var(--pink-alpha-08);
		border-radius: 12px;
	}
</style>
