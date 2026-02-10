use sea_orm::DbErr;

use crate::domain::status::InvalidStatus;

pub type RepoResult<T> = Result<T, RepoError>;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("db error: {0}")]
    Db(#[from] DbErr),

    #[error("not found")]
    NotFound,

    #[error(transparent)]
    InvalidStatus(#[from] InvalidStatus),
}
