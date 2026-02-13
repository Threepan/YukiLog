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


