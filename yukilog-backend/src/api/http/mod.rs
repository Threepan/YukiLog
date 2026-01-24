//! HTTP 处理器
//!
//! 本模块提供所有 HTTP API 的处理函数。
//!
//! # 模块结构
//! - 公开接口 - 文章、评论、分类、标签等
//! - `user/` - 用户接口（需登录）
//! - `admin/` - 管理接口（需管理员）

pub mod admin;
pub mod user;

// 公开接口（待实现）
// - posts.rs - 文章列表、详情
// - categories.rs - 分类列表
// - tags.rs - 标签云
// - comments.rs - 评论
// - links.rs - 友链
// - auth.rs - 登录、注册
