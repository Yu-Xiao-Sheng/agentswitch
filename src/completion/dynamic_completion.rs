use serde::{Deserialize, Serialize};

/// 动态补全数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCompletionData {
    /// 已配置的模型列表
    pub models: Vec<String>,

    /// 已检测的工具列表
    pub tools: Vec<String>,

    /// 已保存的预设列表
    pub presets: Vec<String>,

    /// 数据生成时间
    pub generated_at: chrono::DateTime<chrono::Utc>,
}
