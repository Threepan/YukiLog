use axum::Json;
use serde::Serialize;

/// 统一的 API 响应格式
/// 
/// 所有接口返回格式一致，便于前端处理
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    /// 请求是否成功
    pub success: bool,
    
    /// 响应数据（成功时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    
    /// 错误或提示信息（失败时存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    /// 创建带消息的成功响应
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.into()),
        }
    }
}

impl ApiResponse<()> {
    /// 创建错误响应
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message.into()),
        }
    }

    /// 创建纯成功响应（无数据）
    pub fn ok() -> Self {
        Self {
            success: true,
            data: None,
            message: None,
        }
    }
}

/// 分页数据响应
#[derive(Debug, Serialize)]
pub struct PagedData<T> {
    /// 当前页数据
    pub items: Vec<T>,
    
    /// 数据总数
    pub total: u64,
    
    /// 当前页码（从 1 开始）
    pub page: u64,
    
    /// 每页大小
    pub page_size: u64,
}

impl<T> PagedData<T> {
    /// 创建分页数据
    pub fn new(items: Vec<T>, total: u64, page: u64, page_size: u64) -> Self {
        Self {
            items,
            total,
            page,
            page_size,
        }
    }

    /// 计算总页数
    pub fn total_pages(&self) -> u64 {
        if self.page_size == 0 {
            return 0;
        }
        (self.total + self.page_size - 1) / self.page_size
    }
}

// ================================
// 便捷函数
// ================================

/// 返回成功响应（带数据）
pub fn ok<T: Serialize>(data: T) -> Json<ApiResponse<T>> {
    Json(ApiResponse::success(data))
}

/// 返回成功响应（带数据和消息）
pub fn ok_with_message<T: Serialize>(
    data: T,
    message: impl Into<String>,
) -> Json<ApiResponse<T>> {
    Json(ApiResponse::success_with_message(data, message))
}

/// 返回错误响应
pub fn error(message: impl Into<String>) -> Json<ApiResponse<()>> {
    Json(ApiResponse::error(message))
}

/// 返回纯成功响应（无数据）
pub fn no_content() -> Json<ApiResponse<()>> {
    Json(ApiResponse::ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let resp = ApiResponse::success("data");
        assert!(resp.success);
        assert_eq!(resp.data, Some("data"));
        assert_eq!(resp.message, None);
    }

    #[test]
    fn test_error_response() {
        let resp = ApiResponse::<()>::error("error message");
        assert!(!resp.success);
        assert_eq!(resp.data, None);
        assert_eq!(resp.message, Some("error message".to_string()));
    }

    #[test]
    fn test_paged_data() {
        let items = vec![1, 2, 3];
        let paged = PagedData::new(items, 10, 1, 3);
        
        assert_eq!(paged.items.len(), 3);
        assert_eq!(paged.total, 10);
        assert_eq!(paged.page, 1);
        assert_eq!(paged.page_size, 3);
        assert_eq!(paged.total_pages(), 4);
    }

    #[test]
    fn test_paged_data_total_pages() {
        // 测试边界情况
        assert_eq!(PagedData::new(vec![1], 10, 1, 3).total_pages(), 4);
        assert_eq!(PagedData::new(vec![1], 9, 1, 3).total_pages(), 3);
        assert_eq!(PagedData::new(vec![1], 0, 1, 3).total_pages(), 0);
        assert_eq!(PagedData::<i32>::new(vec![], 10, 1, 0).total_pages(), 0);
    }
}
