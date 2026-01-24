//! 标签仓储层
//!
//! 提供标签相关的数据访问操作

use crate::entities::{
    post_tags,
    prelude::{PostTags, Tags},
    tags,
};
use sea_orm::*;

/// 标签仓储
#[derive(Clone)]
pub struct TagsRepository {
    db: DatabaseConnection,
}

impl TagsRepository {
    /// 创建新的标签仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// 根据 ID 查询标签
    ///
    /// # 参数
    /// - `id`: 标签 ID
    pub async fn find_by_id(&self, id: i64) -> Result<Option<tags::Model>, DbErr> {
        Tags::find_by_id(id).one(&self.db).await
    }

    /// 根据 slug 查询标签
    ///
    /// # 参数
    /// - `slug`: 标签 URL slug
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<tags::Model>, DbErr> {
        Tags::find()
            .filter(tags::Column::Slug.eq(slug))
            .one(&self.db)
            .await
    }

    /// 根据名称查询标签
    ///
    /// # 参数
    /// - `name`: 标签名称
    pub async fn find_by_name(&self, name: &str) -> Result<Option<tags::Model>, DbErr> {
        Tags::find()
            .filter(tags::Column::Name.eq(name))
            .one(&self.db)
            .await
    }

    /// 获取所有标签
    ///
    /// # 说明
    /// - 按创建时间倒序
    pub async fn find_all(&self) -> Result<Vec<tags::Model>, DbErr> {
        Tags::find()
            .order_by_desc(tags::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    /// 获取标签云（含引用计数）
    ///
    /// # 返回
    /// - Vec<(标签Model, 引用次数)>
    ///
    /// # 说明
    /// - 用于前台展示标签云
    /// - 按引用次数倒序排列
    pub async fn find_all_with_count(&self) -> Result<Vec<(tags::Model, u64)>, DbErr> {
        let tags = self.find_all().await?;
        let mut result = Vec::new();

        for tag in tags {
            let count = PostTags::find()
                .filter(post_tags::Column::TagId.eq(tag.id))
                .count(&self.db)
                .await?;

            result.push((tag, count));
        }

        // 按引用次数倒序排序
        result.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(result)
    }

    /// 根据 ID 列表批量查询标签
    ///
    /// # 参数
    /// - `ids`: 标签 ID 列表
    ///
    /// # 用途
    /// - 获取文章的所有标签信息
    pub async fn find_by_ids(&self, ids: Vec<i64>) -> Result<Vec<tags::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        Tags::find()
            .filter(tags::Column::Id.is_in(ids))
            .all(&self.db)
            .await
    }

    /// 检查标签名称是否已存在
    ///
    /// # 参数
    /// - `name`: 标签名称
    pub async fn exists_by_name(&self, name: &str) -> Result<bool, DbErr> {
        let count = Tags::find()
            .filter(tags::Column::Name.eq(name))
            .count(&self.db)
            .await?;
        Ok(count > 0)
    }

    /// 检查标签 slug 是否已存在
    ///
    /// # 参数
    /// - `slug`: 标签 slug
    pub async fn exists_by_slug(&self, slug: &str) -> Result<bool, DbErr> {
        let count = Tags::find()
            .filter(tags::Column::Slug.eq(slug))
            .count(&self.db)
            .await?;
        Ok(count > 0)
    }

    /// 创建标签
    ///
    /// # 参数
    /// - `tag`: 标签 ActiveModel
    pub async fn create(&self, tag: tags::ActiveModel) -> Result<tags::Model, DbErr> {
        tag.insert(&self.db).await
    }

    /// 更新标签
    ///
    /// # 参数
    /// - `tag`: 包含更新字段的 ActiveModel
    pub async fn update(&self, tag: tags::ActiveModel) -> Result<tags::Model, DbErr> {
        tag.update(&self.db).await
    }

    /// 删除标签
    ///
    /// # 参数
    /// - `id`: 标签 ID
    ///
    /// # 注意
    /// - 会级联删除 post_tags 中的关联记录
    pub async fn delete(&self, id: i64) -> Result<DeleteResult, DbErr> {
        Tags::delete_by_id(id).exec(&self.db).await
    }

    /// 获取标签的引用次数
    ///
    /// # 参数
    /// - `tag_id`: 标签 ID
    ///
    /// # 说明
    /// - 统计使用该标签的文章数量
    pub async fn count_posts(&self, tag_id: i64) -> Result<u64, DbErr> {
        PostTags::find()
            .filter(post_tags::Column::TagId.eq(tag_id))
            .count(&self.db)
            .await
    }

    /// 获取标签总数
    pub async fn count_all(&self) -> Result<u64, DbErr> {
        Tags::find().count(&self.db).await
    }

    /// 查找或创建标签
    ///
    /// # 参数
    /// - `name`: 标签名称
    /// - `slug`: 标签 slug
    ///
    /// # 说明
    /// - 如果标签已存在，返回现有标签
    /// - 如果不存在，创建新标签
    /// - 常用于批量导入文章时自动创建标签
    pub async fn find_or_create(&self, name: &str, slug: &str) -> Result<tags::Model, DbErr> {
        if let Some(tag) = self.find_by_slug(slug).await? {
            return Ok(tag);
        }

        let new_tag = tags::ActiveModel {
            name: Set(name.to_string()),
            slug: Set(slug.to_string()),
            ..Default::default()
        };

        self.create(new_tag).await
    }
}
