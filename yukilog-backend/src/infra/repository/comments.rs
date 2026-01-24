//! 评论仓储层
//!
//! 提供评论相关的数据访问操作

use crate::entities::{comments, prelude::Comments};
use sea_orm::*;

/// 评论仓储
#[derive(Clone)]
pub struct CommentsRepository {
    db: DatabaseConnection,
}

impl CommentsRepository {
    /// 创建新的评论仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 根据 ID 查询评论
    ///
    /// # 参数
    /// - `id`: 评论 ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<comments::Model>, DbErr> {
        Comments::find_by_id(id).one(&self.db).await
    }

    /// 获取文章的所有已审核评论
    ///
    /// # 参数
    /// - `post_id`: 文章 ID
    ///
    /// # 返回
    /// - 按创建时间正序排列的评论列表
    ///
    /// # 说明
    /// - 只返回已审核通过的评论（is_reviewed = true）
    /// - 用于前台展示
    /// - Service 层负责构建评论树结构
    pub async fn find_approved_by_post_id(
        &self,
        post_id: i64,
    ) -> Result<Vec<comments::Model>, DbErr> {
        Comments::find()
            .filter(comments::Column::PostId.eq(post_id))
            .filter(comments::Column::IsReviewed.eq(true))
            .order_by_asc(comments::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 获取文章的所有评论（含未审核）
    ///
    /// # 参数
    /// - `post_id`: 文章 ID
    ///
    /// # 说明
    /// - 用于管理后台
    pub async fn find_all_by_post_id(&self, post_id: i64) -> Result<Vec<comments::Model>, DbErr> {
        Comments::find()
            .filter(comments::Column::PostId.eq(post_id))
            .order_by_desc(comments::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 获取待审核评论列表
    ///
    /// # 说明
    /// - is_reviewed = false
    /// - 按创建时间倒序
    /// - 用于管理后台审核
    pub async fn find_pending_review(&self) -> Result<Vec<comments::Model>, DbErr> {
        Comments::find()
            .filter(comments::Column::IsReviewed.eq(false))
            .order_by_desc(comments::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 分页获取所有评论（管理后台）
    ///
    /// # 参数
    /// - `page`: 页码（从 1 开始）
    /// - `size`: 每页数量
    /// - `is_reviewed`: 可选的审核状态过滤
    pub async fn find_all_paginated(
        &self,
        page: u64,
        size: u64,
        is_reviewed: Option<bool>,
    ) -> Result<(Vec<comments::Model>, u64), DbErr> {
        let mut query = Comments::find();

        if let Some(reviewed) = is_reviewed {
            query = query.filter(comments::Column::IsReviewed.eq(reviewed));
        }

        let paginator = query
            .order_by_desc(comments::Column::CreatedAt)
            .paginate(&self.db, size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 获取用户的所有评论
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    ///
    /// # 说明
    /// - 仅查询登录用户发表的评论
    pub async fn find_by_user_id(&self, user_id: i64) -> Result<Vec<comments::Model>, DbErr> {
        Comments::find()
            .filter(comments::Column::UserId.eq(user_id))
            .order_by_desc(comments::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 创建评论
    ///
    /// # 参数
    /// - `comment`: 评论 ActiveModel
    pub async fn create(&self, comment: comments::ActiveModel) -> Result<comments::Model, DbErr> {
        comment.insert(&self.db).await
    }

    /// 更新评论
    ///
    /// # 参数
    /// - `comment`: 包含更新字段的 ActiveModel
    pub async fn update(&self, comment: comments::ActiveModel) -> Result<comments::Model, DbErr> {
        comment.update(&self.db).await
    }

    /// 删除评论
    ///
    /// # 参数
    /// - `id`: 评论 ID
    ///
    /// # 注意
    /// - 会级联删除子评论（根据数据库外键设置）
    pub async fn delete(&self, id: i64) -> Result<DeleteResult, DbErr> {
        Comments::delete_by_id(id).exec(&self.db).await
    }

    /// 审核评论
    ///
    /// # 参数
    /// - `id`: 评论 ID
    /// - `is_approved`: true 表示通过，false 表示拒绝
    ///
    /// # 说明
    /// - 快捷方法，只更新 is_reviewed 字段
    pub async fn review(&self, id: i64, is_approved: bool) -> Result<comments::Model, DbErr> {
        let mut comment: comments::ActiveModel = Comments::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Comment {} not found", id)))?
            .into();

        comment.is_reviewed = Set(Some(is_approved));
        comment.update(&self.db).await
    }

    /// 批量审核评论
    ///
    /// # 参数
    /// - `ids`: 评论 ID 列表
    /// - `is_approved`: 审核结果
    pub async fn batch_review(&self, ids: Vec<i64>, is_approved: bool) -> Result<u64, DbErr> {
        if ids.is_empty() {
            return Ok(0);
        }

        let result = Comments::update_many()
            .filter(comments::Column::Id.is_in(ids))
            .col_expr(comments::Column::IsReviewed, Expr::value(is_approved))
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected)
    }

    /// 获取文章的评论数量
    ///
    /// # 参数
    /// - `post_id`: 文章 ID
    /// - `only_approved`: 是否只统计已审核评论
    pub async fn count_by_post_id(&self, post_id: i64, only_approved: bool) -> Result<u64, DbErr> {
        let mut query = Comments::find().filter(comments::Column::PostId.eq(post_id));

        if only_approved {
            query = query.filter(comments::Column::IsReviewed.eq(true));
        }

        query.count(&self.db).await
    }

    /// 获取待审核评论数量
    ///
    /// 用于管理后台提醒
    pub async fn count_pending_review(&self) -> Result<u64, DbErr> {
        Comments::find()
            .filter(comments::Column::IsReviewed.eq(false))
            .count(&self.db)
            .await
    }

    /// 获取评论总数
    pub async fn count_all(&self) -> Result<u64, DbErr> {
        Comments::find().count(&self.db).await
    }

    /// 获取最近的评论
    ///
    /// # 参数
    /// - `limit`: 数量限制
    ///
    /// # 用途
    /// - 管理后台仪表盘
    pub async fn find_recent(&self, limit: u64) -> Result<Vec<comments::Model>, DbErr> {
        Comments::find()
            .filter(comments::Column::IsReviewed.eq(true))
            .order_by_desc(comments::Column::CreatedAt)
            .limit(limit)
            .all(&self.db)
            .await
    }

    /// 根据 IP 查询评论
    ///
    /// # 参数
    /// - `ip`: IP 地址
    ///
    /// # 用途
    /// - 反垃圾评论
    /// - 查看某 IP 的评论历史
    pub async fn find_by_ip(&self, ip: &str) -> Result<Vec<comments::Model>, DbErr> {
        Comments::find()
            .filter(comments::Column::Ip.eq(ip))
            .order_by_desc(comments::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 检查评论是否存在
    ///
    /// # 参数
    /// - `id`: 评论 ID
    pub async fn exists(&self, id: i64) -> Result<bool, DbErr> {
        let count = Comments::find_by_id(id).count(&self.db).await?;
        Ok(count > 0)
    }
}
