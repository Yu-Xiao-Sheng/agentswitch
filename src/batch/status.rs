//! 批量状态和数据结构

use serde::{Deserialize, Serialize};

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// 总操作数
    pub total: usize,
    /// 成功数
    pub succeeded: usize,
    /// 失败数
    pub failed: usize,
    /// 每个工具的详细结果
    pub results: Vec<ToolOperationResult>,
    /// 操作耗时（毫秒）
    pub duration_ms: u128,
}

/// 工具操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOperationResult {
    /// 工具名称
    pub agent_name: String,
    /// 是否成功
    pub success: bool,
    /// 错误信息（失败时）
    pub error_message: Option<String>,
    /// 备份文件路径（如有）
    pub backup_path: Option<String>,
}
