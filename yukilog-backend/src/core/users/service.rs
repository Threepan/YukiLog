//! 用户服务
//!
//! 提供用户管理相关的业务逻辑

use sea_orm::Set;
use validator::Validate;

use crate::common::PaginatedResponse;
use crate::core::auth::password;
use crate::core::error::AppError;
use crate::core::validation::validate_pagination;
use crate::entities::users;
use crate::infra::repository::UsersRepository;

use super::dto::{
    ChangePasswordRequest, CreateUserRequest, UpdateEmailRequest, UpdateProfileRequest,
    UserDetailResponse, UserListItemResponse,
};

/// 用户服务
///
/// # 依赖
/// - `UsersRepository`: 用户数据访问
///
/// # 线程安全
/// 所有字段都是 Clone + Send + Sync，可安全在多线程中使用
#[derive(Clone)]
pub struct UsersService {
    /// 用户仓储
    user_repo: UsersRepository,
}

impl UsersService {
    /// 创建用户服务实例
    ///
    /// # 参数
    /// - `user_repo`: 用户仓储实例
    pub fn new(user_repo: UsersRepository) -> Self {
        Self { user_repo }
    }

    // ==================== 基础 CRUD ====================

    /// 创建用户
    ///
    /// # 参数
    /// - `req`: 创建用户请求
    ///
    /// # 返回
    /// - 成功：返回用户详情
    /// - 失败：返回 `AppError`
    ///
    /// # 流程
    /// 1. 校验输入参数
    /// 2. 检查用户名唯一性
    /// 3. 检查邮箱唯一性（如果提供）
    /// 4. 哈希密码
    /// 5. 创建用户记录（默认 role 为 "user"）
    ///
    /// # 错误
    /// - `Validation(...)`: 输入参数校验失败
    /// - `Business("用户名已存在")`: 用户名冲突
    /// - `Business("邮箱已被使用")`: 邮箱冲突
    pub async fn create_user(
        &self,
        req: CreateUserRequest,
    ) -> Result<UserDetailResponse, AppError> {
        // 1. 校验输入
        req.validate()
            .map_err(|e| AppError::Validation(format!("输入校验失败: {}", e)))?;

        // 2. 检查用户名唯一性
        if self.user_repo.exists_by_username(&req.username).await? {
            return Err(AppError::Business("用户名已存在".to_string()));
        }

        // 3. 检查邮箱唯一性（如果提供）
        if let Some(ref email) = req.email {
            if self.user_repo.exists_by_email(email).await? {
                return Err(AppError::Business("邮箱已被使用".to_string()));
            }
        }

        // 4. 哈希密码
        let password_hash = password::hash_password(&req.password)?;

        // 5. 创建用户
        let new_user = users::ActiveModel {
            username: Set(req.username),
            password_hash: Set(password_hash),
            email: Set(req.email),
            nickname: Set(req.nickname),
            role: Set(Some("user".to_string())), // 默认角色
            ..Default::default()
        };

        let user = self.user_repo.create(new_user).await?;

        Ok(UserDetailResponse::from_entity(&user, true))
    }

    /// 获取用户详情
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    ///
    /// # 返回
    /// - 成功：返回用户详情（包含 email）
    /// - 失败：返回 `AppError::NotFound`
    ///
    /// # 说明
    /// Service 层返回完整信息，Handler 层根据权限决定是否过滤 email
    pub async fn get_user_by_id(&self, user_id: i64) -> Result<UserDetailResponse, AppError> {
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        Ok(UserDetailResponse::from_entity(&user, true))
    }

    /// 分页获取用户列表
    ///
    /// # 参数
    /// - `page`: 页码（从 1 开始）
    /// - `size`: 每页数量
    /// - `role_filter`: 可选的角色筛选（"admin" 或 "user"）
    ///
    /// # 返回
    /// - 成功：返回分页后的用户列表（不含 email）
    ///
    /// # 使用场景
    /// - 管理员获取用户列表
    pub async fn get_users_paginated(
        &self,
        page: u64,
        size: u64,
        role_filter: Option<String>,
    ) -> Result<PaginatedResponse<UserListItemResponse>, AppError> {
        validate_pagination(page, size)?;
        let (users, total) = if let Some(role) = role_filter {
            // 按角色筛选
            let users = self.user_repo.find_by_role(&role).await?;
            let total = users.len() as u64;

            // 手动分页
            let offset = (page - 1)
                .checked_mul(size)
                .ok_or_else(|| AppError::BadRequest("page 参数过大".to_string()))?;
            let offset_usize = usize::try_from(offset)
                .map_err(|_| AppError::BadRequest("page 参数过大".to_string()))?;
            let size_usize = usize::try_from(size)
                .map_err(|_| AppError::BadRequest("size 参数过大".to_string()))?;

            let paginated = if offset_usize >= users.len() {
                vec![]
            } else {
                let end = (offset_usize + size_usize).min(users.len());
                users[offset_usize..end].to_vec()
            };

            (paginated, total)
        } else {
            // 获取所有用户
            self.user_repo.find_all_paginated(page, size).await?
        };

        // 转换为列表项响应
        let items: Vec<UserListItemResponse> = users
            .iter()
            .map(UserListItemResponse::from_entity)
            .collect();

        Ok(PaginatedResponse::new(items, total, page, size))
    }

    // ==================== 资料管理 ====================

