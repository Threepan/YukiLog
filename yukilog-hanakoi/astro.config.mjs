// @ts-check
import { defineConfig } from 'astro/config';
import vue from '@astrojs/vue';
import tailwindcss from '@tailwindcss/vite';

// https://astro.build/config
export default defineConfig({
  integrations: [vue()],

  vite: {
    plugins: [tailwindcss()],
    css: {
      preprocessorOptions: {
        scss: {
          // 自动注入变量文件到所有 SCSS 文件
          additionalData: `@use "/src/styles/variables.scss" as *;`,
        },
      },
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