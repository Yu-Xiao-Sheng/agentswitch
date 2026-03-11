use thiserror::Error;

/// 补全错误类型
#[derive(Error, Debug)]
pub enum CompletionError {
    #[error("不支持的 Shell 类型: {0}")]
    UnsupportedShell(String),

    #[error("无法确定 Shell 类型")]
    CannotDetectShell,

    #[error("Shell 配置文件不存在: {0}")]
    ShellConfigNotFound(String),

    #[error("写入 Shell 配置文件失败: {0}")]
    CannotWriteShellConfig(String),

    #[error("补全脚本生成失败: {0}")]
    GenerationFailed(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
}
