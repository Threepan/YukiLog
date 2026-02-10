use crate::core::error::AppError;

pub(crate) fn validate_pagination(page: u64, size: u64) -> Result<(), AppError> {
    if page < 1 {
        return Err(AppError::BadRequest("page 必须从 1 开始".to_string()));
    }

    if size < 1 || size > 100 {
        return Err(AppError::BadRequest(
            "size 必须在 1-100 之间".to_string(),
        ));
    }

    Ok(())
}
