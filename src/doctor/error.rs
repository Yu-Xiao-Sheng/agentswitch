use thiserror::Error;

/// 诊断错误类型
#[derive(Error, Debug)]
pub enum DoctorError {
    #[error("工具 {0} 检测失败: {1}")]
    DetectionFailed(String, String),

    #[error("配置文件读取失败: {0}")]
    ConfigReadError(String),

    #[error("权限不足: {0}")]
    PermissionDenied(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    Serde(#[from] toml::de::Error),

    #[error("命令执行错误: {0}")]
    CommandExecution(String),
}
