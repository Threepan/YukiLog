import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port: 4321,
		host: true,
		fs: {
			allow: ['..']
		}
	}
});
