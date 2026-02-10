//! 分类仓储层
//!
//! 提供分类相关的数据访问操作

use crate::entities::{
    categories, posts,
    prelude::{Categories, Posts},
};
use sea_orm::*;

/// 分类仓储
#[derive(Clone)]
pub struct CategoriesRepository {
    db: DatabaseConnection,
}

impl CategoriesRepository {
    /// 创建新的分类仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 根据 ID 查询分类
    ///
    /// # 参数
    /// - `id`: 分类 ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<categories::Model>, DbErr> {
        Categories::find_by_id(id).one(&self.db).await
    }

    /// 根据 slug 查询分类
    ///
    /// # 参数
    /// - `slug`: 分类 URL slug
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<categories::Model>, DbErr> {
        Categories::find()
            .filter(categories::Column::Slug.eq(slug))
            .one(&self.db)
            .await
    }

    /// 获取所有分类
    ///
    /// # 说明
    /// - 按创建时间倒序
    pub async fn find_all(&self) -> Result<Vec<categories::Model>, DbErr> {
        Categories::find()
            .order_by_desc(categories::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 批量根据 ID 查询分类
    ///
    /// # 说明
    /// - 用于避免 Service 层 N+1 查询
    /// - 返回结果不保证顺序
    pub async fn find_by_ids(&self, ids: Vec<i64>) -> Result<Vec<categories::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        Categories::find()
            .filter(categories::Column::Id.is_in(ids))
            .all(&self.db)
            .await
    }

    /// 获取所有分类及其文章数量
    ///
    /// # 返回
    /// - Vec<(分类Model, 文章数量)>
    ///
    /// # 说明
    /// - 只统计已发布文章
    /// - 用于前台展示分类列表
    pub async fn find_all_with_post_count(&self) -> Result<Vec<(categories::Model, u64)>, DbErr> {
        let categories = self.find_all().await?;
        let mut result = Vec::new();

        for category in categories {
            let count = Posts::find()
                .filter(posts::Column::CategoryId.eq(category.id))
                .filter(posts::Column::Status.eq("published"))
                .count(&self.db)
                .await?;

            result.push((category, count));
        }

        Ok(result)
    }

    /// 检查分类名称是否已存在
    ///
    /// # 参数
    /// - `name`: 分类名称
    pub async fn exists_by_name(&self, name: &str) -> Result<bool, DbErr> {
        let count = Categories::find()
            .filter(categories::Column::Name.eq(name))
            .count(&self.db)
            .await?;
        Ok(count > 0)
    }

    /// 检查分类 slug 是否已存在
    ///
    /// # 参数
    /// - `slug`: 分类 slug
    pub async fn exists_by_slug(&self, slug: &str) -> Result<bool, DbErr> {
        let count = Categories::find()
            .filter(categories::Column::Slug.eq(slug))
            .count(&self.db)
            .await?;
        Ok(count > 0)
    }

    /// 创建分类
    ///
    /// # 参数
    /// - `category`: 分类 ActiveModel
    pub async fn create(
        &self,
        category: categories::ActiveModel,
    ) -> Result<categories::Model, DbErr> {
        category.insert(&self.db).await
    }

    /// 更新分类
    ///
    /// # 参数
    /// - `category`: 包含更新字段的 ActiveModel
    pub async fn update(
        &self,
        category: categories::ActiveModel,
    ) -> Result<categories::Model, DbErr> {
        category.update(&self.db).await
    }

    /// 删除分类
    ///
    /// # 参数
    /// - `id`: 分类 ID
    ///
    /// # 注意
    /// - 删除前需要检查是否有文章关联
    /// - 根据数据库设置，可能会将关联文章的 category_id 设为 NULL
    pub async fn delete(&self, id: i64) -> Result<DeleteResult, DbErr> {
        Categories::delete_by_id(id).exec(&self.db).await
    }

    /// 获取分类下的文章数量
    ///
    /// # 参数
    /// - `category_id`: 分类 ID
    ///
    /// # 说明
    /// - 只统计已发布文章
    pub async fn count_posts(&self, category_id: i64) -> Result<u64, DbErr> {
        Posts::find()
            .filter(posts::Column::CategoryId.eq(category_id))
            .filter(posts::Column::Status.eq("published"))
            .count(&self.db)
            .await
    }

    /// 获取分类总数
    pub async fn count_all(&self) -> Result<u64, DbErr> {
        Categories::find().count(&self.db).await
    }
}
