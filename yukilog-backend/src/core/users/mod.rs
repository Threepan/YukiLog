//! 用户服务模块
//!
//! 提供用户管理相关的业务逻辑，包括：
//! - 用户 CRUD（创建、查询、更新、删除）
//! - 资料管理（更新资料、修改密码、更新邮箱）
//! - 管理员操作（修改角色、删除用户）
//!
//! # 模块结构
//!
//! - [`dto`] - 数据传输对象
//! - [`service`] - 用户服务
//!
//! # 示例
//!
//! ```rust,ignore
//! use crate::core::users::{UsersService, dto::CreateUserRequest};
//!
//! // 创建服务
//! let user_service = UsersService::new(user_repo);
//!
//! // 创建用户
//! let req = CreateUserRequest {
//!     username: "alice".to_string(),
//!     password: "password123".to_string(),
//!     email: Some("alice@example.com".to_string()),
//!     nickname: Some("Alice".to_string()),
//! };
//! let user = user_service.create_user(req).await?;
//! ```

pub mod dto;
pub mod service;

// 重导出常用类型
pub use dto::{
    ChangePasswordRequest, CreateUserRequest, UpdateEmailRequest, UpdateProfileRequest,
    UserDetailResponse, UserListItemResponse,
};
pub use service::UsersService;
