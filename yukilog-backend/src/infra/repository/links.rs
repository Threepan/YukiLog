//! 友链仓储层
//!
//! 提供友链相关的数据访问操作

use crate::entities::{links, prelude::Links};
use sea_orm::*;

/// 友链仓储
#[derive(Clone)]
pub struct LinksRepository {
    db: DatabaseConnection,
}

impl LinksRepository {
    /// 创建新的友链仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 根据 ID 查询友链
    ///
    /// # 参数
    /// - `id`: 友链 ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<links::Model>, DbErr> {
        Links::find_by_id(id).one(&self.db).await
    }

    /// 获取所有已激活的友链
    ///
    /// # 说明
    /// - link_status = "active"
    /// - 按创建时间倒序
    /// - 用于前台展示
    pub async fn find_active(&self) -> Result<Vec<links::Model>, DbErr> {
        Links::find()
            .filter(links::Column::LinkStatus.eq("active"))
            .order_by_desc(links::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 获取所有友链（含各种状态）
    ///
    /// # 说明
    /// - 用于管理后台
    /// - 按创建时间倒序
    pub async fn find_all(&self) -> Result<Vec<links::Model>, DbErr> {
        Links::find()
            .order_by_desc(links::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 根据状态查询友链
    ///
    /// # 参数
    /// - `status`: 友链状态（active/pending/broken）
    pub async fn find_by_status(&self, status: &str) -> Result<Vec<links::Model>, DbErr> {
        Links::find()
            .filter(links::Column::LinkStatus.eq(status))
            .order_by_desc(links::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 分页获取友链
    ///
    /// # 参数
    /// - `page`: 页码（从 1 开始）
    /// - `size`: 每页数量
    /// - `status`: 可选的状态过滤
    pub async fn find_paginated(
        &self,
        page: u64,
        size: u64,
        status: Option<&str>,
    ) -> Result<(Vec<links::Model>, u64), DbErr> {
        let mut query = Links::find();

        if let Some(s) = status {
            query = query.filter(links::Column::LinkStatus.eq(s));
        }

        let paginator = query
            .order_by_desc(links::Column::CreatedAt)
            .paginate(&self.db, size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 根据 URL 查询友链
    ///
    /// # 参数
    /// - `url`: 友链 URL
    ///
    /// # 用途
    /// - 检查是否已存在相同 URL 的友链
    pub async fn find_by_url(&self, url: &str) -> Result<Option<links::Model>, DbErr> {
        Links::find()
            .filter(links::Column::LinkUrl.eq(url))
            .one(&self.db)
            .await
    }

    /// 检查 URL 是否已存在
    ///
    /// # 参数
    /// - `url`: 友链 URL
    pub async fn exists_by_url(&self, url: &str) -> Result<bool, DbErr> {
        let count = Links::find()
            .filter(links::Column::LinkUrl.eq(url))
            .count(&self.db)
            .await?;
        Ok(count > 0)
    }

    /// 创建友链
    ///
    /// # 参数
    /// - `link`: 友链 ActiveModel
    pub async fn create(&self, link: links::ActiveModel) -> Result<links::Model, DbErr> {
        link.insert(&self.db).await
    }

    /// 更新友链
    ///
    /// # 参数
    /// - `link`: 包含更新字段的 ActiveModel
    pub async fn update(&self, link: links::ActiveModel) -> Result<links::Model, DbErr> {
        link.update(&self.db).await
    }

    /// 删除友链
    ///
    /// # 参数
    /// - `id`: 友链 ID
    pub async fn delete(&self, id: i64) -> Result<DeleteResult, DbErr> {
        Links::delete_by_id(id).exec(&self.db).await
    }

    /// 更新友链状态
    ///
    /// # 参数
    /// - `id`: 友链 ID
    /// - `status`: 新状态（active/pending/broken）
    ///
    /// # 说明
    /// - 快捷方法，只更新状态字段
    pub async fn update_status(&self, id: i64, status: &str) -> Result<links::Model, DbErr> {
        let mut link: links::ActiveModel = Links::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Link {} not found", id)))?
            .into();

        link.link_status = Set(status.to_string());
        link.update(&self.db).await
    }

    /// 批量更新友链状态
    ///
    /// # 参数
    /// - `ids`: 友链 ID 列表
    /// - `status`: 新状态
    pub async fn batch_update_status(&self, ids: Vec<i64>, status: &str) -> Result<u64, DbErr> {
        if ids.is_empty() {
            return Ok(0);
        }

        let result = Links::update_many()
            .filter(links::Column::Id.is_in(ids))
            .col_expr(links::Column::LinkStatus, Expr::value(status))
            .exec(&self.db)
            .await?;

        Ok(result.rows_affected)
    }

    /// 获取待审核友链列表
    ///
    /// # 说明
    /// - link_status = "pending"
    /// - 用于管理后台审核
    pub async fn find_pending(&self) -> Result<Vec<links::Model>, DbErr> {
        self.find_by_status("pending").await
    }

    /// 获取待审核友链数量
    ///
    /// 用于管理后台提醒
    pub async fn count_pending(&self) -> Result<u64, DbErr> {
        Links::find()
            .filter(links::Column::LinkStatus.eq("pending"))
            .count(&self.db)
            .await
    }

    /// 获取已激活友链数量
    pub async fn count_active(&self) -> Result<u64, DbErr> {
        Links::find()
            .filter(links::Column::LinkStatus.eq("active"))
            .count(&self.db)
            .await
    }

    /// 获取友链总数
    pub async fn count_all(&self) -> Result<u64, DbErr> {
        Links::find().count(&self.db).await
    }

    /// 获取最近申请的友链
    ///
    /// # 参数
    /// - `limit`: 数量限制
    ///
    /// # 用途
    /// - 管理后台仪表盘
    pub async fn find_recent(&self, limit: u64) -> Result<Vec<links::Model>, DbErr> {
        Links::find()
            .order_by_desc(links::Column::CreatedAt)
            .limit(limit)
            .all(&self.db)
            .await
    }
}
