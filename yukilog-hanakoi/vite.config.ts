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
	},
	build: {
		rollupOptions: {
			output: {
				manualChunks(id) {
					if (id.includes('node_modules/mermaid') || id.includes('node_modules/dagre') ||
						id.includes('node_modules/cytoscape') || id.includes('node_modules/d3') ||
						id.includes('node_modules/elkjs')) {
						return 'vendor-mermaid';
					}
					if (id.includes('node_modules/shiki') || id.includes('node_modules/@shikijs')) {
						return 'vendor-shiki';
					}
					if (id.includes('node_modules/katex')) {
						return 'vendor-katex';
					}
				}
			}
		}
	}
});
