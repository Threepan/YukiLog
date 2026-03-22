<script lang="ts">
	import { onMount } from 'svelte';
	import { adminApi } from '$lib/api';
	import { formatDate } from '$lib/date';
	import { contentConfig } from '$lib/config';
	import type { Note } from '$types';

	const PAGE_SIZE = 20;
	const moodMap: Record<string, string> = contentConfig.pages.notes.moodLabels as Record<string, string>;
	const moodEntries = Object.entries(moodMap);

	let notes: Note[] = $state([]);
	let totalCount = $state(0);
	let loading = $state(true);
	let error = $state('');
	let currentStatus: string = $state('all');
	let currentPage = $state(1);

	// 编辑弹窗
	let showModal = $state(false);
	let editingNote: Note | null = $state(null);
	let formContent = $state('');
	let formMood = $state('');
	let formStatus = $state<'published' | 'draft' | 'private'>('published');
	let submitting = $state(false);

	// mood combobox
	let showMoodDropdown = $state(false);
	const filteredMoods = $derived(
		formMood.trim()
			? moodEntries.filter(([k, v]) => k.includes(formMood.toLowerCase()) || v.includes(formMood))
			: moodEntries
	);

	// 删除确认
	let deleteTarget: Note | null = $state(null);
	let deleting = $state(false);

	const totalPages = $derived(Math.max(1, Math.ceil(totalCount / PAGE_SIZE)));

	const statusTabs: { key: string; label: string }[] = [
		{ key: 'all', label: '全部' },
		{ key: 'published', label: '已发布' },
		{ key: 'draft', label: '草稿' },
		{ key: 'private', label: '私密' },
	];

	async function loadNotes() {
		loading = true;
		error = '';
		try {
			const params: any = { page: currentPage, page_size: PAGE_SIZE };
			if (currentStatus !== 'all') params.status = currentStatus;
			const res = await adminApi.notes.list(params);
			notes = res.items;
			totalCount = res.total;
		} catch (err: any) {
			error = err.message || '加载失败';
		} finally {
			loading = false;
		}
	}

	function switchTab(status: string) {
		currentStatus = status;
		currentPage = 1;
		loadNotes();
	}

	function openCreateModal() {
		editingNote = null;
		formContent = '';
		formMood = '';
		formStatus = 'published';
		showModal = true;
	}

	function openEditModal(note: Note) {
		editingNote = note;
		formContent = note.content;
		formMood = note.mood || '';
		formStatus = note.status as any;
		showModal = true;
	}

	function closeModal() {
		showModal = false;
		editingNote = null;
	}

	async function handleSubmit() {
		if (!formContent.trim()) { alert('请输入内容'); return; }
		submitting = true;
		try {
			const data = {
				content: formContent.trim(),
				mood: formMood || undefined,
				status: formStatus,
			};
			if (editingNote) {
				await adminApi.notes.update(editingNote.id, data);
			} else {
				await adminApi.notes.create(data);
			}
			closeModal();
			await loadNotes();
		} catch (err: any) {
			alert(err.message || '操作失败');
		} finally {
			submitting = false;
		}
	}

	function confirmDelete(note: Note) {
		deleteTarget = note;
	}

	async function executeDelete() {
		if (!deleteTarget) return;
		deleting = true;
		try {
			await adminApi.notes.delete(deleteTarget.id);
			deleteTarget = null;
			await loadNotes();
		} catch (err: any) {
			alert(err.message || '删除失败');
		} finally {
			deleting = false;
		}
	}

	function getStatusText(status: string): string {
		const map: Record<string, string> = { published: '已发布', draft: '草稿', private: '私密' };
		return map[status] || status;
	}

	onMount(() => { loadNotes(); });
</script>

<svelte:head>
	<title>随记管理 - YukiLog</title>
</svelte:head>

