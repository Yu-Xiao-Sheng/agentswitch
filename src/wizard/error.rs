use thiserror::Error;

/// 向导错误类型
#[derive(Error, Debug)]
pub enum WizardError {
    #[error("向导状态文件损坏: {0}")]
    CorruptedState(String),

    #[error("用户取消操作")]
    Cancelled,

    #[error("验证失败: {0}")]
    ValidationFailed(String),

    #[error("非交互式环境")]
    NotInteractive,

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    Serde(#[from] toml::de::Error),

    #[error("序列化错误: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("输入错误: {0}")]
    Inquire(#[from] inquire::InquireError),

    #[error("配置错误: {0}")]
    Config(String),

    #[error("其他错误: {0}")]
    Other(#[from] anyhow::Error),
}
