// ================================
// JWT 认证管理（管理端使用）
// ================================

const TOKEN_KEY = 'yukilog_token';
const TOKEN_EXPIRY_KEY = 'yukilog_token_expiry';

/**
 * 保存 JWT Token
 */
export function setToken(token: string, expiresIn: number): void {
  if (typeof window === 'undefined') return;

  const expiryTime = Date.now() + expiresIn * 1000;

  localStorage.setItem(TOKEN_KEY, token);
  localStorage.setItem(TOKEN_EXPIRY_KEY, expiryTime.toString());
}

/**
 * 获取 JWT Token
 */
export function getToken(): string | null {
  if (typeof window === 'undefined') return null;

  const token = localStorage.getItem(TOKEN_KEY);
  const expiry = localStorage.getItem(TOKEN_EXPIRY_KEY);

  if (!token || !expiry) return null;

  // 检查是否过期
  if (Date.now() > parseInt(expiry)) {
    clearToken();
    return null;
  }

  return token;
}

/**
 * 清除 JWT Token
 */
export function clearToken(): void {
  if (typeof window === 'undefined') return;

  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(TOKEN_EXPIRY_KEY);
}

/**
 * 检查是否已登录
 */
export function isAuthenticated(): boolean {
  return getToken() !== null;
}

/**
 * Token 是否即将过期（小于 5 分钟）
 */
export function isTokenExpiring(): boolean {
  if (typeof window === 'undefined') return false;

  const expiry = localStorage.getItem(TOKEN_EXPIRY_KEY);
  if (!expiry) return false;

  const timeLeft = parseInt(expiry) - Date.now();
  const fiveMinutes = 5 * 60 * 1000;

  return timeLeft < fiveMinutes && timeLeft > 0;
}

/**
 * 获取 Token 剩余时间（秒）
 */
export function getTokenTimeLeft(): number {
  if (typeof window === 'undefined') return 0;

  const expiry = localStorage.getItem(TOKEN_EXPIRY_KEY);
  if (!expiry) return 0;

  const timeLeft = parseInt(expiry) - Date.now();
  return Math.max(0, Math.floor(timeLeft / 1000));
}

/**
 * 开发模式：设置 Mock Token（绕过真实登录）
 * ⚠️ 仅用于开发阶段，生产环境需删除此函数
 */
export function setDevToken(): void {
  if (typeof window === 'undefined') return;
  
  // 设置一个 24 小时有效的 mock token
  const mockToken = 'dev_mock_token_' + Date.now();
  const expiresIn = 24 * 60 * 60; // 24 小时
  
  setToken(mockToken, expiresIn);
  console.warn('🔓 开发模式：已设置 Mock Token，有效期 24 小时');
}

