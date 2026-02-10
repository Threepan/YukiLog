//! 用户仓储层
//!
//! 提供用户相关的数据访问操作

use crate::entities::{prelude::Users, users};
use sea_orm::*;

/// 用户仓储
#[derive(Clone)]
pub struct UsersRepository {
    db: DatabaseConnection,
}

impl UsersRepository {
    /// 创建新的用户仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 根据 ID 查询用户
    ///
    /// # 参数
    /// - `id`: 用户 ID
    ///
    /// # 返回
    /// - `Ok(Some(Model))`: 找到用户
    /// - `Ok(None)`: 用户不存在
    /// - `Err`: 数据库错误
    pub async fn find_by_id(&self, id: i64) -> Result<Option<users::Model>, DbErr> {
        Users::find_by_id(id).one(&self.db).await
    }

    /// 根据用户名查询用户
    ///
    /// 主要用于登录验证场景
    ///
    /// # 参数
    /// - `username`: 用户名（精确匹配）
    pub async fn find_by_username(&self, username: &str) -> Result<Option<users::Model>, DbErr> {
        Users::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.db)
            .await
    }

    /// 根据用户名查询用户（包含密码哈希）
    ///
    /// 主要用于登录验证场景，比 `find_by_username` 性能更好。
    /// 使用 `select_only()` 只选择必要字段，减少数据传输。
    ///
    /// # 参数
    /// - `username`: 用户名（精确匹配）
    pub async fn find_by_username_with_password(
        &self,
        username: &str,
    ) -> Result<Option<users::Model>, DbErr> {
        Users::find()
            .filter(users::Column::Username.eq(username))
            .select_only()
            .columns([
                users::Column::Id,
                users::Column::Username,
                users::Column::PasswordHash,
                users::Column::Role,
            ])
            .into_model::<users::Model>()
            .one(&self.db)
            .await
    }

    /// 根据邮箱查询用户
    ///
    /// # 参数
    /// - `email`: 邮箱地址（精确匹配）
    pub async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr> {
        Users::find()
            .filter(users::Column::Email.eq(email))
            .one(&self.db)
            .await
    }

    /// 检查用户名是否已存在
    ///
    /// 用于注册时验证用户名唯一性
    ///
    /// # 参数
    /// - `username`: 待检查的用户名
    ///
    /// # 返回
    /// - `Ok(true)`: 用户名已存在
    /// - `Ok(false)`: 用户名可用
    pub async fn exists_by_username(&self, username: &str) -> Result<bool, DbErr> {
        let count = Users::find()
            .filter(users::Column::Username.eq(username))
            .count(&self.db)
            .await?;
        Ok(count > 0)
    }

    /// 检查邮箱是否已存在
    ///
    /// 用于注册时验证邮箱唯一性
    ///
    /// # 参数
    /// - `email`: 待检查的邮箱
    pub async fn exists_by_email(&self, email: &str) -> Result<bool, DbErr> {
        let count = Users::find()
            .filter(users::Column::Email.eq(email))
            .count(&self.db)
            .await?;
        Ok(count > 0)
    }

    /// 获取所有用户列表（分页）
    ///
    /// 主要用于管理后台的用户管理
    ///
    /// # 参数
    /// - `page`: 页码（从 1 开始）
    /// - `size`: 每页数量
    ///
    /// # 返回
    /// - 用户列表和总数的元组 `(Vec<Model>, u64)`
    pub async fn find_all_paginated(
        &self,
        page: u64,
        size: u64,
    ) -> Result<(Vec<users::Model>, u64), DbErr> {
        let paginator = Users::find()
            .order_by_desc(users::Column::CreatedAt)
            .paginate(&self.db, size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 根据角色查询用户
    ///
    /// # 参数
    /// - `role`: 角色名称（如 "admin", "user"）
    pub async fn find_by_role(&self, role: &str) -> Result<Vec<users::Model>, DbErr> {
        Users::find()
            .filter(users::Column::Role.eq(role))
            .order_by_desc(users::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 批量根据 ID 查询用户
    ///
    /// # 说明
    /// - 用于避免 Service 层 N+1 查询
    /// - 返回结果不保证顺序
    pub async fn find_by_ids(&self, ids: Vec<i64>) -> Result<Vec<users::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        Users::find()
            .filter(users::Column::Id.is_in(ids))
            .all(&self.db)
            .await
    }

    /// 创建新用户
    ///
    /// # 参数
    /// - `user`: 用户 ActiveModel
    ///
    /// # 返回
    /// - 创建成功的用户 Model
    pub async fn create(&self, user: users::ActiveModel) -> Result<users::Model, DbErr> {
        user.insert(&self.db).await
    }

    /// 更新用户信息
    ///
    /// # 参数
    /// - `user`: 包含更新字段的 ActiveModel
    ///
    /// # 注意
    /// - 需要设置 `id` 字段以标识要更新的用户
    /// - 只有标记为 `Set` 的字段会被更新
    pub async fn update(&self, user: users::ActiveModel) -> Result<users::Model, DbErr> {
        user.update(&self.db).await
    }

    /// 删除用户
    ///
    /// # 参数
    /// - `id`: 用户 ID
    ///
    /// # 注意
    /// - 这是硬删除操作
    /// - 由于数据库外键约束，会级联删除相关的文章和评论
    pub async fn delete(&self, id: i64) -> Result<DeleteResult, DbErr> {
        Users::delete_by_id(id).exec(&self.db).await
    }

    /// 更新用户角色
    ///
    /// 快捷方法，只更新角色字段
    ///
    /// # 参数
    /// - `id`: 用户 ID
    /// - `role`: 新角色名称
    pub async fn update_role(&self, id: i64, role: &str) -> Result<users::Model, DbErr> {
        let mut user: users::ActiveModel = Users::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("User {} not found", id)))?
            .into();

        user.role = Set(Some(role.to_string()));
        user.update(&self.db).await
    }

    /// 获取用户总数
    ///
    /// 用于仪表盘统计
    pub async fn count_all(&self) -> Result<u64, DbErr> {
        Users::find().count(&self.db).await
    }
}
