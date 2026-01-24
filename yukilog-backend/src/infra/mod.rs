//! 基础设施层模块
//!
//! 本模块提供数据访问和外部服务集成。
//!
//! # 模块结构
//! - `repository/` - 数据访问层，封装数据库操作
//!
//! # 设计原则
//! - 基础设施层只负责数据访问和外部服务调用
//! - 不包含业务逻辑（业务逻辑在 Service 层）
//! - 提供统一的错误处理（返回 `sea_orm::DbErr`）

pub mod repository;

// 重导出常用类型
pub use repository::{
    CategoriesRepository, CommentsRepository, LinksRepository, PostsRepository, TagsRepository,
    UsersRepository,
};
