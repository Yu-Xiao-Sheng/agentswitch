//! 预设相关错误

use thiserror::Error;

/// 预设相关错误
#[derive(Debug, Error)]
pub enum PresetError {
    #[error("预设不存在: {0}")]
    PresetNotFound(String),

    #[error("预设名称已存在: {0}")]
    PresetAlreadyExists(String),

    #[error("模型配置不存在: {0}")]
    ModelConfigNotFound(String),

    #[error("工具未安装: {0}")]
    AgentNotInstalled(String),

    #[error("备份失败: {0}")]
    BackupFailed(String),

    #[error("应用失败: 工具={agent}, 原因={reason}")]
    ApplyFailed { agent: String, reason: String },

    #[error("回滚失败: {0}")]
    RollbackFailed(String),

    #[error("验证失败: {0}")]
    ValidationFailed(String),

    #[error("导入失败: {0}")]
    ImportFailed(String),

    #[error("导出失败: {0}")]
    ExportFailed(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("反序列化错误: {0}")]
    Deserialization(#[from] toml::de::Error),
}
