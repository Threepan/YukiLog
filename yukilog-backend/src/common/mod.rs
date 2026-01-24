//! 公共工具模块
//!
//! 提供跨层使用的通用工具和数据结构

pub mod pagination;
pub mod response;

// 重导出常用类型，方便其他模块使用
pub use pagination::{PaginatedResponse, PaginationParams};
pub use response::ApiResponse;
