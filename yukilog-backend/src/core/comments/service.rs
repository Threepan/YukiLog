use chrono::Utc;
use sea_orm::{ActiveValue, DatabaseConnection};
use std::collections::HashMap;

use crate::core::error::AppError;
use crate::core::validation::validate_pagination;
use crate::entities::comments;
use crate::infra::repository::{comments::CommentsRepository, users::UsersRepository};

use super::dto::{
    BatchReviewRequest, CommentAuthor, CommentListResponse, CommentNode, CommentResponse,
    CreateCommentRequest, ReviewCommentRequest, UpdateCommentRequest,
};

/// 评论服务
pub struct CommentsService {
    repo: CommentsRepository,
    users_repo: UsersRepository,
}

impl CommentsService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            repo: CommentsRepository::new(db.clone()),
            users_repo: UsersRepository::new(db),
        }
    }

    // ===== 前台公开方法 =====

    /// 获取文章评论树 (已审核)
    ///
    /// # 说明
    /// - 仅返回已审核通过的评论
    /// - 构建树形结构（无限层级嵌套）
    /// - 按创建时间正序
    pub async fn get_comment_tree(&self, post_id: i64) -> Result<Vec<CommentNode>, AppError> {
        let comments = self.repo.find_approved_by_post_id(post_id).await?;

        // 转换为 CommentResponse
        let mut responses = Vec::new();
        for comment in comments {
            responses.push(self.comment_to_response(comment, false).await?);
        }

        // 构建树结构
        Ok(self.build_comment_tree(responses))
    }

    /// 游客发表评论
    ///
    /// # 说明
    /// - guest_nickname, guest_email 必填
    /// - 默认 is_reviewed = false（需要审核）
    /// - 防刷屏检查：同IP在1分钟内最多发3条评论
    pub async fn create_comment_as_guest(
        &self,
        req: CreateCommentRequest,
        ip: String,
        ua: Option<String>,
    ) -> Result<CommentResponse, AppError> {
        // 验证游客信息
        let guest_nickname = req
            .guest_nickname
            .ok_or_else(|| AppError::BadRequest("游客昵称不能为空".to_string()))?;
        let guest_email = req
            .guest_email
            .ok_or_else(|| AppError::BadRequest("游客邮箱不能为空".to_string()))?;

        // 防刷屏检查：同IP在短时间内的评论数量
        let recent_comments = self.repo.find_by_ip(&ip).await?;
        let now = Utc::now();
        let one_minute_ago = now - chrono::Duration::minutes(1);

        let recent_count = recent_comments
            .iter()
            .filter(|c| {
                c.created_at
                    .as_ref()
                    .map(|dt| dt.to_utc() > one_minute_ago)
                    .unwrap_or(false)
            })
            .count();

        if recent_count >= 3 {
            return Err(AppError::Business("评论过于频繁，请稍后再试".to_string()));
        }

        // 如果是回复评论，检查父评论是否存在
        if let Some(parent_id) = req.parent_id {
            if !self.repo.exists(parent_id).await? {
                return Err(AppError::NotFound(format!("父评论ID {} 不存在", parent_id)));
            }
        }

        // 创建评论
        let new_comment = comments::ActiveModel {
            post_id: ActiveValue::Set(Some(req.post_id)),
            content: ActiveValue::Set(req.content),
            parent_id: ActiveValue::Set(req.parent_id),
            user_id: ActiveValue::Set(None),
            guest_nickname: ActiveValue::Set(Some(guest_nickname)),
            guest_email: ActiveValue::Set(Some(guest_email)),
            guest_website: ActiveValue::Set(req.guest_website),
            is_reviewed: ActiveValue::Set(Some(false)), // 游客评论需要审核
            ip: ActiveValue::Set(Some(ip)),
            ua: ActiveValue::Set(ua),
            ..Default::default()
        };

        let comment = self.repo.create(new_comment).await?;

        self.comment_to_response(comment, false).await
    }

    /// 登录用户发表评论
    ///
    /// # 说明
    /// - 自动通过审核（is_reviewed = true）
    /// - 不需要填写游客信息
    pub async fn create_comment_as_user(
        &self,
        user_id: i64,
        req: CreateCommentRequest,
        ip: String,
        ua: Option<String>,
    ) -> Result<CommentResponse, AppError> {
        // 检查用户是否存在
        if self.users_repo.find_by_id(user_id).await?.is_none() {
            return Err(AppError::NotFound(format!("用户ID {} 不存在", user_id)));
        }

        // 如果是回复评论，检查父评论是否存在
        if let Some(parent_id) = req.parent_id {
            if !self.repo.exists(parent_id).await? {
                return Err(AppError::NotFound(format!("父评论ID {} 不存在", parent_id)));
            }
        }

        // 创建评论
        let new_comment = comments::ActiveModel {
            post_id: ActiveValue::Set(Some(req.post_id)),
            content: ActiveValue::Set(req.content),
            parent_id: ActiveValue::Set(req.parent_id),
            user_id: ActiveValue::Set(Some(user_id)),
            guest_nickname: ActiveValue::Set(None),
            guest_email: ActiveValue::Set(None),
            guest_website: ActiveValue::Set(None),
            is_reviewed: ActiveValue::Set(Some(true)), // 登录用户评论自动通过
            ip: ActiveValue::Set(Some(ip)),
            ua: ActiveValue::Set(ua),
            ..Default::default()
        };

        let comment = self.repo.create(new_comment).await?;

        self.comment_to_response(comment, false).await
    }

    // ===== 管理后台方法 =====

    /// 获取所有评论 (分页)
    ///
    /// # 参数
    /// - `page`: 页码
    /// - `size`: 每页数量
    /// - `is_reviewed`: 可选的审核状态过滤
    pub async fn get_all_comments(
        &self,
        page: u64,
        size: u64,
        is_reviewed: Option<bool>,
    ) -> Result<CommentListResponse, AppError> {
        validate_pagination(page, size)?;
        let (comments, total) = self
            .repo
            .find_all_paginated(page, size, is_reviewed)
            .await?;

        let mut responses = Vec::new();
        for comment in comments {
            responses.push(self.comment_to_response(comment, true).await?);
        }

        Ok(CommentListResponse {
            comments: responses,
            total,
        })
    }

    /// 获取待审核评论列表
    pub async fn get_pending_comments(&self) -> Result<Vec<CommentResponse>, AppError> {
        let comments = self.repo.find_pending_review().await?;

        let mut responses = Vec::new();
        for comment in comments {
            responses.push(self.comment_to_response(comment, true).await?);
        }

        Ok(responses)
    }

    /// 获取单个评论
    pub async fn get_comment_by_id(&self, id: i64) -> Result<CommentResponse, AppError> {
        let comment = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("评论ID {} 不存在", id)))?;

        self.comment_to_response(comment, true).await
    }

    /// 更新评论 (仅Admin)
    pub async fn update_comment(
        &self,
        id: i64,
        req: UpdateCommentRequest,
    ) -> Result<CommentResponse, AppError> {
        let current_comment = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("评论ID {} 不存在", id)))?;

        let mut comment_model: comments::ActiveModel = current_comment.into();
        if let Some(content) = req.content {
            comment_model.content = ActiveValue::Set(content);
        }
        if let Some(is_reviewed) = req.is_reviewed {
            comment_model.is_reviewed = ActiveValue::Set(Some(is_reviewed));
        }

        let updated_comment = self.repo.update(comment_model).await?;

        self.comment_to_response(updated_comment, true).await
    }

    /// 审核单个评论
    pub async fn review_comment(
        &self,
        id: i64,
        req: ReviewCommentRequest,
    ) -> Result<CommentResponse, AppError> {
        let updated_comment = self.repo.review(id, req.is_approved).await?;

        self.comment_to_response(updated_comment, true).await
    }

    /// 批量审核评论
    ///
    /// # 返回
    /// - 受影响的行数
    pub async fn batch_review(&self, req: BatchReviewRequest) -> Result<u64, AppError> {
        let affected_rows = self.repo.batch_review(req.ids, req.is_approved).await?;
        Ok(affected_rows)
    }

    /// 删除评论 (智能删除)
    ///
    /// # 删除策略
    /// - 如果有子评论，将子评论的 parent_id 设为 NULL（变成顶层评论）
    /// - 然后删除当前评论
    /// - 如果没有子评论，直接删除
    pub async fn delete_comment(&self, id: i64) -> Result<(), AppError> {
        // 获取评论（避免 exists + unwrap 的竞态崩溃）
        let comment = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("评论ID {} 不存在", id)))?;

        let post_id = comment
            .post_id
            .ok_or_else(|| AppError::Internal("评论数据异常：缺少 post_id".to_string()))?;

        // 查找同文章下所有评论，筛选子评论
        let all_comments = self.repo.find_all_by_post_id(post_id).await?;

        let child_comments: Vec<_> = all_comments
            .into_iter()
            .filter(|c| c.parent_id == Some(id))
            .collect();

        // 如果有子评论，将它们的 parent_id 设为 NULL
        if !child_comments.is_empty() {
            for child in child_comments {
                let mut child_model: comments::ActiveModel = child.into();
                child_model.parent_id = ActiveValue::Set(None);
                self.repo.update(child_model).await?;
            }
        }

        // 删除当前评论
        let deleted = self.repo.delete(id).await?;
        if deleted.rows_affected == 0 {
            return Err(AppError::NotFound(format!("评论ID {} 不存在", id)));
        }

        Ok(())
    }

    /// 根据IP查询评论 (反垃圾)
    pub async fn get_comments_by_ip(&self, ip: &str) -> Result<Vec<CommentResponse>, AppError> {
        let comments = self.repo.find_by_ip(ip).await?;

        let mut responses = Vec::new();
        for comment in comments {
            responses.push(self.comment_to_response(comment, true).await?);
        }

        Ok(responses)
    }

    /// 获取用户的所有评论
    pub async fn get_user_comments(&self, user_id: i64) -> Result<Vec<CommentResponse>, AppError> {
        let comments = self.repo.find_by_user_id(user_id).await?;

        let mut responses = Vec::new();
        for comment in comments {
            responses.push(self.comment_to_response(comment, true).await?);
        }

        Ok(responses)
    }

    // ===== 辅助方法 =====

    /// 将 Model 转换为 Response
    ///
    /// # 参数
    /// - `include_ip`: 是否包含IP地址（管理员可见）
    async fn comment_to_response(
        &self,
        comment: comments::Model,
        include_ip: bool,
    ) -> Result<CommentResponse, AppError> {
        // 构建作者信息
        let author = if let Some(user_id) = comment.user_id {
            // 登录用户
            let user = self.users_repo.find_by_id(user_id).await?;
            if let Some(u) = user {
                CommentAuthor::User {
                    id: u.id,
                    username: u.username,
                    nickname: u.nickname,
                    avatar_url: u.avatar_url,
                }
            } else {
                // 用户已被删除，使用默认信息
                CommentAuthor::Guest {
                    nickname: "已删除用户".to_string(),
                    email: "deleted@example.com".to_string(),
                    website: None,
                }
            }
        } else {
            // 游客
            CommentAuthor::Guest {
                nickname: comment.guest_nickname.unwrap_or_else(|| "匿名".to_string()),
                email: comment
                    .guest_email
                    .unwrap_or_else(|| "anonymous@example.com".to_string()),
                website: comment.guest_website,
            }
        };

        Ok(CommentResponse {
            id: comment.id,
            post_id: comment.post_id.unwrap_or(0),
            content: comment.content,
            parent_id: comment.parent_id,
            author,
            is_reviewed: comment.is_reviewed.unwrap_or(false),
            ua: comment.ua,
            ip: if include_ip { comment.ip } else { None },
            created_at: comment
                .created_at
                .as_ref()
                .map(|dt| dt.to_utc())
                .unwrap_or_else(Utc::now),
        })
    }

    /// 构建评论树
    ///
    /// # 说明
    /// - 支持无限层级嵌套
    /// - 使用递归算法
    fn build_comment_tree(&self, comments: Vec<CommentResponse>) -> Vec<CommentNode> {
        // 按 parent_id 分组
        let mut children_map: HashMap<Option<i64>, Vec<CommentResponse>> = HashMap::new();
        for comment in comments {
            children_map
                .entry(comment.parent_id)
                .or_insert_with(Vec::new)
                .push(comment);
        }

        // 递归构建树
        fn build_nodes(
            parent_id: Option<i64>,
            children_map: &HashMap<Option<i64>, Vec<CommentResponse>>,
        ) -> Vec<CommentNode> {
            if let Some(children) = children_map.get(&parent_id) {
                children
                    .iter()
                    .map(|comment| {
                        let child_nodes = build_nodes(Some(comment.id), children_map);
                        CommentNode {
                            comment: comment.clone(),
                            children: child_nodes,
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            }
        }

        build_nodes(None, &children_map)
    }
}
