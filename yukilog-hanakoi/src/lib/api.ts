// ================================
// YukiLog API 调用封装
// 基于后端 RESTful API
// ================================

import type {
  ApiResponse,
  PaginatedData,
  Post,
  PostWithRelations,
  PostListParams,
  CreatePostRequest,
  UpdatePostRequest,
  SearchQuery,
  Theme,
  CreateThemeRequest,
  UpdateThemeRequest,
  Tag,
  CreateTagRequest,
  UpdateTagRequest,
  MergeTagsRequest,
  CommentNode,
  Comment,
  CommentListParams,
  CreateCommentRequest,
  CreateCommentResponse,
  UpdateCommentRequest,
  Link,
  SubmitLinkRequest,
  SubmitLinkResponse,
  UpdateLinkRequest,
  LoginRequest,
  LoginResponse,
} from '../types';

// API 基础地址（从环境变量读取，默认本地）
const API_BASE = import.meta.env.PUBLIC_API_URL || 'http://localhost:3000';

/**
 * 通用 fetch 封装
 */
async function fetchApi<T>(
  endpoint: string,
  options?: RequestInit
): Promise<T> {
  const response = await fetch(`${API_BASE}${endpoint}`, {
    headers: {
      'Content-Type': 'application/json',
      ...options?.headers,
    },
    ...options,
  });

  if (!response.ok) {
    throw new Error(`API Error: ${response.status} ${response.statusText}`);
  }

  const result: ApiResponse<T> = await response.json();

  if (!result.success) {
    throw new Error(result.message || 'API 请求失败');
  }

  return result.data!;
}

// ================================
// 文章 API
// ================================

export const postsApi = {
  /**
   * 获取文章列表（支持分页、筛选、排序）
   */
  async list(params?: PostListParams): Promise<PaginatedData<PostWithRelations>> {
    const query = new URLSearchParams(params as any).toString();
    return fetchApi<PaginatedData<PostWithRelations>>(`/api/public/posts?${query}`);
  },

  /**
   * 获取文章详情
   */
  async getBySlug(slug: string): Promise<PostWithRelations> {
    return fetchApi<PostWithRelations>(`/api/public/posts/${slug}`);
  },

  /**
   * 增加文章浏览计数（无需返回）
   */
  async incrementView(slug: string): Promise<void> {
    await fetch(`${API_BASE}/api/public/posts/${slug}/view`, {
      method: 'POST',
    });
  },

  /**
   * 全文搜索文章
   * 搜索结果中 title/summary/content 的关键词会被 <mark> 标签包裹
   * content 被截取为关键词附近的摘要
   */
  async search(params: SearchQuery): Promise<PaginatedData<PostWithRelations>> {
    const query = new URLSearchParams({
      q: params.q,
      ...(params.page && { page: String(params.page) }),
      ...(params.page_size && { page_size: String(params.page_size) }),
    }).toString();
    return fetchApi<PaginatedData<PostWithRelations>>(`/api/public/search?${query}`);
  },
};

// ================================
// 主题 API
// ================================

export const themesApi = {
  /**
   * 获取主题列表
   */
  async list(
    sort?: 'post_count' | 'view_count' | 'created_at'
  ): Promise<Theme[]> {
    const query = sort ? `?sort=${sort}` : '';
    return fetchApi<Theme[]>(`/api/public/themes${query}`);
  },

  /**
   * 获取主题详情
   */
  async getBySlug(slug: string): Promise<Theme> {
    return fetchApi<Theme>(`/api/public/themes/${slug}`);
  },

  /**
   * 增加主题浏览计数
   */
  async incrementView(slug: string): Promise<void> {
    await fetch(`${API_BASE}/api/public/themes/${slug}/view`, {
      method: 'POST',
    });
  },
};

// ================================
// 标签 API
// ================================

