//! 分类服务模块
//!
//! 提供分类管理相关的业务逻辑，包括：
//! - 分类 CRUD（创建、查询、更新、删除）
//! - 唯一性校验（name、slug）
//! - 文章数统计
//!
//! # 模块结构
//!
//! - [`dto`] - 数据传输对象
//! - [`service`] - 分类服务
//!
//! # 示例
//!
//! ```rust,ignore
//! use crate::core::categories::{CategoriesService, dto::CreateCategoryRequest};
//!
//! // 创建服务
//! let category_service = CategoriesService::new(category_repo);
//!
//! // 创建分类
//! let req = CreateCategoryRequest {
//!     name: "Rust 编程".to_string(),
//!     slug: "rust-programming".to_string(),
//!     description: Some("Rust 语言相关文章".to_string()),
//! };
//! let category = category_service.create_category(req).await?;
//! ```

pub mod dto;
pub mod service;

// 重导出常用类型
pub use dto::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest};
pub use service::CategoriesService;
