use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection};
use std::collections::HashMap;

use crate::core::error::AppError;
use crate::core::validation::validate_pagination;
use crate::entities::posts;
use crate::infra::repository::{
    categories::CategoriesRepository, posts::PostsRepository, tags::TagsRepository,
    users::UsersRepository,
};

use super::dto::{
    ArchiveGroup, AuthorInfo, CategoryInfo, CreatePostRequest, PostArchiveItem, PostDetailResponse,
    PostListItemResponse, PostListResponse, PublishPostRequest, SyncTagsRequest, TagInfo,
    UpdatePostRequest,
};

/// 文章服务
pub struct PostsService {
    repo: PostsRepository,
    categories_repo: CategoriesRepository,
    tags_repo: TagsRepository,
    users_repo: UsersRepository,
}

impl PostsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repo: PostsRepository::new(db.clone()),
            categories_repo: CategoriesRepository::new(db.clone()),
            tags_repo: TagsRepository::new(db.clone()),
            users_repo: UsersRepository::new(db),
        }
    }

    // ===== 前台公开方法 =====

    /// 获取已发布文章列表 (分页)
    ///
    /// # 说明
    /// - 仅返回 status = "published" 的文章
    /// - 置顶文章优先
    /// - 按发布时间倒序
    pub async fn get_published_posts(
        &self,
        page: u64,
        size: u64,
    ) -> Result<PostListResponse, AppError> {
        validate_pagination(page, size)?;
        let (posts, total) = self.repo.find_published_paginated(page, size).await?;

        let items = self.posts_to_list_items(posts).await?;

        Ok(PostListResponse {
            posts: items,
            total,
        })
    }

    /// 根据 Slug 获取文章详情 (并增加浏览量)
    ///
    /// # 说明
    /// - 仅返回已发布的文章
    /// - 每次访问自动增加浏览量
    pub async fn get_post_by_slug(&self, slug: &str) -> Result<PostDetailResponse, AppError> {
        let post = self
            .repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章 '{}' 不存在", slug)))?;

        // 检查状态
        if post.status.as_deref() != Some("published") {
            return Err(AppError::NotFound(format!("文章 '{}' 未发布", slug)));
        }

        // 增加浏览量
        self.repo.increment_view_count(post.id).await?;

        // 重新查询以获取更新后的浏览量（并发/删除场景下不崩溃）
        let post = self
            .repo
            .find_by_slug(slug)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章 '{}' 不存在", slug)))?;

        self.post_to_detail(post).await
    }

    /// 按分类获取已发布文章 (分页)
    pub async fn get_posts_by_category(
        &self,
        category_id: i64,
        page: u64,
        size: u64,
    ) -> Result<PostListResponse, AppError> {
        validate_pagination(page, size)?;
        let (posts, total) = self
            .repo
            .find_published_by_category(category_id, page, size)
            .await?;

        let items = self.posts_to_list_items(posts).await?;

        Ok(PostListResponse {
            posts: items,
            total,
        })
    }

    /// 按标签获取已发布文章 (分页)
    pub async fn get_posts_by_tag(
        &self,
        tag_id: i64,
        page: u64,
        size: u64,
    ) -> Result<PostListResponse, AppError> {
        validate_pagination(page, size)?;
        let (posts, total) = self.repo.find_published_by_tag(tag_id, page, size).await?;

        let items = self.posts_to_list_items(posts).await?;

        Ok(PostListResponse {
            posts: items,
            total,
        })
    }

    /// 获取归档时间轴 (按年月分组)
    ///
    /// # 说明
    /// - 仅包含已发布文章
    /// - 按发布时间倒序
    /// - 分组格式: 2024-01, 2024-02
    pub async fn get_archives(&self) -> Result<Vec<ArchiveGroup>, AppError> {
        let posts = self.repo.find_for_archives().await?;

        // 按年月分组
        let mut groups: HashMap<String, Vec<PostArchiveItem>> = HashMap::new();

        for post in posts {
            if let Some(published_at) = post.published_at {
                let year_month = published_at.format("%Y-%m").to_string();
                let item = PostArchiveItem {
                    id: post.id,
                    title: post.title,
                    slug: post.slug,
                    published_at: published_at.to_utc(),
                };

                groups.entry(year_month).or_insert_with(Vec::new).push(item);
            }
        }

        // 转换为列表并排序
        let mut result: Vec<ArchiveGroup> = groups
            .into_iter()
            .map(|(year_month, posts)| ArchiveGroup { year_month, posts })
            .collect();

        result.sort_by(|a, b| b.year_month.cmp(&a.year_month));

        Ok(result)
    }

    // ===== 后台管理方法 =====

    /// 创建文章 (默认草稿状态)
    ///
    /// # 唯一性检查
    /// - slug 必须唯一
    ///
    /// # 标签同步
    /// - 如果提供了 tag_ids，自动同步标签
    pub async fn create_post(
        &self,
        user_id: i64,
        req: CreatePostRequest,
    ) -> Result<PostDetailResponse, AppError> {
        // Slug唯一性检查
        if self.repo.find_by_slug(&req.slug).await?.is_some() {
            return Err(AppError::Business(format!("Slug '{}' 已被使用", req.slug)));
        }

        // 如果指定了分类，检查分类是否存在
        if let Some(category_id) = req.category_id {
            if self
                .categories_repo
                .find_by_id(category_id)
                .await?
                .is_none()
            {
                return Err(AppError::NotFound(format!("分类ID {} 不存在", category_id)));
            }
        }

        // 创建文章
        let new_post = posts::ActiveModel {
            title: ActiveValue::Set(req.title),
            sub_title: ActiveValue::Set(req.sub_title),
            slug: ActiveValue::Set(req.slug),
            summary: ActiveValue::Set(req.summary),
            content: ActiveValue::Set(req.content),
            cover_image: ActiveValue::Set(req.cover_image),
            status: ActiveValue::Set(Some(req.status.unwrap_or_else(|| "draft".to_string()))),
            category_id: ActiveValue::Set(req.category_id),
            user_id: ActiveValue::Set(Some(user_id)),
            view_count: ActiveValue::Set(Some(0)),
            is_pinned: ActiveValue::Set(Some(false)),
            ..Default::default()
        };

        let post = self.repo.create(new_post).await?;

        // 同步标签
        if let Some(tag_ids) = req.tag_ids {
            self.repo.sync_tags(post.id, tag_ids).await?;
        }

        self.post_to_detail(post).await
    }

    /// 更新文章
    ///
    /// # Slug唯一性检查
    /// - 如果修改 slug，新 slug 必须唯一（排除自身）
    ///
    /// # 标签同步
    /// - 如果提供了 tag_ids，自动同步标签
    pub async fn update_post(
        &self,
        id: i64,
        req: UpdatePostRequest,
    ) -> Result<PostDetailResponse, AppError> {
        // 获取当前文章
        let current_post = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章ID {} 不存在", id)))?;

        // Slug唯一性检查（排除自身）
        if let Some(ref new_slug) = req.slug {
            if new_slug != &current_post.slug {
                if self.repo.find_by_slug(new_slug).await?.is_some() {
                    return Err(AppError::Business(format!("Slug '{}' 已被使用", new_slug)));
                }
            }
        }

        // 如果指定了新分类，检查分类是否存在
        if let Some(category_id) = req.category_id {
            if self
                .categories_repo
                .find_by_id(category_id)
                .await?
                .is_none()
            {
                return Err(AppError::NotFound(format!("分类ID {} 不存在", category_id)));
            }
        }

        // 更新字段
        let mut post_model: posts::ActiveModel = current_post.into();
        if let Some(title) = req.title {
            post_model.title = ActiveValue::Set(title);
        }
        if let Some(sub_title) = req.sub_title {
            post_model.sub_title = ActiveValue::Set(Some(sub_title));
        }
        if let Some(slug) = req.slug {
            post_model.slug = ActiveValue::Set(slug);
        }
        if let Some(summary) = req.summary {
            post_model.summary = ActiveValue::Set(Some(summary));
        }
        if let Some(content) = req.content {
            post_model.content = ActiveValue::Set(content);
        }
        if let Some(cover_image) = req.cover_image {
            post_model.cover_image = ActiveValue::Set(Some(cover_image));
        }
        if let Some(category_id) = req.category_id {
            post_model.category_id = ActiveValue::Set(Some(category_id));
        }
        if let Some(status) = req.status {
            post_model.status = ActiveValue::Set(Some(status));
        }

        let updated_post = self.repo.update(post_model).await?;

        // 同步标签
        if let Some(tag_ids) = req.tag_ids {
            self.repo.sync_tags(updated_post.id, tag_ids).await?;
        }

        self.post_to_detail(updated_post).await
    }

    /// 发布文章
    ///
    /// # 说明
    /// - 将状态改为 "published"
    /// - 设置 published_at（默认当前时间，可指定）
    pub async fn publish_post(
        &self,
        id: i64,
        req: Option<PublishPostRequest>,
    ) -> Result<PostDetailResponse, AppError> {
        let current_post = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章ID {} 不存在", id)))?;

        let mut post_model: posts::ActiveModel = current_post.into();
        post_model.status = ActiveValue::Set(Some("published".to_string()));

        // 设置发布时间
        let published_at = req
            .and_then(|r| r.published_at)
            .unwrap_or_else(|| Utc::now());
        post_model.published_at = ActiveValue::Set(Some(published_at.into()));

        let updated_post = self.repo.update(post_model).await?;

        self.post_to_detail(updated_post).await
    }

    /// 取消发布 (改为草稿)
    pub async fn unpublish_post(&self, id: i64) -> Result<PostDetailResponse, AppError> {
        let updated_post = self.repo.update_status(id, "draft").await?;
        self.post_to_detail(updated_post).await
    }

    /// 归档文章
    pub async fn archive_post(&self, id: i64) -> Result<PostDetailResponse, AppError> {
        let updated_post = self.repo.update_status(id, "archived").await?;
        self.post_to_detail(updated_post).await
    }

    /// 删除文章
    ///
    /// # 说明
    /// - 硬删除
    /// - 数据库外键设置 CASCADE，会自动删除关联的评论和标签关系
    pub async fn delete_post(&self, id: i64) -> Result<(), AppError> {
        // 检查文章是否存在
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound(format!("文章ID {} 不存在", id)));
        }

        self.repo.delete(id).await?;
        Ok(())
    }

    /// 切换置顶状态
    pub async fn toggle_pin(
        &self,
        id: i64,
        is_pinned: bool,
    ) -> Result<PostDetailResponse, AppError> {
        let updated_post = self.repo.update_pinned(id, is_pinned).await?;
        self.post_to_detail(updated_post).await
    }

    /// 同步文章标签 (独立接口)
    ///
    /// # 说明
    /// - 会先删除原有关联，再创建新关联
    pub async fn sync_tags(&self, post_id: i64, req: SyncTagsRequest) -> Result<(), AppError> {
        // 检查文章是否存在
        if self.repo.find_by_id(post_id).await?.is_none() {
            return Err(AppError::NotFound(format!("文章ID {} 不存在", post_id)));
        }

        self.repo.sync_tags(post_id, req.tag_ids).await?;
        Ok(())
    }

    /// 获取所有文章 (管理后台)
    ///
    /// # 参数
    /// - `page`: 页码
    /// - `size`: 每页数量
    /// - `status`: 可选的状态过滤
    ///
    /// # 说明
    /// - 包含所有状态的文章
    /// - 按更新时间倒序
    pub async fn get_all_posts(
        &self,
        page: u64,
        size: u64,
        status: Option<String>,
    ) -> Result<PostListResponse, AppError> {
        validate_pagination(page, size)?;

        let (posts, total) = if let Some(status_filter) = status {
            // 按状态过滤（仍然分页，避免一次性返回过多数据）
            let all = self.repo.find_by_status(&status_filter).await?;
            let total = all.len() as u64;

            let offset = (page - 1)
                .checked_mul(size)
                .ok_or_else(|| AppError::BadRequest("page 参数过大".to_string()))?;
            let offset_usize = usize::try_from(offset)
                .map_err(|_| AppError::BadRequest("page 参数过大".to_string()))?;
            let size_usize = usize::try_from(size)
                .map_err(|_| AppError::BadRequest("size 参数过大".to_string()))?;

            let items = if offset_usize >= all.len() {
                vec![]
            } else {
                let end = (offset_usize + size_usize).min(all.len());
                all[offset_usize..end].to_vec()
            };

            (items, total)
        } else {
            // 获取所有文章
            self.repo.find_all_paginated(page, size).await?
        };

        let items = self.posts_to_list_items(posts).await?;

        Ok(PostListResponse { posts: items, total })
    }

    /// 根据ID获取文章 (管理后台)
    ///
    /// # 说明
    /// - 不限制状态
    /// - 不增加浏览量
    pub async fn get_post_by_id(&self, id: i64) -> Result<PostDetailResponse, AppError> {
        let post = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("文章ID {} 不存在", id)))?;

        self.post_to_detail(post).await
    }

    // ===== 辅助方法 =====

    /// 将 Model 转换为 DetailResponse
    async fn post_to_detail(&self, post: posts::Model) -> Result<PostDetailResponse, AppError> {
        // 获取分类信息
        let category = if let Some(cat_id) = post.category_id {
            self.categories_repo
                .find_by_id(cat_id)
                .await?
                .map(|c| CategoryInfo {
                    id: c.id,
                    name: c.name,
                    slug: c.slug,
                })
        } else {
            None
        };

        // 获取标签信息
        let tag_ids = self.repo.get_tag_ids(post.id).await?;
        let tags = self.tags_repo.find_by_ids(tag_ids).await?;
        let tags_info: Vec<TagInfo> = tags
            .into_iter()
            .map(|t| TagInfo {
                id: t.id,
                name: t.name,
                slug: t.slug,
            })
            .collect();

        // 获取作者信息
        let author = if let Some(user_id) = post.user_id {
            self.users_repo
                .find_by_id(user_id)
                .await?
                .map(|u| AuthorInfo {
                    id: u.id,
                    username: u.username,
                    nickname: u.nickname,
                    avatar_url: u.avatar_url,
                })
        } else {
            None
        };

        Ok(PostDetailResponse {
            id: post.id,
            title: post.title,
            sub_title: post.sub_title,
            slug: post.slug,
            summary: post.summary,
            content: post.content,
            cover_image: post.cover_image,
            status: post.status.unwrap_or_else(|| "draft".to_string()),
            category,
            tags: tags_info,
            author,
            view_count: post.view_count.unwrap_or(0),
            is_pinned: post.is_pinned.unwrap_or(false),
            published_at: post.published_at.map(|dt| dt.to_utc()),
            created_at: post
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
            updated_at: post
                .updated_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
        })
    }

    /// 批量转换为 ListItemResponse
    async fn posts_to_list_items(
        &self,
        posts: Vec<posts::Model>,
    ) -> Result<Vec<PostListItemResponse>, AppError> {
        // 批量预加载分类/作者，减少 N+1 查询
        let category_ids: Vec<i64> = posts.iter().filter_map(|p| p.category_id).collect();
        let user_ids: Vec<i64> = posts.iter().filter_map(|p| p.user_id).collect();

        let categories = self.categories_repo.find_by_ids(category_ids).await?;
        let users = self.users_repo.find_by_ids(user_ids).await?;

        let categories_map: HashMap<i64, CategoryInfo> = categories
            .into_iter()
            .map(|c| {
                (
                    c.id,
                    CategoryInfo {
                        id: c.id,
                        name: c.name,
                        slug: c.slug,
                    },
                )
            })
            .collect();

        let users_map: HashMap<i64, AuthorInfo> = users
            .into_iter()
            .map(|u| {
                (
                    u.id,
                    AuthorInfo {
                        id: u.id,
                        username: u.username,
                        nickname: u.nickname,
                        avatar_url: u.avatar_url,
                    },
                )
            })
            .collect();

        let mut items = Vec::new();

        for post in posts {
            // 获取分类信息
            let category = post
                .category_id
                .and_then(|cat_id| categories_map.get(&cat_id).cloned());

            // 获取标签信息
            let tag_ids = self.repo.get_tag_ids(post.id).await?;
            let tags = self.tags_repo.find_by_ids(tag_ids).await?;
            let tags_info: Vec<TagInfo> = tags
                .into_iter()
                .map(|t| TagInfo {
                    id: t.id,
                    name: t.name,
                    slug: t.slug,
                })
                .collect();

            // 获取作者信息
            let author = post
                .user_id
                .and_then(|user_id| users_map.get(&user_id).cloned());

            items.push(PostListItemResponse {
                id: post.id,
                title: post.title,
                sub_title: post.sub_title,
                slug: post.slug,
                summary: post.summary,
                cover_image: post.cover_image,
                status: post.status.unwrap_or_else(|| "draft".to_string()),
                category,
                tags: tags_info,
                author,
                view_count: post.view_count.unwrap_or(0),
                is_pinned: post.is_pinned.unwrap_or(false),
                published_at: post.published_at.map(|dt| dt.to_utc()),
                created_at: post
                    .created_at
                    .as_ref()
                    .map(|dt| dt.to_utc())
                    .unwrap_or_else(Utc::now),
                updated_at: post
                    .updated_at
                    .as_ref()
                    .map(|dt| dt.to_utc())
                    .unwrap_or_else(Utc::now),
            });
        }

        Ok(items)
    }
}