export const tagsApi = {
  /**
   * 获取标签列表
   */
  async list(
    sort?: 'post_count' | 'view_count' | 'created_at' | 'name'
  ): Promise<Tag[]> {
    const query = sort ? `?sort=${sort}` : '';
    return fetchApi<Tag[]>(`/api/public/tags${query}`);
  },

  /**
   * 获取标签详情
   */
  async getBySlug(slug: string): Promise<Tag> {
    return fetchApi<Tag>(`/api/public/tags/${slug}`);
  },

  /**
   * 增加标签浏览计数
   */
  async incrementView(slug: string): Promise<void> {
    await fetch(`${API_BASE}/api/public/tags/${slug}/view`, {
      method: 'POST',
    });
  },
};

// ================================
// 评论 API
// ================================

export const commentsApi = {
  /**
   * 获取文章评论树（包含所有层级）
   */
  async getPostComments(slug: string): Promise<CommentNode[]> {
    return fetchApi<CommentNode[]>(`/api/public/posts/${slug}/comments`);
  },

  /**
   * 获取某条评论的直接回复列表
   */
  async getCommentReplies(slug: string, id: number): Promise<Comment[]> {
    return fetchApi<Comment[]>(`/api/public/posts/${slug}/comments/${id}`);
  },

  /**
   * 提交评论
   */
  async submit(
    slug: string,
    data: CreateCommentRequest
  ): Promise<CreateCommentResponse> {
    return fetchApi<CreateCommentResponse>(
      `/api/public/posts/${slug}/comments`,
      {
        method: 'POST',
        body: JSON.stringify(data),
      }
    );
  },
};

// ================================
// 友链 API
// ================================

