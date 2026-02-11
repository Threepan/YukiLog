// ================================
// YukiLog 后端 API 响应类型
// 对应后端统一响应格式
// ================================

/**
 * 后端统一响应格式
 */
export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  message: string | null;
}

/**
 * 分页数据响应
 */
export interface PaginatedData<T> {
  items: T[];
  total: number;
  page: number;
  page_size: number;
  total_pages: number;
}

// ================================
// 主题（Themes）
// ================================

export interface Theme {
  id: number;
  name: string;
  slug: string;
  description: string | null;
  post_count: number;
  view_count: number;
  created_at: string;
}

// ================================
// 标签（Tags）
// ================================

export interface Tag {
  id: number;
  name: string;
  slug: string;
  post_count: number;
  view_count: number;
  created_at: string;
}

// ================================
// 文章（Posts）
// ================================

export type PostStatus = 'draft' | 'published';

export interface Post {
  id: number;
  title: string;
  slug: string;
  summary: string | null;
  content: string;
  cover_image: string | null;
  status: PostStatus;
  theme_id: number | null;
  view_count: number;
  created_at: string;
  updated_at: string;
}

/**
 * 创建文章请求体
 */
export interface CreatePostRequest {
  title: string;
  slug: string;
  content: string;
  summary?: string | null;
  cover_image?: string | null;
  status: PostStatus;
  theme_slug?: string | null;
  tag_slugs?: string[];
}

/**
 * 更新文章请求体
 */
export interface UpdatePostRequest {
  title?: string;
  slug?: string;
  content?: string;
  summary?: string | null;
  cover_image?: string | null;
  status?: PostStatus;
  theme_slug?: string | null;
  tag_slugs?: string[];
}

/**
 * 文章列表查询参数
 */
export interface PostListParams {
  page?: number;
  page_size?: number;
  sort?: 'created_at' | 'updated_at' | 'view_count';
  status?: PostStatus; // 仅管理端使用
  theme_slugs?: string;  // 逗号分隔
  tag_slugs?: string;    // 逗号分隔，AND 关系
}

// ================================
// 评论（Comments）
// ================================

export type CommentStatus = 'approved' | 'pending' | 'spam';

export interface Comment {
  id: number;
  post_id: number;
  content: string;
  guest_nick: string;
  guest_email: string;
  guest_website: string | null;
  parent_id: number | null;
  root_id: number | null;
  status: CommentStatus;
  ip: string | null;
  ua: string | null;
  created_at: string;
}

/**
 * 评论树节点（包含子评论）
 */
export interface CommentNode {
  comment: Comment;
  children: CommentNode[];
}

/**
 * 创建评论请求体
 */
export interface CreateCommentRequest {
  nickname: string;
  email: string;
  content: string;
  parent_id?: number | null;
  website?: string | null;
}

/**
 * 创建评论响应
 */
export interface CreateCommentResponse {
  id: number;
  avatar_url: string;
  created_at: string;
}

/**
 * 更新评论请求体（管理端）
 */
export interface UpdateCommentRequest {
  content?: string;
  guest_nick?: string;
  guest_email?: string;
  guest_website?: string | null;
}

/**
 * 评论列表查询参数（管理端）
 */
export interface CommentListParams {
  page?: number;
  page_size?: number;
  sort?: 'created_at_asc' | 'created_at_desc';
  post_slug?: string;
}

// ================================
// 友链（Links）
// ================================

export type LinkStatus = 'active' | 'pending' | 'broken';

export interface Link {
  id: number;
  title: string;
  url: string;
  avatar: string | null;
  description: string | null;
  status: LinkStatus;
  created_at: string;
}

/**
 * 提交友链请求体
 */
export interface SubmitLinkRequest {
  title: string;
  url: string;
  avatar?: string | null;
  description?: string | null;
}

/**
 * 提交友链响应
 */
export interface SubmitLinkResponse {
  id: number;
  message: string;
}

/**
 * 更新友链请求体（管理端）
 */
export interface UpdateLinkRequest {
  title?: string;
  url?: string;
  avatar?: string | null;
  description?: string | null;
}

// ================================
// 认证（Auth）
// ================================

export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  expires_in: number; // 秒
}
