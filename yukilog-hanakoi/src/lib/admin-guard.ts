// ================================
// Admin 路由守卫
// 所有后台页面在 <script> 中导入此函数
// ================================

import { isAuthenticated, isTokenExpiring, clearToken } from './auth';

/**
 * 检查登录状态并守卫路由
 * 未登录 → 跳转登录页
 * Token 即将过期 → 提示续期（可选）
 */
export function guardAdminRoute(): void {
  if (typeof window === 'undefined') return;

  // 未登录，跳转到登录页
  if (!isAuthenticated()) {
    const loginUrl = '/admin/login';
    if (window.location.pathname !== loginUrl) {
      window.location.href = loginUrl;
    }
    return;
  }

  // Token 即将过期提示（可选功能）
  if (isTokenExpiring()) {
    // TODO: 显示 toast 提示用户 token 即将过期
  }
}

/**
 * 管理员登出
 */
export function logout(): void {
  clearToken();
  window.location.href = '/admin/login';
}
