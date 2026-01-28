use sea_orm::{ActiveValue, DatabaseConnection};

use crate::core::error::AppError;
use crate::entities::links;
use crate::infra::repository::links::LinksRepository;

use super::dto::{
    ApplyLinkRequest, BatchUpdateStatusRequest, LinkListResponse, LinkResponse, UpdateLinkRequest,
    UpdateStatusRequest,
};

/// 友链服务
pub struct LinksService {
    repo: LinksRepository,
}

impl LinksService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repo: LinksRepository::new(db),
        }
    }

    /// 获取所有已激活的友链 (前台展示)
    ///
    /// # 说明
    /// - 仅返回 status = "active" 的友链
    /// - 按创建时间倒序
    /// - 公开方法，无需认证
    pub async fn get_active_links(&self) -> Result<Vec<LinkResponse>, AppError> {
        let links = self.repo.find_active().await?;

        Ok(links
            .into_iter()
            .map(|link| self.to_response(link))
            .collect())
    }

    /// 申请友链 (公开接口)
    ///
    /// # 唯一性处理
    /// - 如果 URL 已存在且状态为 active/pending，返回错误
    /// - 如果 URL 已存在但状态为 broken，更新为 pending 重新申请
    ///
    /// # 默认状态
    /// - 新申请的友链状态为 "pending"，需要管理员审核
    pub async fn apply_link(&self, req: ApplyLinkRequest) -> Result<LinkResponse, AppError> {
        // 检查 URL 是否已存在
        if let Some(existing_link) = self.repo.find_by_url(&req.link_url).await? {
            match existing_link.link_status.as_str() {
                "broken" => {
                    // broken 状态允许重新申请，更新为 pending
                    let mut link_model: links::ActiveModel = existing_link.into();
                    link_model.link_title = ActiveValue::Set(req.link_title);
                    link_model.link_avatar = ActiveValue::Set(req.link_avatar);
                    link_model.link_desc = ActiveValue::Set(req.link_desc);
                    link_model.link_status = ActiveValue::Set("pending".to_string());

                    let updated_link = self.repo.update(link_model).await?;
                    Ok(self.to_response(updated_link))
                }
                "active" | "pending" => {
                    // 已激活或待审核，不允许重复申请
                    Err(AppError::Business(format!(
                        "该站点 URL 已存在，当前状态为: {}",
                        existing_link.link_status
                    )))
                }
                _ => {
                    // 未知状态，按新申请处理
                    let new_link = links::ActiveModel {
                        link_title: ActiveValue::Set(req.link_title),
                        link_url: ActiveValue::Set(req.link_url),
                        link_avatar: ActiveValue::Set(req.link_avatar),
                        link_desc: ActiveValue::Set(req.link_desc),
                        link_status: ActiveValue::Set("pending".to_string()),
                        ..Default::default()
                    };

                    let link = self.repo.create(new_link).await?;
                    Ok(self.to_response(link))
                }
            }
        } else {
            // URL 不存在，创建新友链
            let new_link = links::ActiveModel {
                link_title: ActiveValue::Set(req.link_title),
                link_url: ActiveValue::Set(req.link_url),
                link_avatar: ActiveValue::Set(req.link_avatar),
                link_desc: ActiveValue::Set(req.link_desc),
                link_status: ActiveValue::Set("pending".to_string()),
                ..Default::default()
            };

            let link = self.repo.create(new_link).await?;
            Ok(self.to_response(link))
        }
    }

    /// 获取单个友链详情 (管理后台)
    ///
    /// # 权限
    /// - 需要 Admin 权限
    pub async fn get_link_by_id(&self, id: i64) -> Result<LinkResponse, AppError> {
        let link = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("友链ID {} 不存在", id)))?;

        Ok(self.to_response(link))
    }

    /// 获取所有友链 (管理后台)
    ///
    /// # 说明
    /// - 返回所有状态的友链
    /// - 按创建时间倒序
    ///
    /// # 权限
    /// - 需要 Admin 权限
    pub async fn get_all_links(&self) -> Result<Vec<LinkResponse>, AppError> {
        let links = self.repo.find_all().await?;

        Ok(links
            .into_iter()
            .map(|link| self.to_response(link))
            .collect())
    }

    /// 分页获取友链 (管理后台)
    ///
    /// # 参数
    /// - `page`: 页码 (从1开始)
    /// - `size`: 每页数量
    /// - `status`: 可选的状态过滤 (active/pending/broken)
    ///
    /// # 权限
    /// - 需要 Admin 权限
    pub async fn get_links_paginated(
        &self,
        page: u64,
        size: u64,
        status: Option<String>,
    ) -> Result<LinkListResponse, AppError> {
        let (links, total) = self
            .repo
            .find_paginated(page, size, status.as_deref())
            .await?;

        Ok(LinkListResponse {
            links: links
                .into_iter()
                .map(|link| self.to_response(link))
                .collect(),
            total,
        })
    }

    /// 更新友链信息 (管理后台)
    ///
    /// # 说明
    /// - 仅更新提供的字段 (部分更新)
    ///
    /// # 权限
    /// - 当前版本: 仅 Admin 可更新
    /// - 未来扩展: 可支持申请人通过 email/token 验证后自主更新
    ///   (需要修改数据库schema添加申请人标识字段)
    pub async fn update_link(
        &self,
        id: i64,
        req: UpdateLinkRequest,
    ) -> Result<LinkResponse, AppError> {
        // 检查友链是否存在
        let current_link = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("友链ID {} 不存在", id)))?;

        // 更新字段
        let mut link_model: links::ActiveModel = current_link.into();
        if let Some(title) = req.link_title {
            link_model.link_title = ActiveValue::Set(title);
        }
        if let Some(avatar) = req.link_avatar {
            link_model.link_avatar = ActiveValue::Set(Some(avatar));
        }
        if let Some(desc) = req.link_desc {
            link_model.link_desc = ActiveValue::Set(Some(desc));
        }

        let updated_link = self.repo.update(link_model).await?;
        Ok(self.to_response(updated_link))
    }

    /// 更新单个友链状态 (管理后台)
    ///
    /// # 参数
    /// - `id`: 友链ID
    /// - `req.status`: 新状态 (active/pending/broken)
    ///
    /// # 权限
    /// - 需要 Admin 权限
    pub async fn update_status(
        &self,
        id: i64,
        req: UpdateStatusRequest,
    ) -> Result<LinkResponse, AppError> {
        let updated_link = self.repo.update_status(id, &req.status).await?;
        Ok(self.to_response(updated_link))
    }

    /// 批量更新友链状态 (管理后台)
    ///
    /// # 参数
    /// - `req.ids`: 友链ID列表
    /// - `req.status`: 新状态 (active/pending/broken)
    ///
    /// # 返回
    /// - 受影响的行数
    ///
    /// # 权限
    /// - 需要 Admin 权限
    pub async fn batch_update_status(
        &self,
        req: BatchUpdateStatusRequest,
    ) -> Result<u64, AppError> {
        let affected_rows = self.repo.batch_update_status(req.ids, &req.status).await?;
        Ok(affected_rows)
    }

    /// 删除友链 (管理后台)
    ///
    /// # 权限
    /// - 需要 Admin 权限
    pub async fn delete_link(&self, id: i64) -> Result<(), AppError> {
        // 检查友链是否存在
        if self.repo.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound(format!("友链ID {} 不存在", id)));
        }

        self.repo.delete(id).await?;
        Ok(())
    }

    /// 检查 URL 是否已存在
    ///
    /// # 说明
    /// - 用于前端实时校验
    pub async fn exists_by_url(&self, url: &str) -> Result<bool, AppError> {
        Ok(self.repo.exists_by_url(url).await?)
    }

    // ===== 辅助方法 =====

    /// 将 Model 转换为 Response
    fn to_response(&self, link: links::Model) -> LinkResponse {
        LinkResponse {
            id: link.id,
            link_title: link.link_title,
            link_url: link.link_url,
            link_avatar: link.link_avatar,
            link_desc: link.link_desc,
            link_status: link.link_status,
            created_at: link
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(chrono::Utc::now),
            updated_at: link
                .updated_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(chrono::Utc::now),
        }
    }
}
