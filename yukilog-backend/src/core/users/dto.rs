//! 用户服务数据传输对象
//!
//! 定义用户管理相关的请求和响应结构

use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

// 定义静态正则表达式
static USERNAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").expect("Invalid regex"));

// ==================== 请求 DTO ====================

/// 创建用户请求
///
/// # 使用场景
/// - 用户自助注册（公开接口）
/// - 管理员创建用户（管理接口）
///
/// # 校验规则
/// - `username`: 3-20 字符，仅字母数字下划线
/// - `password`: 最少 8 字符
/// - `email`: 有效的邮箱格式（可选）
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserRequest {
    /// 用户名（唯一）
    ///
    /// # 校验规则
    /// - 长度: 3-20 字符
    /// - 格式: 字母、数字、下划线
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在 3-20 字符之间"))]
    #[validate(regex(path = "*USERNAME_REGEX", message = "用户名只能包含字母、数字和下划线"))]
    pub username: String,

    /// 密码（明文，将被哈希）
    ///
    /// # 校验规则
    /// - 长度: 最少 8 字符
    #[validate(length(min = 8, message = "密码长度至少为 8 字符"))]
    pub password: String,

    /// 邮箱（可选，唯一）
    ///
    /// # 校验规则
    /// - 必须是有效的邮箱格式
    #[validate(email(message = "邮箱格式无效"))]
    pub email: Option<String>,

    /// 昵称（可选）
    ///
    /// # 校验规则
    /// - 长度: 1-50 字符
    #[validate(length(min = 1, max = 50, message = "昵称长度必须在 1-50 字符之间"))]
    pub nickname: Option<String>,
}

/// 更新用户资料请求
///
/// # 使用场景
/// - 用户更新自己的资料（`/api/user/me`）
///
/// # 说明
/// - 所有字段都是可选的（部分更新）
/// - 不允许修改 `username`、`email`、`role`
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    /// 昵称
    #[validate(length(min = 1, max = 50, message = "昵称长度必须在 1-50 字符之间"))]
    pub nickname: Option<String>,

    /// 头像 URL
    #[validate(url(message = "头像 URL 格式无效"))]
    pub avatar_url: Option<String>,

    /// 个人简介
    #[validate(length(max = 500, message = "个人简介最多 500 字符"))]
    pub bio: Option<String>,
}

/// 修改密码请求
///
/// # 使用场景
/// - 用户修改自己的密码
///
/// # 安全要求
/// - 必须验证旧密码
/// - 新密码不能与旧密码相同
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    /// 当前密码（用于验证用户身份）
    pub old_password: String,

    /// 新密码
    #[validate(length(min = 8, message = "新密码长度至少为 8 字符"))]
    pub new_password: String,
}

/// 更新邮箱请求
///
/// # 使用场景
/// - 用户更新自己的邮箱
///
/// # 安全要求
/// - 必须验证当前密码
/// - 检查新邮箱是否已被使用
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateEmailRequest {
    /// 新邮箱
    #[validate(email(message = "邮箱格式无效"))]
    pub email: String,

    /// 当前密码（验证用户身份）
    pub password: String,
}

// ==================== 响应 DTO ====================

/// 用户详情响应
///
/// # 使用场景
/// - 获取用户详情（本人或 Admin）
/// - 用户资料页
///
/// # 安全说明
/// - 不返回 `password_hash`
/// - `email` 字段应根据权限控制是否返回（Handler 层处理）
#[derive(Debug, Clone, Serialize)]
pub struct UserDetailResponse {
    /// 用户 ID
    pub id: i64,

    /// 用户名
    pub username: String,

    /// 邮箱（可能为空或被隐藏）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// 昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// 头像 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// 个人简介
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,

    /// 用户角色（"admin" 或 "user"）
    pub role: String,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

impl UserDetailResponse {
    /// 从 User Entity 创建响应
    ///
    /// # 参数
    /// - `user`: users::Model 实体
    /// - `include_email`: 是否包含邮箱字段（根据权限决定）
    pub fn from_entity(user: &crate::entities::users::Model, include_email: bool) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            email: if include_email {
                user.email.clone()
            } else {
                None
            },
            nickname: user.nickname.clone(),
            avatar_url: user.avatar_url.clone(),
            bio: None, // 数据库暂无 bio 字段，预留
            role: user.role.clone().unwrap_or_else(|| "user".to_string()),
            created_at: user
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
            updated_at: user
                .updated_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
        }
    }
}

/// 用户列表项响应（简化版）
///
/// # 使用场景
/// - 管理员获取用户列表
/// - 文章作者信息展示
///
/// # 特点
/// - 不返回 `email`（隐私保护）
/// - 不返回 `bio`（减少数据量）
#[derive(Debug, Clone, Serialize)]
pub struct UserListItemResponse {
    /// 用户 ID
    pub id: i64,

    /// 用户名
    pub username: String,

    /// 昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// 头像 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// 用户角色
    pub role: String,

    /// 创建时间
    pub created_at: DateTime<Utc>,
}

impl UserListItemResponse {
    /// 从 User Entity 创建列表项响应
    pub fn from_entity(user: &crate::entities::users::Model) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            nickname: user.nickname.clone(),
            avatar_url: user.avatar_url.clone(),
            role: user.role.clone().unwrap_or_else(|| "user".to_string()),
            created_at: user
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
        }
    }
}