export const linksApi = {
  /**
   * 获取友链列表（仅显示 active 状态）
   */
  async list(): Promise<Link[]> {
    return fetchApi<Link[]>('/api/public/links');
  },

  /**
   * 提交友链申请
   */
  async submit(data: SubmitLinkRequest): Promise<SubmitLinkResponse> {
    return fetchApi<SubmitLinkResponse>('/api/public/links/submit', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
};

// ================================
// 认证 API（管理端）
// ================================

export const authApi = {
  /**
   * 管理员登录
   */
  async login(data: LoginRequest): Promise<LoginResponse> {
    return fetchApi<LoginResponse>('/api/admin/login', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  },
};

// ================================
// 管理端 API（需要 JWT）
// ================================

/**
 * 创建带 JWT 的请求配置
 */
function createAuthHeaders(): HeadersInit {
  const token = localStorage.getItem('yukilog_token');
  return {
    'Content-Type': 'application/json',
    ...(token && { Authorization: `Bearer ${token}` }),
  };
}

export const adminApi = {
  // ===== 文章管理 =====
  posts: {
    async list(params?: PostListParams): Promise<PaginatedData<PostWithRelations>> {
      const query = new URLSearchParams(params as any).toString();
      const headers = createAuthHeaders();
      return fetchApi<PaginatedData<PostWithRelations>>(`/api/admin/posts?${query}`, { headers });
    },
    
    async getBySlug(slug: string): Promise<PostWithRelations> {
      const headers = createAuthHeaders();
      return fetchApi<PostWithRelations>(`/api/admin/posts/${slug}`, { headers });
    },
    
    async create(data: CreatePostRequest): Promise<Post> {
      const headers = createAuthHeaders();
      return fetchApi<Post>('/api/admin/posts', {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
    },
    
    async update(slug: string, data: UpdatePostRequest): Promise<Post> {
      const headers = createAuthHeaders();
      return fetchApi<Post>(`/api/admin/posts/${slug}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
    },
    
    async delete(slug: string): Promise<void> {
      const headers = createAuthHeaders();
      await fetchApi<void>(`/api/admin/posts/${slug}`, {
        method: 'DELETE',
        headers,
      });
    },
  },

  // ===== 评论管理 =====
  comments: {
    async list(params?: CommentListParams): Promise<PaginatedData<Comment>> {
      const query = params ? new URLSearchParams(params as any).toString() : '';
      const headers = createAuthHeaders();
      return fetchApi<PaginatedData<Comment>>(`/api/admin/comments?${query}`, { headers });
    },
    
    async pending(): Promise<Comment[]> {
      const headers = createAuthHeaders();
      return fetchApi<Comment[]>('/api/admin/comments/pending', { headers });
    },
    
    async approve(id: number): Promise<Comment> {
      const headers = createAuthHeaders();
      return fetchApi<Comment>(`/api/admin/comments/${id}/approve`, {
        method: 'PUT',
        headers,
      });
    },
    
    async reject(id: number): Promise<void> {
      const headers = createAuthHeaders();
      await fetchApi<void>(`/api/admin/comments/${id}/reject`, {
        method: 'PUT',
        headers,
      });
    },
    
    async update(id: number, data: UpdateCommentRequest): Promise<Comment> {
      const headers = createAuthHeaders();
      return fetchApi<Comment>(`/api/admin/comments/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
    },
    
    async delete(id: number): Promise<void> {
      const headers = createAuthHeaders();
      await fetchApi<void>(`/api/admin/comments/${id}`, {
        method: 'DELETE',
        headers,
      });
    },
  },

  // ===== 主题管理 =====
  themes: {
    async create(data: CreateThemeRequest): Promise<Theme> {
      const headers = createAuthHeaders();
      return fetchApi<Theme>('/api/admin/themes', {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
    },
    
    async update(id: number, data: UpdateThemeRequest): Promise<Theme> {
      const headers = createAuthHeaders();
      return fetchApi<Theme>(`/api/admin/themes/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
    },
    
    async delete(id: number): Promise<void> {
      const headers = createAuthHeaders();
      await fetchApi<void>(`/api/admin/themes/${id}`, {
        method: 'DELETE',
        headers,
      });
    },
  },

  // ===== 标签管理 =====
  tags: {
    async create(data: CreateTagRequest): Promise<Tag> {
      const headers = createAuthHeaders();
      return fetchApi<Tag>('/api/admin/tags', {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
    },
    
    async update(id: number, data: UpdateTagRequest): Promise<Tag> {
      const headers = createAuthHeaders();
      return fetchApi<Tag>(`/api/admin/tags/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
    },
    
    async delete(id: number): Promise<void> {
      const headers = createAuthHeaders();
      await fetchApi<void>(`/api/admin/tags/${id}`, {
        method: 'DELETE',
        headers,
      });
    },
    
    async merge(data: MergeTagsRequest): Promise<Tag> {
      const headers = createAuthHeaders();
      return fetchApi<Tag>('/api/admin/tags/merge', {
        method: 'POST',
        headers,
        body: JSON.stringify(data),
      });
    },
  },

  // ===== 友链管理 =====
  links: {
    async list(): Promise<Link[]> {
      const headers = createAuthHeaders();
      return fetchApi<Link[]>('/api/admin/links', { headers });
    },
    
    async pending(): Promise<Link[]> {
      const headers = createAuthHeaders();
      return fetchApi<Link[]>('/api/admin/links/pending', { headers });
    },
    
    async approve(id: number): Promise<Link> {
      const headers = createAuthHeaders();
      return fetchApi<Link>(`/api/admin/links/${id}/approve`, {
        method: 'PUT',
        headers,
      });
    },
    
    async markBroken(id: number): Promise<Link> {
      const headers = createAuthHeaders();
      return fetchApi<Link>(`/api/admin/links/${id}/broken`, {
        method: 'PUT',
        headers,
      });
    },
    
    async update(id: number, data: UpdateLinkRequest): Promise<Link> {
      const headers = createAuthHeaders();
      return fetchApi<Link>(`/api/admin/links/${id}`, {
        method: 'PUT',
        headers,
        body: JSON.stringify(data),
      });
    },
    
    async delete(id: number): Promise<void> {
      const headers = createAuthHeaders();
      await fetchApi<void>(`/api/admin/links/${id}`, {
        method: 'DELETE',
        headers,
      });
    },
  },
};
