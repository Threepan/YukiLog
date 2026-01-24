//! HTTP API 层
//!
//! 本模块提供 RESTful API 接口，处理 HTTP 请求和响应。
//!
//! # 模块结构
//! - `middleware/` - 中间件（认证、权限、日志等）
//! - `extractors/` - 自定义提取器（分页、当前用户等）
//! - `http/` - HTTP 处理器（按资源分组）
//!   - 公开接口（文章、评论、分类等）
//!   - `user/` - 用户接口（需登录）
//!   - `admin/` - 管理接口（需管理员权限）
//!
//! # 请求处理流程
//! 1. 请求到达 → Middleware 处理
//! 2. Extractor 提取参数
//! 3. Handler 调用 Service
//! 4. Service 返回结果
//! 5. Handler 构造 `ApiResponse` 返回
//!
//! # 响应格式
//! 所有接口返回统一的 JSON 格式：
//! ```json
//! {
//!   "code": 0,
//!   "data": { ... },
//!   "message": "success"
//! }
//! ```

pub mod extractors;
pub mod http;
pub mod middleware;

// FFI 接口（如有需要）
pub mod ffi;
