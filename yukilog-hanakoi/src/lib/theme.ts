/**
 * 主题切换工具函数
 */

export type Theme = 'light' | 'dark';

const THEME_KEY = 'yukilog-theme';

/**
 * 获取当前主题
 */
export function getTheme(): Theme {
  if (typeof window === 'undefined') return 'light';
  
  const stored = localStorage.getItem(THEME_KEY);
  if (stored === 'dark' || stored === 'light') {
    return stored;
  }
  
  // 如果没有存储，检测系统偏好
  if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark';
  }
  
  return 'light';
}

/**
 * 设置主题
 */
export function setTheme(theme: Theme): void {
  if (typeof window === 'undefined') return;
  
  document.documentElement.setAttribute('data-theme', theme);
  localStorage.setItem(THEME_KEY, theme);
  
  // 触发自定义事件，便于其他组件监听
  window.dispatchEvent(new CustomEvent('theme-change', { detail: { theme } }));
}

/**
 * 切换主题
 */
export function toggleTheme(): Theme {
  const current = getTheme();
  const next = current === 'light' ? 'dark' : 'light';
  setTheme(next);
  return next;
}

/**
 * 初始化主题（避免闪烁）
 * 这个函数应该在页面加载时尽早执行
 */
export function initTheme(): void {
  const theme = getTheme();
  document.documentElement.setAttribute('data-theme', theme);
}
