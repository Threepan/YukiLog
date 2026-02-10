//! 分页查询工具
//!
//! 本模块提供统一的分页参数和响应结构。
//!
//! # 设计规范
//! - `page` 从 1 开始（符合用户习惯，不是从 0 开始）
//! - `size` 限制在 1-100 之间，防止单次查询过大
//! - 自动计算 `total_pages`（总页数）

use serde::{Deserialize, Serialize};

/// 分页查询参数
///
/// # 字段
/// - `page`: 页码，从 1 开始
/// - `size`: 每页数量，范围 1-100
///
/// # 使用方式
/// ```rust,ignore
/// // 从查询字符串解析
/// // GET /api/posts?page=2&size=20
/// async fn list_posts(Query(params): Query<PaginationParams>) {
///     let params = params.normalize(); // 校验并规范化
///     // ...
/// }
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    /// 页码（从 1 开始）
    ///
    /// 如果不提供，默认为 1
    #[serde(default = "default_page")]
    pub page: u64,

    /// 每页数量
    ///
    /// 如果不提供，默认为 10
    /// 最大值限制为 100，防止单次查询数据过多
    #[serde(default = "default_size")]
    pub size: u64,
}

/// 默认页码：第 1 页
fn default_page() -> u64 {
    1
}

/// 默认每页数量：10 条
fn default_size() -> u64 {
    10
}

impl PaginationParams {
    /// 校验并规范化分页参数
    ///
    /// # 规则
    /// - `page` 最小值为 1（用户传 0 或负数会被修正）
    /// - `size` 限制在 1-100 之间
    ///
    /// # 返回
    /// 返回规范化后的参数
    ///
    /// # 示例
    /// ```rust,ignore
    /// let params = PaginationParams { page: 0, size: 200 };
    /// let normalized = params.normalize();
    /// // normalized.page = 1 (修正为最小值)
    /// // normalized.size = 100 (限制为最大值)
    /// ```
    pub fn normalize(self) -> Self {
        Self {
            page: self.page.max(1),        // 最小为 1
            size: self.size.clamp(1, 100), // 1 到 100 之间
        }
    }

    /// 计算数据库查询的偏移量（offset）
    ///
    /// # 说明
    /// SeaORM 的 `paginate` 方法从第 0 页开始，
    /// 所以需要将用户的页码（从 1 开始）转换为数据库的页码（从 0 开始）
    ///
    /// # 公式
    /// `offset = (page - 1) * size`
    ///
    /// # 示例
    /// ```rust,ignore
    /// let params = PaginationParams { page: 3, size: 10 };
    /// let offset = params.offset(); // 返回 20
    /// // 意味着跳过前 20 条记录，从第 21 条开始
    /// ```
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.size
    }

    /// 获取 SeaORM 分页所需的页码（从 0 开始）
    ///
    /// # 说明
    /// SeaORM 的 `fetch_page(n)` 中，n 从 0 开始
    /// 用户的 page=1 对应 SeaORM 的 page=0
    ///
    /// # 示例
    /// ```rust,ignore
    /// let params = PaginationParams { page: 1, size: 10 };
    /// let db_page = params.db_page(); // 返回 0
    /// ```
    pub fn db_page(&self) -> u64 {
        self.page.saturating_sub(1)
    }
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            size: default_size(),
        }
    }
}

/// 分页响应数据
///
/// # 泛型参数
/// - `T`: 列表项的数据类型
///
/// # 字段说明
/// - `items`: 当前页的数据列表
/// - `total`: 符合条件的总记录数
/// - `page`: 当前页码
/// - `size`: 每页数量
/// - `total_pages`: 总页数（自动计算）
///
/// # 示例响应
/// ```json
/// {
///   "items": [{ "id": 1, "title": "..." }, ...],
///   "total": 100,
///   "page": 1,
///   "size": 10,
///   "total_pages": 10
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    /// 当前页的数据列表
    pub items: Vec<T>,

    /// 符合条件的总记录数
    pub total: u64,

    /// 当前页码（从 1 开始）
    pub page: u64,

    /// 每页数量
    pub size: u64,

    /// 总页数（向上取整）
    ///
    /// 计算公式: ceil(total / size)
    pub total_pages: u64,
}

impl<T> PaginatedResponse<T> {
    /// 创建分页响应
    ///
    /// # 参数
    /// - `items`: 当前页的数据列表
    /// - `total`: 总记录数
    /// - `page`: 当前页码
    /// - `size`: 每页数量
    ///
    /// # 说明
    /// `total_pages` 会自动计算（向上取整）
    ///
    /// # 示例
    /// ```rust,ignore
    /// let posts = vec![post1, post2, post3];
    /// let response = PaginatedResponse::new(posts, 100, 1, 10);
    /// // response.total_pages = 10
    /// ```
    pub fn new(items: Vec<T>, total: u64, page: u64, size: u64) -> Self {
        // 计算总页数（向上取整）
        // 使用 (total + size - 1) / size 的技巧避免浮点运算
        let total_pages = if size > 0 {
            (total + size - 1) / size
        } else {
            0
        };

        Self {
            items,
            total,
            page,
            size,
            total_pages,
        }
    }

    /// 从 Repository 返回的元组创建分页响应
    ///
    /// # 参数
    /// - `items_and_total`: Repository 返回的 `(Vec<T>, u64)` 元组
    /// - `params`: 分页参数
    ///
    /// # 示例
    /// ```rust,ignore
    /// let (posts, total) = repo.find_paginated(page, size).await?;
    /// let response = PaginatedResponse::from_tuple((posts, total), &params);
    /// ```
    pub fn from_tuple((items, total): (Vec<T>, u64), params: &PaginationParams) -> Self {
        Self::new(items, total, params.page, params.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_page() {
        let params = PaginationParams { page: 0, size: 10 };
        let normalized = params.normalize();
        assert_eq!(normalized.page, 1); // 修正为最小值
    }

    #[test]
    fn test_normalize_size() {
        let params = PaginationParams { page: 1, size: 200 };
        let normalized = params.normalize();
        assert_eq!(normalized.size, 100); // 限制为最大值
    }

    #[test]
    fn test_offset() {
        let params = PaginationParams { page: 3, size: 10 };
        assert_eq!(params.offset(), 20); // (3-1) * 10
    }

    #[test]
    fn test_db_page() {
        let params = PaginationParams { page: 1, size: 10 };
        assert_eq!(params.db_page(), 0); // page 1 -> db page 0
    }

    #[test]
    fn test_total_pages() {
        let response = PaginatedResponse::new(vec![1, 2, 3], 25, 1, 10);
        assert_eq!(response.total_pages, 3); // ceil(25/10) = 3
    }
}
