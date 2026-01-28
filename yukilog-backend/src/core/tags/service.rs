use sea_orm::{ActiveValue, DatabaseConnection};

use crate::core::error::AppError;
use crate::entities::tags;
use crate::infra::repository::tags::TagsRepository;

use super::dto::{
    CreateTagRequest, FindOrCreateBatchResponse, TagCreationError, TagResponse,
    TagWithCountResponse, UpdateTagRequest,
};

/// 标签服务
pub struct TagsService {
    repo: TagsRepository,
}

impl TagsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repo: TagsRepository::new(db),
        }
    }

    /// 创建新标签
    ///
    /// # 唯一性检查
    /// - name必须唯一
    /// - slug必须唯一
    pub async fn create_tag(&self, req: CreateTagRequest) -> Result<TagResponse, AppError> {
        // 唯一性检查
        if self.repo.exists_by_name(&req.name).await? {
            return Err(AppError::Business(format!("标签名 '{}' 已存在", req.name)));
        }
        if self.repo.exists_by_slug(&req.slug).await? {
            return Err(AppError::Business(format!("Slug '{}' 已存在", req.slug)));
        }

        // 创建标签
        let new_tag = tags::ActiveModel {
            name: ActiveValue::Set(req.name),
            slug: ActiveValue::Set(req.slug),
            ..Default::default()
        };
        let tag = self.repo.create(new_tag).await?;

        Ok(TagResponse {
            id: tag.id as i32,
            name: tag.name,
            slug: tag.slug,
            created_at: tag
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(chrono::Utc::now),
        })
    }

    /// 根据ID获取标签
    pub async fn get_tag_by_id(&self, id: i32) -> Result<TagResponse, AppError> {
        let tag = self
            .repo
            .find_by_id(id as i64)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("标签ID {} 不存在", id)))?;

        Ok(TagResponse {
            id: tag.id as i32,
            name: tag.name,
            slug: tag.slug,
            created_at: tag
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(chrono::Utc::now),
        })
    }

    /// 根据Slug获取标签
    pub async fn get_tag_by_slug(&self, slug: &str) -> Result<TagResponse, AppError> {
        let tag = self
            .repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Slug '{}' 不存在", slug)))?;

        Ok(TagResponse {
            id: tag.id as i32,
            name: tag.name,
            slug: tag.slug,
            created_at: tag
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(chrono::Utc::now),
        })
    }

    /// 获取所有标签 (不分页)
    ///
    /// 按创建时间倒序排列
    pub async fn get_all_tags(&self) -> Result<Vec<TagResponse>, AppError> {
        let tags = self.repo.find_all().await?;

        Ok(tags
            .into_iter()
            .map(|tag| TagResponse {
                id: tag.id as i32,
                name: tag.name,
                slug: tag.slug,
                created_at: tag
                    .created_at
                    .as_ref()
                    .map(|dt| dt.to_utc())
                    .unwrap_or_else(chrono::Utc::now),
            })
            .collect())
    }

    /// 获取所有标签 (含文章数, 按引用次数倒序)
    ///
    /// 用于标签云展示
    pub async fn get_all_tags_with_count(&self) -> Result<Vec<TagWithCountResponse>, AppError> {
        let tags_with_count = self.repo.find_all_with_count().await?;

        Ok(tags_with_count
            .into_iter()
            .map(|(tag, count)| TagWithCountResponse {
                id: tag.id as i32,
                name: tag.name,
                slug: tag.slug,
                created_at: tag
                    .created_at
                    .as_ref()
                    .map(|dt| dt.to_utc())
                    .unwrap_or_else(chrono::Utc::now),
                post_count: count,
            })
            .collect())
    }

    /// 更新标签
    ///
    /// # 唯一性检查
    /// - 如果修改name, 新name必须唯一 (排除自身)
    /// - 如果修改slug, 新slug必须唯一 (排除自身)
    pub async fn update_tag(
        &self,
        id: i32,
        req: UpdateTagRequest,
    ) -> Result<TagResponse, AppError> {
        // 获取当前标签
        let current_tag = self
            .repo
            .find_by_id(id as i64)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("标签ID {} 不存在", id)))?;

        // 检查name唯一性 (排除自身)
        if let Some(ref new_name) = req.name {
            if new_name != &current_tag.name && self.repo.exists_by_name(new_name).await? {
                return Err(AppError::Business(format!("标签名 '{}' 已存在", new_name)));
            }
        }

        // 检查slug唯一性 (排除自身)
        if let Some(ref new_slug) = req.slug {
            if new_slug != &current_tag.slug && self.repo.exists_by_slug(new_slug).await? {
                return Err(AppError::Business(format!("Slug '{}' 已存在", new_slug)));
            }
        }

        // 执行更新
        let mut tag_model: tags::ActiveModel = current_tag.into();
        if let Some(name) = req.name {
            tag_model.name = ActiveValue::Set(name);
        }
        if let Some(slug) = req.slug {
            tag_model.slug = ActiveValue::Set(slug);
        }
        let updated_tag = self.repo.update(tag_model).await?;

        Ok(TagResponse {
            id: updated_tag.id as i32,
            name: updated_tag.name,
            slug: updated_tag.slug,
            created_at: updated_tag
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(chrono::Utc::now),
        })
    }

    /// 删除标签
    ///
    /// # 删除策略
    /// - 数据库外键设置: ON DELETE CASCADE
    /// - post_tags中的关联记录会被自动删除
    pub async fn delete_tag(&self, id: i32) -> Result<(), AppError> {
        // 检查标签是否存在
        if !self.repo.find_by_id(id as i64).await?.is_some() {
            return Err(AppError::NotFound(format!("标签ID {} 不存在", id)));
        }

        // 直接删除 (CASCADE会自动删除post_tags关联)
        self.repo.delete(id as i64).await?;

        Ok(())
    }

    /// 检查标签名是否存在
    pub async fn exists_by_name(&self, name: &str) -> Result<bool, AppError> {
        Ok(self.repo.exists_by_name(name).await?)
    }

    /// 检查Slug是否存在
    pub async fn exists_by_slug(&self, slug: &str) -> Result<bool, AppError> {
        Ok(self.repo.exists_by_slug(slug).await?)
    }

    /// 批量查找或创建标签 (部分成功模式)
    ///
    /// # 处理逻辑
    /// 1. 逐个处理每个标签
    /// 2. 优先通过slug查找, 不存在则创建
    /// 3. 创建失败的标签记录到errors
    /// 4. 返回成功的标签列表 + 失败列表
    ///
    /// # 使用场景
    /// - PostsService创建文章时批量处理标签
    /// - 允许部分标签创建失败, 不影响其他标签
    pub async fn find_or_create_batch(
        &self,
        tags: Vec<CreateTagRequest>,
    ) -> Result<FindOrCreateBatchResponse, AppError> {
        let mut results = Vec::new();
        let mut errors = Vec::new();

        for tag_req in tags {
            // 先尝试通过slug查找
            match self.repo.find_by_slug(&tag_req.slug).await {
                Ok(Some(existing_tag)) => {
                    // 标签已存在, 直接使用
                    results.push(TagResponse {
                        id: existing_tag.id as i32,
                        name: existing_tag.name,
                        slug: existing_tag.slug,
                        created_at: existing_tag
                            .created_at
                            .as_ref()
                            .map(|dt| dt.to_utc())
                            .unwrap_or_else(chrono::Utc::now),
                    });
                }
                Ok(None) => {
                    // 标签不存在, 尝试创建
                    match self.create_tag(tag_req.clone()).await {
                        Ok(new_tag) => {
                            results.push(new_tag);
                        }
                        Err(e) => {
                            // 创建失败, 记录错误
                            errors.push(TagCreationError {
                                name: tag_req.name,
                                slug: tag_req.slug,
                                reason: e.to_string(),
                            });
                        }
                    }
                }
                Err(e) => {
                    // 查询出错, 记录错误
                    errors.push(TagCreationError {
                        name: tag_req.name,
                        slug: tag_req.slug,
                        reason: e.to_string(),
                    });
                }
            }
        }

        Ok(FindOrCreateBatchResponse {
            tags: results,
            errors,
        })
    }
}
