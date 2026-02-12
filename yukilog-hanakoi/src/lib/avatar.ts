/**
 * 生成评论头像 URL
 * 优先级：① website favicon → ② Gravatar → ③ 默认头像
 */

import { getGravatarUrl } from './utils';

const DEFAULT_AVATAR = 'https://www.gravatar.com/avatar/00000000000000000000000000000000?s=80&d=mp';

/**
 * 根据 website 和 email 生成头像 URL
 * @param website - 网站 URL（可选）
 * @param email - 邮箱地址
 * @returns 头像 URL
 */
export function getCommentAvatar(website: string | null, email: string): string {
  // ① 优先使用 website favicon
  if (website) {
    const trimmedUrl = website.trim().replace(/\/+$/, '');
    return `${trimmedUrl}/favicon.ico`;
  }
  
  // ② 使用邮箱生成 Gravatar
  if (email && email.trim()) {
    return getGravatarUrl(email, 80);
  }
  
  // ③ 默认头像
  return DEFAULT_AVATAR;
}
