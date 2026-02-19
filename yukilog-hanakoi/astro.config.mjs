// @ts-check
import { defineConfig } from 'astro/config';
import vue from '@astrojs/vue';
import tailwindcss from '@tailwindcss/vite';
import node from '@astrojs/node';

// https://astro.build/config
export default defineConfig({
  output: 'static',  // 默认静态，admin 页面通过 prerender: false 标记为动态
  adapter: node({
    mode: 'standalone'
  }),
  
  integrations: [vue()],

  vite: {
    plugins: [tailwindcss()],

    server: {
      allowedHosts: true,
    },
  },
  
  // SEO 友好的 URL（移除尾随斜杠）
  trailingSlash: 'never',
  
  // 构建输出目录
  outDir: './dist',
  
  // 服务器配置
  server: {
    port: 4321,
    host: true,
  },
});