<!-- 操作栏 -->
<div class="action-bar">
	<div class="tabs">
		{#each statusTabs as tab}
			<button class="tab" class:active={currentStatus === tab.key} onclick={() => switchTab(tab.key)}>{tab.label}</button>
		{/each}
	</div>
	<button class="btn-create" onclick={openCreateModal}>
		<span>+</span> 新建随记
	</button>
</div>

<!-- 内容区 -->
{#if loading}
	<div class="admin-loading"><div class="spinner"></div><p>加载中...</p></div>
{:else if error}
	<div class="admin-error"><p>{error}</p><button class="btn-retry" onclick={loadNotes}>重新加载</button></div>
{:else if notes.length === 0}
	<div class="admin-empty"><p>暂无随记</p></div>
{:else}
	<div class="notes-list">
		{#each notes as note (note.id)}
			<div class="note-card">
				<div class="note-content">{note.content}</div>
				<div class="note-footer">
					<div class="note-meta">
						<span class="status-badge status-{note.status}">{getStatusText(note.status)}</span>
						{#if note.mood && moodMap[note.mood]}
							<span class="note-mood">{moodMap[note.mood]}</span>
						{/if}
						<span class="note-date">{formatDate(note.created_at)}</span>
					</div>
					<div class="note-actions">
						<button class="action-btn edit" onclick={() => openEditModal(note)}>编辑</button>
						<button class="action-btn delete" onclick={() => confirmDelete(note)}>删除</button>
					</div>
				</div>
			</div>
		{/each}
	</div>

	{#if totalPages > 1}
		<div class="pagination">
			<button class="page-btn" disabled={currentPage === 1} onclick={() => { currentPage--; loadNotes(); }}>上一页</button>
			<span class="page-info">第 {currentPage} / {totalPages} 页</span>
			<button class="page-btn" disabled={currentPage === totalPages} onclick={() => { currentPage++; loadNotes(); }}>下一页</button>
		</div>
	{/if}
{/if}

<!-- 创建/编辑弹窗 -->
{#if showModal}
	<div class="modal-backdrop" onclick={closeModal} role="presentation"></div>
	<div class="modal-card">
		<div class="modal-header">
			<h3>{editingNote ? '编辑随记' : '新建随记'}</h3>
			<button class="modal-close" onclick={closeModal}>×</button>
		</div>
		<div class="modal-body">
			<div class="form-group">
				<label for="note-content">内容 *</label>
				<textarea id="note-content" bind:value={formContent} rows="6" placeholder="写点什么..."></textarea>
			</div>
			<div class="form-group">
				<label for="note-mood">心情</label>
				<div class="mood-combobox">
					<input
						id="note-mood"
						type="text"
						bind:value={formMood}
						placeholder="搜索或输入自定义心情…"
						autocomplete="off"
						onfocus={() => (showMoodDropdown = true)}
						onblur={() => setTimeout(() => (showMoodDropdown = false), 150)}
					/>
					{#if showMoodDropdown}
						<div class="mood-dropdown">
							<button type="button" class="mood-option" onmousedown={() => { formMood = ''; showMoodDropdown = false; }}>无</button>
							{#each filteredMoods as [key, label]}
								<button type="button" class="mood-option" onmousedown={() => { formMood = key; showMoodDropdown = false; }}>{label}</button>
							{/each}
							{#if filteredMoods.length === 0}
								<div class="mood-empty">按 Enter 使用自定义值</div>
							{/if}
						</div>
					{/if}
				</div>
			</div>
			<div class="form-group">
				<label for="note-status">状态</label>
				<select id="note-status" bind:value={formStatus}>
					<option value="published">已发布</option>
					<option value="draft">草稿</option>
					<option value="private">私密</option>
				</select>
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
				<p class="confirm-title">确认删除这条随记？</p>
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
	.notes-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.note-card {
		background: var(--color-white);
		border-radius: 16px;
		padding: 1.25rem 1.5rem;
		box-shadow: var(--shadow-sm);
		border: 1px solid var(--color-divider);
		transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
	}

	.note-card:hover {
		box-shadow: var(--shadow-blue-md);
		border-color: var(--blue-alpha-30);
	}

	.note-content {
		font-size: 0.9375rem;
		color: var(--color-text);
		line-height: 1.7;
		white-space: pre-wrap;
		word-break: break-word;
		margin-bottom: 1rem;
	}

	.note-footer {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding-top: 0.75rem;
		border-top: 1px solid var(--color-divider);
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.note-meta {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.note-mood {
		font-size: 0.75rem;
		color: var(--color-text-light);
	}

	.mood-combobox {
		position: relative;
	}

	.mood-combobox input {
		width: 100%;
		box-sizing: border-box;
	}

	.mood-dropdown {
		position: absolute;
		top: calc(100% + 4px);
		left: 0;
		right: 0;
		max-height: 220px;
		overflow-y: auto;
		background: var(--color-white);
		border: 1px solid var(--color-divider);
		border-radius: 10px;
		box-shadow: var(--shadow-sm);
		z-index: 100;
		display: flex;
		flex-direction: column;
		padding: 4px;
		gap: 2px;
	}

	.mood-option {
		text-align: left;
		background: none;
		border: none;
		border-radius: 6px;
		padding: 0.4rem 0.6rem;
		font-size: 0.875rem;
		cursor: pointer;
		color: var(--color-text);
		transition: background 150ms;
	}

	.mood-option:hover {
		background: var(--color-bg);
	}

	.mood-empty {
		padding: 0.4rem 0.6rem;
		font-size: 0.8125rem;
		color: var(--color-text-muted);
	}

	.note-date {
		font-size: 0.75rem;
		color: var(--color-text-muted);
		font-family: 'JetBrains Mono', 'Fira Code', monospace;
	}

	.note-actions {
		display: flex;
		gap: 0.5rem;
	}
</style>
