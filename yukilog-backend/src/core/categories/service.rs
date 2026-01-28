//! 分类服务
//!
//! 提供分类管理相关的业务逻辑

use sea_orm::Set;
use validator::Validate;

use crate::core::error::AppError;
use crate::entities::categories;
use crate::infra::repository::CategoriesRepository;

use super::dto::{CategoryResponse, CreateCategoryRequest, UpdateCategoryRequest};

/// 分类服务
///
/// # 依赖
/// - `CategoriesRepository`: 分类数据访问
///
/// # 线程安全
/// 所有字段都是 Clone + Send + Sync，可安全在多线程中使用
#[derive(Clone)]
pub struct CategoriesService {
    /// 分类仓储
    category_repo: CategoriesRepository,
}

impl CategoriesService {
    /// 创建分类服务实例
    ///
    /// # 参数
    /// - `category_repo`: 分类仓储实例
    pub fn new(category_repo: CategoriesRepository) -> Self {
        Self { category_repo }
    }

    // ==================== 基础 CRUD ====================

    /// 创建分类
    ///
    /// # 参数
    /// - `req`: 创建分类请求
    ///
    /// # 返回
    /// - 成功：返回分类详情（post_count = 0）
    /// - 失败：返回 `AppError`
    ///
    /// # 流程
    /// 1. 校验输入参数
    /// 2. 检查 name 唯一性
    /// 3. 检查 slug 唯一性
    /// 4. 创建分类记录
    ///
    /// # 错误
    /// - `Validation(...)`: 输入参数校验失败
    /// - `Business("分类名称已存在")`: name 冲突
    /// - `Business("Slug 已存在")`: slug 冲突
    pub async fn create_category(
        &self,
        req: CreateCategoryRequest,
    ) -> Result<CategoryResponse, AppError> {
        // 1. 校验输入
        req.validate()
            .map_err(|e| AppError::Validation(format!("输入校验失败: {}", e)))?;

        // 2. 检查 name 唯一性
        if self.category_repo.exists_by_name(&req.name).await? {
            return Err(AppError::Business("分类名称已存在".to_string()));
        }

        // 3. 检查 slug 唯一性
        if self.category_repo.exists_by_slug(&req.slug).await? {
            return Err(AppError::Business("Slug 已存在".to_string()));
        }

        // 4. 创建分类
        let new_category = categories::ActiveModel {
            name: Set(req.name),
            slug: Set(req.slug),
            description: Set(req.description),
            ..Default::default()
        };

        let category = self.category_repo.create(new_category).await?;

        // 新创建的分类文章数为 0
        Ok(CategoryResponse::from_entity_with_count(&category, 0))
    }

    /// 获取分类详情（根据 ID）
    ///
    /// # 参数
    /// - `category_id`: 分类 ID
    ///
    /// # 返回
    /// - 成功：返回分类详情（含文章数）
    /// - 失败：返回 `AppError::NotFound`
    pub async fn get_category_by_id(&self, category_id: i64) -> Result<CategoryResponse, AppError> {
        // 1. 查询分类
        let category = self
            .category_repo
            .find_by_id(category_id)
            .await?
            .ok_or_else(|| AppError::NotFound("分类不存在".to_string()))?;

        // 2. 查询文章数
        let post_count = self.category_repo.count_posts(category_id).await?;

        Ok(CategoryResponse::from_entity_with_count(
            &category, post_count,
        ))
    }

    /// 获取分类详情（根据 slug）
    ///
    /// # 参数
    /// - `slug`: 分类 slug
    ///
    /// # 返回
    /// - 成功：返回分类详情（含文章数）
    /// - 失败：返回 `AppError::NotFound`
    ///
    /// # 使用场景
    /// - 前台根据 URL 路径（如 `/category/rust-programming`）获取分类
    pub async fn get_category_by_slug(&self, slug: &str) -> Result<CategoryResponse, AppError> {
        // 1. 查询分类
        let category = self
            .category_repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| AppError::NotFound("分类不存在".to_string()))?;

        // 2. 查询文章数
        let post_count = self.category_repo.count_posts(category.id).await?;

