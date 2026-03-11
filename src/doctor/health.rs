/// 健康检查结果
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// 工具名称
    pub tool_name: String,

    /// 健康状态
    pub status: HealthStatus,

    /// 状态消息
    pub message: String,

    /// 修复建议
    pub suggestion: String,

    /// 错误详情（如果有）
    pub error_details: Option<String>,
}

/// 健康状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    /// 健康
    Healthy,

    /// 警告（非致命问题）
    Warning,

    /// 错误（需要修复）
    Error,
}
