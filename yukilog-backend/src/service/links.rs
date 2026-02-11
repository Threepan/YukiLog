use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::domain::status::LinkStatus;
use crate::entities::prelude::Links;
use crate::entities::links::Column as LinkColumn;
use crate::repo;
use crate::repo::links::{CreateLink as RepoCreateLink, UpdateLink as RepoUpdateLink};
use crate::service::error::{ServiceError, ServiceResult};

// ================================
// DTO 定义
// ================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Link {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub status: LinkStatus,
    pub created_at: DateTime<FixedOffset>,
}

impl From<repo::links::LinkDto> for Link {
    fn from(dto: repo::links::LinkDto) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
            url: dto.url,
            avatar: dto.avatar,
            description: dto.description,
            status: dto.status.unwrap_or(LinkStatus::Pending),
            created_at: dto.created_at.unwrap_or_else(|| chrono::Utc::now().into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLinkInput {
    pub title: String,
    pub url: String,
    pub avatar: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateLinkInput {
    pub title: Option<String>,
    pub url: Option<String>,
    pub avatar: Option<Option<String>>,
    pub description: Option<Option<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct LinkFilter {
    pub status: Option<LinkStatus>,
    pub sort_by: Option<LinkSortBy>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkSortBy {
    CreatedAtAsc,   // 按创建时间正序
    CreatedAtDesc,  // 按创建时间倒序（默认）
}

// ================================
// 业务逻辑
// ================================

/// 1. 申请友链（前台）
/// 
/// 逻辑：
/// - 验证 URL 格式（http:// 或 https://）
/// - 检查 URL 是否已存在：
///   - 不存在：创建新友链，status 默认 pending
///   - 存在且 broken：更新信息并置为 pending（重新审核）
///   - 存在且非 broken：返回错误（已存在）
pub async fn create_link_application(
    db: &DatabaseConnection,
    input: CreateLinkInput,
) -> ServiceResult<Link> {
    // 验证 URL 格式
    if !is_valid_url(&input.url) {
        return Err(ServiceError::InvalidInput(
            "url must start with http:// or https://".to_string(),
        ));
    }

    // 检查 URL 是否已存在
    match repo::links::get_link_by_url(db, &input.url).await {
        Ok(existing) => {
            // URL 已存在
            if existing.status == Some(LinkStatus::Broken) {
                // broken 状态，更新信息并置为 pending
                let update_input = RepoUpdateLink {
                    title: Some(input.title),
                    avatar: Some(input.avatar),
                    description: Some(input.description),
                    status: Some(Some(LinkStatus::Pending)),
                    ..Default::default()
                };
                let updated = repo::links::update_link(db, existing.id, update_input).await?;
                Ok(updated.into())
            } else {
                // 其他状态（pending/active），返回错误
                Err(ServiceError::InvalidInput(
                    "this url has already been submitted".to_string(),
                ))
            }
        }
        Err(repo::error::RepoError::NotFound) => {
            // URL 不存在，创建新友链
            let create_input = RepoCreateLink {
                title: input.title,
                url: input.url,
                avatar: input.avatar,
                description: input.description,
            };
            let link_dto = repo::links::create_link(db, create_input).await?;
            Ok(link_dto.into())
        }
        Err(e) => Err(e.into()),
    }
}

/// 2. 获取活跃友链列表（前台）
/// 
/// 仅返回 status=active 的友链，按创建时间倒序
pub async fn list_active_links(
    db: &DatabaseConnection,
) -> ServiceResult<Vec<Link>> {
    let models = Links::find()
        .filter(LinkColumn::Status.eq(LinkStatus::Active.as_str()))
        .order_by_desc(LinkColumn::CreatedAt)
        .all(db)
        .await?;

    let dtos: Result<Vec<_>, _> = models
        .into_iter()
        .map(|m| repo::links::LinkDto::try_from(m))
        .collect();
    Ok(dtos?.into_iter().map(Into::into).collect())
}

/// 3. 列出所有友链（后台）
/// 
/// 支持按 status 筛选 + 排序
pub async fn list_all_links(
    db: &DatabaseConnection,
    filter: LinkFilter,
) -> ServiceResult<Vec<Link>> {
    let mut query = Links::find();

    // 按状态筛选
    if let Some(status) = filter.status {
        query = query.filter(LinkColumn::Status.eq(status.as_str()));
    }

    // 排序
    match filter.sort_by.unwrap_or(LinkSortBy::CreatedAtDesc) {
        LinkSortBy::CreatedAtAsc => {
            query = query.order_by_asc(LinkColumn::CreatedAt);
        }
        LinkSortBy::CreatedAtDesc => {
            query = query.order_by_desc(LinkColumn::CreatedAt);
        }
    }

    let models = query.all(db).await?;
    let dtos: Result<Vec<_>, _> = models
        .into_iter()
        .map(|m| repo::links::LinkDto::try_from(m))
        .collect();
    Ok(dtos?.into_iter().map(Into::into).collect())
}

/// 4. 获取待审核友链列表（后台）
/// 
/// 快捷查询 status=pending
pub async fn list_pending_links(
    db: &DatabaseConnection,
) -> ServiceResult<Vec<Link>> {
    let filter = LinkFilter {
        status: Some(LinkStatus::Pending),
        sort_by: Some(LinkSortBy::CreatedAtDesc),
    };
    list_all_links(db, filter).await
}

/// 5. 获取友链详情
pub async fn get_link_by_id(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Link> {
    let dto = repo::links::get_link_by_id(db, id).await?;
    Ok(dto.into())
}

/// 6. 审核友链：通过
/// 
/// pending -> active
pub async fn approve_link(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Link> {
    let update_input = RepoUpdateLink {
        status: Some(Some(LinkStatus::Active)),
        ..Default::default()
    };
    let updated = repo::links::update_link(db, id, update_input).await?;
    Ok(updated.into())
}

/// 7. 标记友链失效
/// 
/// 任意状态 -> broken（用于标记站点失效、信息异常等）
pub async fn mark_link_broken(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<Link> {
    let update_input = RepoUpdateLink {
        status: Some(Some(LinkStatus::Broken)),
        ..Default::default()
    };
    let updated = repo::links::update_link(db, id, update_input).await?;
    Ok(updated.into())
}

/// 8. 更新友链信息（后台）
pub async fn update_link(
    db: &DatabaseConnection,
    id: i64,
    input: UpdateLinkInput,
) -> ServiceResult<Link> {
    // 如果更新 URL，验证格式
    if let Some(ref new_url) = input.url {
        if !is_valid_url(new_url) {
            return Err(ServiceError::InvalidInput(
                "url must start with http:// or https://".to_string(),
            ));
        }
    }

    let update_input = RepoUpdateLink {
        title: input.title,
        url: input.url,
        avatar: input.avatar,
        description: input.description,
        ..Default::default()
    };
    let updated = repo::links::update_link(db, id, update_input).await?;
    Ok(updated.into())
}

/// 9. 删除友链（后台）
pub async fn delete_link(
    db: &DatabaseConnection,
    id: i64,
) -> ServiceResult<()> {
    repo::links::delete_link(db, id).await?;
    Ok(())
}

// ================================
// 辅助函数
// ================================

/// 验证 URL 格式（简单验证）
fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://")
}
