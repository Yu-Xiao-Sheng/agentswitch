use std::path::PathBuf;

/// AgentSwitch 错误类型
#[derive(thiserror::Error, Debug)]
pub enum AgentSwitchError {
    /// Agent 工具未找到
    #[error("未检测到 Agent 工具: {0}")]
    AgentNotFound(String),

    /// 配置文件只读
    #[error("配置文件只读，无法修改: {0}")]
    ConfigFileReadOnly(PathBuf),

    /// 备份文件损坏
    #[error("备份文件损坏: {0}")]
    BackupCorrupted(String),

    /// 磁盘空间不足
    #[error("磁盘空间不足，无法创建备份")]
    DiskSpaceInsufficient,

    /// 工具配置错误
    #[error("工具配置错误: {0}")]
    ToolConfigError(String),

    /// 未找到模型配置
    #[error("模型配置 '{0}' 不存在")]
    ModelConfigNotFound(String),

    /// 配置文件不存在
    #[error("配置文件不存在: {0}")]
    ConfigNotFound(PathBuf),

    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialization(String),

    /// 其他错误
    #[error("错误: {0}")]
    Other(String),
}

impl From<serde_json::Error> for AgentSwitchError {
    fn from(err: serde_json::Error) -> Self {
        AgentSwitchError::Serialization(err.to_string())
    }
}

impl From<toml::de::Error> for AgentSwitchError {
    fn from(err: toml::de::Error) -> Self {
        AgentSwitchError::Serialization(err.to_string())
    }
}

impl From<toml::ser::Error> for AgentSwitchError {
    fn from(err: toml::ser::Error) -> Self {
        AgentSwitchError::Serialization(err.to_string())
    }
}

/// AgentSwitch Result 类型
pub type Result<T> = std::result::Result<T, AgentSwitchError>;