        Ok(CategoryResponse::from_entity_with_count(
            &category, post_count,
        ))
    }

    /// 获取所有分类（带文章数）
    ///
    /// # 返回
    /// - 成功：返回所有分类列表
    ///
    /// # 使用场景
    /// - 前台分类列表
    /// - 后台分类管理列表
    ///
    /// # 说明
    /// - 不分页，返回所有分类
    /// - 按创建时间倒序
    /// - 文章数仅统计已发布文章
    pub async fn get_all_categories(&self) -> Result<Vec<CategoryResponse>, AppError> {
        // 调用 Repository 的 find_all_with_post_count
        let categories_with_count = self.category_repo.find_all_with_post_count().await?;

        // 转换为 CategoryResponse
        let responses: Vec<CategoryResponse> = categories_with_count
            .into_iter()
            .map(|(category, count)| CategoryResponse::from_entity_with_count(&category, count))
            .collect();

        Ok(responses)
    }

    /// 更新分类
    ///
    /// # 参数
    /// - `category_id`: 分类 ID
    /// - `req`: 更新分类请求
    ///
    /// # 流程
    /// 1. 校验输入
    /// 2. 查询分类（检查是否存在）
    /// 3. 检查 name 唯一性（排除自身）
    /// 4. 检查 slug 唯一性（排除自身）
    /// 5. 更新字段（仅更新非 None 的字段）
    ///
    /// # 错误
    /// - `Validation(...)`: 输入校验失败
    /// - `NotFound("分类不存在")`: 分类不存在
    /// - `Business("分类名称已存在")`: name 冲突
    /// - `Business("Slug 已存在")`: slug 冲突
    pub async fn update_category(
        &self,
        category_id: i64,
        req: UpdateCategoryRequest,
    ) -> Result<CategoryResponse, AppError> {
        // 1. 校验输入
        req.validate()
            .map_err(|e| AppError::Validation(format!("输入校验失败: {}", e)))?;

        // 2. 查询分类
        let category = self
            .category_repo
            .find_by_id(category_id)
            .await?
            .ok_or_else(|| AppError::NotFound("分类不存在".to_string()))?;

        // 3. 检查 name 唯一性（排除自身）
        if let Some(ref new_name) = req.name {
            if new_name != &category.name {
                if self.category_repo.exists_by_name(new_name).await? {
                    return Err(AppError::Business("分类名称已存在".to_string()));
                }
            }
        }

        // 4. 检查 slug 唯一性（排除自身）
        if let Some(ref new_slug) = req.slug {
            if new_slug != &category.slug {
                if self.category_repo.exists_by_slug(new_slug).await? {
                    return Err(AppError::Business("Slug 已存在".to_string()));
                }
            }
        }

        // 5. 更新字段（部分更新）
        let mut active_category: categories::ActiveModel = category.into();

        if let Some(name) = req.name {
            active_category.name = Set(name);
        }

        if let Some(slug) = req.slug {
            active_category.slug = Set(slug);
        }

        if let Some(description) = req.description {
            active_category.description = Set(Some(description));
        }

        // 6. 保存
        let updated_category = self.category_repo.update(active_category).await?;

        // 7. 查询文章数
        let post_count = self.category_repo.count_posts(category_id).await?;

        Ok(CategoryResponse::from_entity_with_count(
            &updated_category,
            post_count,
        ))
    }

    /// 删除分类
    ///
    /// # 参数
    /// - `category_id`: 分类 ID
    ///
    /// # 流程
    /// 1. 检查分类是否存在
    /// 2. 删除分类
    ///
    /// # 说明
    /// - 数据库外键设置为 `ON DELETE SET NULL`
    /// - 删除分类后，使用该分类的文章的 `category_id` 会被设为 NULL
    /// - 文章本身不会被删除
    ///
    /// # 错误
    /// - `NotFound("分类不存在")`: 分类不存在
    pub async fn delete_category(&self, category_id: i64) -> Result<(), AppError> {
        // 1. 检查分类是否存在
        if self.category_repo.find_by_id(category_id).await?.is_none() {
            return Err(AppError::NotFound("分类不存在".to_string()));
        }

        // 2. 删除分类（数据库会自动将关联文章的 category_id 设为 NULL）
        self.category_repo.delete(category_id).await?;

        Ok(())
    }

    // ==================== 工具方法 ====================

    /// 检查分类名称是否存在
    ///
    /// # 使用场景
    /// - 创建/更新分类时校验唯一性
    /// - 前端实时校验
    pub async fn exists_by_name(&self, name: &str) -> Result<bool, AppError> {
        self.category_repo
            .exists_by_name(name)
            .await
            .map_err(AppError::Database)
    }

    /// 检查分类 slug 是否存在
    ///
    /// # 使用场景
    /// - 创建/更新分类时校验唯一性
    /// - 前端实时校验
    pub async fn exists_by_slug(&self, slug: &str) -> Result<bool, AppError> {
        self.category_repo
            .exists_by_slug(slug)
            .await
            .map_err(AppError::Database)
    }
}
