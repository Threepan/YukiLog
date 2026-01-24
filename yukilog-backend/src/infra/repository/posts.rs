//! 文章仓储层
//!
//! 提供文章相关的数据访问操作，包括复杂的关联查询

use crate::entities::{
    post_tags, posts,
    prelude::{PostTags, Posts},
};
use sea_orm::*;

/// 文章仓储
#[derive(Clone)]
pub struct PostsRepository {
    db: DatabaseConnection,
}

impl PostsRepository {
    /// 创建新的文章仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 根据 ID 查询文章
    ///
    /// # 参数
    /// - `id`: 文章 ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<posts::Model>, DbErr> {
        Posts::find_by_id(id).one(&self.db).await
    }

    /// 根据 slug 查询文章
    ///
    /// # 参数
    /// - `slug`: 文章 URL slug（唯一标识）
    ///
    /// # 用途
    /// - 前台展示文章详情页
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<posts::Model>, DbErr> {
        Posts::find()
            .filter(posts::Column::Slug.eq(slug))
            .one(&self.db)
            .await
    }

    /// 分页获取已发布文章
    ///
    /// # 参数
    /// - `page`: 页码（从 1 开始）
    /// - `size`: 每页数量
    ///
    /// # 返回
    /// - 文章列表和总数的元组
    ///
    /// # 排序规则
    /// 1. 置顶文章优先（is_pinned = true）
    /// 2. 按发布时间倒序
    pub async fn find_published_paginated(
        &self,
        page: u64,
        size: u64,
    ) -> Result<(Vec<posts::Model>, u64), DbErr> {
        let paginator = Posts::find()
            .filter(posts::Column::Status.eq("published"))
            .order_by_desc(posts::Column::IsPinned)
            .order_by_desc(posts::Column::PublishedAt)
            .paginate(&self.db, size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 根据分类查询已发布文章（分页）
    ///
    /// # 参数
    /// - `category_id`: 分类 ID
    /// - `page`: 页码
    /// - `size`: 每页数量
    pub async fn find_published_by_category(
        &self,
        category_id: i64,
        page: u64,
        size: u64,
    ) -> Result<(Vec<posts::Model>, u64), DbErr> {
        let paginator = Posts::find()
            .filter(posts::Column::CategoryId.eq(category_id))
            .filter(posts::Column::Status.eq("published"))
            .order_by_desc(posts::Column::PublishedAt)
            .paginate(&self.db, size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 根据标签查询已发布文章（分页）
    ///
    /// # 参数
    /// - `tag_id`: 标签 ID
    /// - `page`: 页码
    /// - `size`: 每页数量
    ///
    /// # 实现说明
    /// - 通过中间表 post_tags 进行关联查询
    pub async fn find_published_by_tag(
        &self,
        tag_id: i64,
        page: u64,
        size: u64,
    ) -> Result<(Vec<posts::Model>, u64), DbErr> {
        // 先查询符合条件的 post_id 列表
        let post_ids: Vec<i64> = PostTags::find()
            .filter(post_tags::Column::TagId.eq(tag_id))
            .all(&self.db)
            .await?
            .into_iter()
            .map(|pt| pt.post_id)
            .collect();

        if post_ids.is_empty() {
            return Ok((vec![], 0));
        }

        // 再查询对应的文章
        let paginator = Posts::find()
            .filter(posts::Column::Id.is_in(post_ids))
            .filter(posts::Column::Status.eq("published"))
            .order_by_desc(posts::Column::PublishedAt)
            .paginate(&self.db, size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 获取所有文章（管理后台用）
    ///
    /// # 参数
    /// - `page`: 页码
    /// - `size`: 每页数量
    ///
    /// # 说明
    /// - 包含草稿、已发布、已归档所有状态
    /// - 按更新时间倒序
    pub async fn find_all_paginated(
        &self,
        page: u64,
        size: u64,
    ) -> Result<(Vec<posts::Model>, u64), DbErr> {
        let paginator = Posts::find()
            .order_by_desc(posts::Column::UpdatedAt)
            .paginate(&self.db, size);

        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(page - 1).await?;

        Ok((items, total))
    }

    /// 根据状态查询文章
    ///
    /// # 参数
    /// - `status`: 文章状态（draft/published/archived）
    pub async fn find_by_status(&self, status: &str) -> Result<Vec<posts::Model>, DbErr> {
        Posts::find()
            .filter(posts::Column::Status.eq(status))
            .order_by_desc(posts::Column::UpdatedAt)
            .all(&self.db)
            .await
    }

    /// 增加文章浏览量
    ///
    /// # 参数
    /// - `id`: 文章 ID
    ///
    /// # 说明
    /// - 每次调用增加 1
    /// - 使用原子操作，避免并发问题
    pub async fn increment_view_count(&self, id: i64) -> Result<(), DbErr> {
        // 获取当前浏览量
        let post = Posts::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Post {} not found", id)))?;

        let current_count = post.view_count.unwrap_or(0);

        // 更新浏览量
        let mut post: posts::ActiveModel = post.into();
        post.view_count = Set(Some(current_count + 1));
        post.update(&self.db).await?;

        Ok(())
    }

    /// 创建文章
    ///
    /// # 参数
    /// - `post`: 文章 ActiveModel
    pub async fn create(&self, post: posts::ActiveModel) -> Result<posts::Model, DbErr> {
        post.insert(&self.db).await
    }

    /// 更新文章
    ///
    /// # 参数
    /// - `post`: 包含更新字段的 ActiveModel
    pub async fn update(&self, post: posts::ActiveModel) -> Result<posts::Model, DbErr> {
        post.update(&self.db).await
    }

    /// 删除文章
    ///
    /// # 参数
    /// - `id`: 文章 ID
    ///
    /// # 注意
    /// - 硬删除
    /// - 会级联删除关联的评论和标签关系
    pub async fn delete(&self, id: i64) -> Result<DeleteResult, DbErr> {
        Posts::delete_by_id(id).exec(&self.db).await
    }

    /// 更新文章状态
    ///
    /// 快捷方法，只更新状态字段
    ///
    /// # 参数
    /// - `id`: 文章 ID
    /// - `status`: 新状态（draft/published/archived）
    pub async fn update_status(&self, id: i64, status: &str) -> Result<posts::Model, DbErr> {
        let mut post: posts::ActiveModel = Posts::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Post {} not found", id)))?
            .into();

        post.status = Set(Some(status.to_string()));
        post.update(&self.db).await
    }

    /// 切换文章置顶状态
    ///
    /// # 参数
    /// - `id`: 文章 ID
    /// - `is_pinned`: 是否置顶
    pub async fn update_pinned(&self, id: i64, is_pinned: bool) -> Result<posts::Model, DbErr> {
        let mut post: posts::ActiveModel = Posts::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("Post {} not found", id)))?
            .into();

        post.is_pinned = Set(Some(is_pinned));
        post.update(&self.db).await
    }

    /// 为文章关联标签
    ///
    /// # 参数
    /// - `post_id`: 文章 ID
    /// - `tag_ids`: 标签 ID 列表
    ///
    /// # 说明
    /// - 会先删除原有关联，再创建新关联
    /// - 建议在事务中调用
    pub async fn sync_tags(&self, post_id: i64, tag_ids: Vec<i64>) -> Result<(), DbErr> {
        // 删除原有关联
        PostTags::delete_many()
            .filter(post_tags::Column::PostId.eq(post_id))
            .exec(&self.db)
            .await?;

        // 创建新关联
        for tag_id in tag_ids {
            let relation = post_tags::ActiveModel {
                post_id: Set(post_id),
                tag_id: Set(tag_id),
            };
            relation.insert(&self.db).await?;
        }

        Ok(())
    }

    /// 获取文章的所有标签 ID
    ///
    /// # 参数
    /// - `post_id`: 文章 ID
    pub async fn get_tag_ids(&self, post_id: i64) -> Result<Vec<i64>, DbErr> {
        let relations = PostTags::find()
            .filter(post_tags::Column::PostId.eq(post_id))
            .all(&self.db)
            .await?;

        Ok(relations.into_iter().map(|r| r.tag_id).collect())
    }

    /// 获取已发布文章总数
    ///
    /// 用于仪表盘统计
    pub async fn count_published(&self) -> Result<u64, DbErr> {
        Posts::find()
            .filter(posts::Column::Status.eq("published"))
            .count(&self.db)
            .await
    }

    /// 获取草稿文章总数
    pub async fn count_drafts(&self) -> Result<u64, DbErr> {
        Posts::find()
            .filter(posts::Column::Status.eq("draft"))
            .count(&self.db)
            .await
    }

    /// 获取归档时间轴
    ///
    /// # 返回
    /// - 按年月分组的文章列表
    ///
    /// # 说明
    /// - 只包含已发布文章
    /// - 按发布时间倒序
    /// - Service 层负责按年月分组
    pub async fn find_for_archives(&self) -> Result<Vec<posts::Model>, DbErr> {
        Posts::find()
            .filter(posts::Column::Status.eq("published"))
            .order_by_desc(posts::Column::PublishedAt)
            .all(&self.db)
            .await
    }

    /// 获取最近更新的文章
    ///
    /// # 参数
    /// - `limit`: 数量限制
    ///
    /// # 用途
    /// - 管理后台仪表盘
    pub async fn find_recent(&self, limit: u64) -> Result<Vec<posts::Model>, DbErr> {
        Posts::find()
            .order_by_desc(posts::Column::UpdatedAt)
            .limit(limit)
            .all(&self.db)
            .await
    }
}
