//! 认证服务
//!
//! 提供用户认证相关的业务逻辑

use crate::core::error::AppError;
use crate::infra::repository::UsersRepository;

use super::dto::{LoginRequest, LoginResponse, RefreshTokenResponse, UserInfo};
use super::jwt::{Claims, JwtUtils};
use super::password;

/// 认证服务
#[derive(Clone)]
pub struct AuthService {
    user_repo: UsersRepository,
    jwt_utils: JwtUtils,
}

impl AuthService {
    /// 创建认证服务实例
    pub fn new(user_repo: UsersRepository, jwt_utils: JwtUtils) -> Self {
        Self {
            user_repo,
            jwt_utils,
        }
    }

    /// 用户登录
    ///
    /// # 流程
    /// 1. 根据用户名查询用户
    /// 2. 验证密码
    /// 3. 生成 Access Token + Refresh Token
    ///
    /// # 安全说明
    /// 不区分"用户不存在"和"密码错误"，统一返回"用户名或密码错误"
    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        // 1. 查询用户
        let user = self
            .user_repo
            .find_by_username(&req.username)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::Unauthorized("用户名或密码错误".to_string()))?;

        // 2. 验证密码
        let is_valid = password::verify_password(&req.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::Unauthorized("用户名或密码错误".to_string()));
        }

        // 3. 生成 Token 对
        let role = user.role.as_deref().unwrap_or("user");
        let (access_token, refresh_token) =
            self.jwt_utils
                .generate_token_pair(user.id, &user.username, role)?;

        // 4. 返回登录响应
        let user_info = UserInfo::from_entity(&user);
        Ok(LoginResponse::new(
            access_token,
            refresh_token,
            self.jwt_utils.access_expires(),
            user_info,
        ))
    }

    /// 刷新 Token
    ///
    /// # Token 轮换
    /// 每次刷新都返回新的 Refresh Token，旧的应视为失效
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<RefreshTokenResponse, AppError> {
        // 1. 验证 Refresh Token
        let claims = self.jwt_utils.verify_refresh_token(refresh_token)?;

        // 2. 查询用户（确保用户仍存在）
        let user = self
            .user_repo
            .find_by_id(claims.user_id())
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::Unauthorized("用户不存在".to_string()))?;

        // 3. 生成新 Token 对
        let role = user.role.as_deref().unwrap_or("user");
        let (new_access, new_refresh) =
            self.jwt_utils
                .generate_token_pair(user.id, &user.username, role)?;

        // 4. 返回刷新响应
        let user_info = UserInfo::from_entity(&user);
        Ok(RefreshTokenResponse::new(
            new_access,
            new_refresh,
            self.jwt_utils.access_expires(),
            user_info,
        ))
    }

    /// 验证 Access Token
    pub fn verify_access_token(&self, token: &str) -> Result<Claims, AppError> {
        self.jwt_utils.verify_access_token(token)
    }

    /// 获取 JwtUtils 引用
    pub fn jwt_utils(&self) -> &JwtUtils {
        &self.jwt_utils
    }
}