    /// 更新用户资料
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    /// - `req`: 更新资料请求
    ///
    /// # 流程
    /// 1. 校验输入
    /// 2. 查询用户
    /// 3. 更新字段（仅更新非 None 的字段）
    /// 4. 保存到数据库
    ///
    /// # 注意
    /// - 不允许修改 `username`、`email`、`role`、`password_hash`
    pub async fn update_profile(
        &self,
        user_id: i64,
        req: UpdateProfileRequest,
    ) -> Result<UserDetailResponse, AppError> {
        // 1. 校验输入
        req.validate()
            .map_err(|e| AppError::Validation(format!("输入校验失败: {}", e)))?;

        // 2. 查询用户
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        // 3. 更新字段（部分更新）
        let mut active_user: users::ActiveModel = user.into();

        if let Some(nickname) = req.nickname {
            active_user.nickname = Set(Some(nickname));
        }

        if let Some(avatar_url) = req.avatar_url {
            active_user.avatar_url = Set(Some(avatar_url));
        }

        // 注意：bio 字段数据库暂无，预留
        // if let Some(bio) = req.bio {
        //     active_user.bio = Set(Some(bio));
        // }

        // 4. 保存
        let updated_user = self.user_repo.update(active_user).await?;

        Ok(UserDetailResponse::from_entity(&updated_user, true))
    }

    /// 修改密码
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    /// - `req`: 修改密码请求
    ///
    /// # 流程
    /// 1. 校验输入
    /// 2. 查询用户
    /// 3. 验证旧密码
    /// 4. 哈希新密码
    /// 5. 更新 `password_hash`
    ///
    /// # 错误
    /// - `Unauthorized("当前密码错误")`: 旧密码验证失败
    pub async fn change_password(
        &self,
        user_id: i64,
        req: ChangePasswordRequest,
    ) -> Result<(), AppError> {
        // 1. 校验输入
        req.validate()
            .map_err(|e| AppError::Validation(format!("输入校验失败: {}", e)))?;

        // 2. 查询用户
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        // 3. 验证旧密码
        let is_valid = password::verify_password(&req.old_password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::Unauthorized("当前密码错误".to_string()));
        }

        // 4. 哈希新密码
        let new_hash = password::hash_password(&req.new_password)?;

        // 5. 更新密码
        let mut active_user: users::ActiveModel = user.into();
        active_user.password_hash = Set(new_hash);

        self.user_repo.update(active_user).await?;

        Ok(())
    }

    /// 更新邮箱
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    /// - `req`: 更新邮箱请求
    ///
    /// # 流程
    /// 1. 校验输入
    /// 2. 查询用户
    /// 3. 验证当前密码
    /// 4. 检查新邮箱是否已被使用
    /// 5. 更新 email 字段
    ///
    /// # 错误
    /// - `Unauthorized("密码错误")`: 密码验证失败
    /// - `Business("邮箱已被使用")`: 邮箱冲突
    pub async fn update_email(
        &self,
        user_id: i64,
        req: UpdateEmailRequest,
    ) -> Result<UserDetailResponse, AppError> {
        // 1. 校验输入
        req.validate()
            .map_err(|e| AppError::Validation(format!("输入校验失败: {}", e)))?;

        // 2. 查询用户
        let user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

        // 3. 验证当前密码
        let is_valid = password::verify_password(&req.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::Unauthorized("密码错误".to_string()));
        }

        // 4. 检查新邮箱是否已被使用
        if self.user_repo.exists_by_email(&req.email).await? {
            return Err(AppError::Business("邮箱已被使用".to_string()));
        }

        // 5. 更新邮箱
        let mut active_user: users::ActiveModel = user.into();
        active_user.email = Set(Some(req.email));

        let updated_user = self.user_repo.update(active_user).await?;

        Ok(UserDetailResponse::from_entity(&updated_user, true))
    }

    // ==================== 管理员操作 ====================

    /// 修改用户角色（Admin only）
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    /// - `new_role`: 新角色（"admin" 或 "user"）
    ///
    /// # 流程
    /// 1. 校验 role 值
    /// 2. 调用 Repository 更新角色
    ///
    /// # 错误
    /// - `Validation("无效的角色值")`: role 不是 "admin" 或 "user"
    pub async fn update_role(
        &self,
        user_id: i64,
        new_role: &str,
    ) -> Result<UserDetailResponse, AppError> {
        // 1. 校验 role 值
        if !["admin", "user"].contains(&new_role) {
            return Err(AppError::Validation(
                "无效的角色值，只能是 'admin' 或 'user'".to_string(),
            ));
        }

        // 2. 更新角色
        let user = self.user_repo.update_role(user_id, new_role).await?;

        Ok(UserDetailResponse::from_entity(&user, true))
    }

    /// 删除用户（Admin only）
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    ///
    /// # 流程
    /// 1. 检查用户是否存在
    /// 2. 调用 Repository 删除（硬删除）
    ///
    /// # 注意
    /// - 数据库外键设置为 CASCADE，会自动删除关联的文章、评论
    /// - 如需更安全的删除策略，可在调用前检查用户是否有内容
    pub async fn delete_user(&self, user_id: i64) -> Result<(), AppError> {
        // 1. 检查用户是否存在
        if !self.user_repo.find_by_id(user_id).await?.is_some() {
            return Err(AppError::NotFound("用户不存在".to_string()));
        }

        // 2. 删除用户（CASCADE 由数据库处理）
        self.user_repo.delete(user_id).await?;

        Ok(())
    }

    // ==================== 工具方法 ====================

    /// 检查用户名是否存在
    ///
    /// # 使用场景
    /// - 注册时实时校验用户名
    /// - 前端表单验证
    pub async fn exists_by_username(&self, username: &str) -> Result<bool, AppError> {
        self.user_repo
            .exists_by_username(username)
            .await
            .map_err(AppError::Database)
    }

    /// 检查邮箱是否存在
    ///
    /// # 使用场景
    /// - 注册时实时校验邮箱
    /// - 更新邮箱时校验
    pub async fn exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        self.user_repo
            .exists_by_email(email)
            .await
            .map_err(AppError::Database)
    }
}
