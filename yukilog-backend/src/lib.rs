//! YukiLog 后端服务
//!
//! 一个使用 Rust + Axum + SeaORM 构建的博客系统后端。
//!
//! # 架构分层
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │   HTTP Layer (api/)                     │  ← 路由、中间件、Handler
//! ├─────────────────────────────────────────┤
//! │   Service Layer (core/)                 │  ← 业务逻辑、事务处理
//! ├─────────────────────────────────────────┤
//! │   Repository Layer (infra/)             │  ← 数据访问、CRUD 抽象
//! ├─────────────────────────────────────────┤
//! │   Entity Layer (entities/)              │  ← SeaORM 模型定义
//! └─────────────────────────────────────────┘
//! ```
//!
//! # 核心模块
//!
//! - [`api`] - HTTP API 层，提供 RESTful 接口
//! - [`core`] - 业务逻辑层，包含所有 Service
//! - [`infra`] - 基础设施层，数据访问和外部服务
//! - [`entities`] - 数据模型层，SeaORM 实体定义
//! - [`common`] - 公共工具，响应格式、分页等
//! - [`config`] - 配置管理，环境变量加载
//!
//! # 快速开始
//!
//! ```rust,no_run
//! use yukilog_backend::config::AppConfig;
//!
//! #[tokio::main]
//! async fn main() {
//!     // 加载配置
//!     dotenvy::dotenv().ok();
//!     let config = AppConfig::from_env().expect("Failed to load config");
//!     
//!     // 启动服务器
//!     println!("Server running on {}:{}", config.server.host, config.server.port);
//! }
//! ```
//!
//! # 功能特性
//!
//! - ✅ 用户认证（JWT Token）
//! - ✅ 文章管理（CRUD、分类、标签）
//! - ✅ 评论系统（嵌套评论树）
//! - ✅ 分类与标签
//! - ✅ 友链管理
//! - ✅ 管理后台
//!
//! # API 文档
//!
//! 详见 [`api`] 模块和项目文档 `docs/Contract.md`

// HTTP API 层
pub mod api;

// 公共工具
pub mod common;

// 配置管理
pub mod config;

// 业务逻辑层
pub mod core;

// 数据模型层
pub mod entities;

// 基础设施层
pub mod infra;
