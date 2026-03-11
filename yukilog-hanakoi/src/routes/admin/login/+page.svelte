<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { authApi } from '$lib/api';
	import { setToken, isAuthenticated } from '$lib/auth';

	let username = $state('');
	let password = $state('');
	let error = $state('');
	let loading = $state(false);

	onMount(() => {
		if (isAuthenticated()) {
			goto('/admin');
		}
	});

	async function handleLogin(e: Event) {
		e.preventDefault();
		error = '';
		loading = true;

		try {
			const res = await authApi.login({ username, password });
			setToken(res.token, res.expires_in);
			goto('/admin');
		} catch (err: any) {
			error = err.message || '登录失败，请重试';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>登录 - YukiLog 管理后台</title>
</svelte:head>

<div class="login-page">
	<div class="login-card">
		<div class="login-header">
			<h1 class="login-logo">YukiLog</h1>
			<p class="login-subtitle">管理后台</p>
		</div>

		<form class="login-form" onsubmit={handleLogin}>
			{#if error}
				<div class="login-error">{error}</div>
			{/if}

			<div class="form-group">
				<label for="username">用户名</label>
				<input
					type="text"
					id="username"
					bind:value={username}
					placeholder="请输入用户名"
					required
					autocomplete="username"
				/>
			</div>

			<div class="form-group">
				<label for="password">密码</label>
				<input
					type="password"
					id="password"
					bind:value={password}
					placeholder="请输入密码"
					required
					autocomplete="current-password"
				/>
			</div>

			<button type="submit" class="login-btn" disabled={loading}>
				{loading ? '登录中...' : '登录'}
			</button>
		</form>
	</div>
</div>

<style>
	.login-page {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-bg);
		padding: 2rem;
	}

	.login-card {
		width: 100%;
		max-width: 400px;
		background: var(--color-white);
		border-radius: 24px;
		box-shadow: var(--shadow-lg);
		overflow: hidden;
	}

	.login-header {
		padding: 2.5rem 2rem 1.5rem;
		text-align: center;
		background: var(--gradient-blue-pink);
	}

	.login-logo {
		font-size: 2rem;
		font-weight: 700;
		background: linear-gradient(135deg, var(--color-blue) 0%, var(--color-pink) 100%);
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		margin: 0;
	}

	.login-subtitle {
		font-size: 0.875rem;
		color: var(--color-text-muted);
		margin: 0.5rem 0 0;
	}

	.login-form {
		padding: 2rem;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.login-error {
		padding: 0.75rem 1rem;
		font-size: 0.875rem;
		color: var(--color-error);
		background: var(--error-alpha-08);
		border: 1px solid var(--error-alpha-20);
		border-radius: 12px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-group label {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--color-text);
	}

	.form-group input {
		padding: 0.75rem 1rem;
		font-size: 0.9375rem;
		color: var(--color-text);
		background: var(--color-white);
		border: 1px solid var(--color-border);
		border-radius: 12px;
		transition: border-color 200ms, box-shadow 200ms;
	}

	.form-group input:focus {
		outline: none;
		border-color: var(--color-blue);
		box-shadow: 0 0 0 3px var(--blue-alpha-15);
	}

	.login-btn {
		margin-top: 0.5rem;
		padding: 0.875rem;
		font-size: 0.9375rem;
		font-weight: 600;
		color: var(--color-on-primary);
		background: linear-gradient(135deg, var(--color-blue) 0%, var(--color-pink) 100%);
		border: none;
		border-radius: 16px;
		cursor: pointer;
		transition: all 300ms cubic-bezier(0.4, 0, 0.2, 1);
	}

	.login-btn:hover:not(:disabled) {
		transform: translateY(-1px);
		box-shadow: var(--shadow-blue-md);
	}

	.login-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}
</style>
