//! Repository 层模块
//!
//! 本模块提供数据访问抽象层，封装所有数据库操作。
//!
//! # 设计原则
//! - Repository 只负责数据访问，不包含业务逻辑
//! - 返回原始 Entity Model，不返回 DTO
//! - 复杂查询使用 SeaORM 的 LoaderTrait 避免 N+1 问题
//! - 事务控制在 Service 层，Repository 可提供事务辅助方法

pub mod categories;
pub mod comments;
pub mod links;
pub mod posts;
pub mod tags;
pub mod users;

// 重导出，方便其他模块使用
pub use categories::CategoriesRepository;
pub use comments::CommentsRepository;
pub use links::LinksRepository;
pub use posts::PostsRepository;
pub use tags::TagsRepository;
pub use users::UsersRepository;
