use thiserror::Error;

/// 同步错误类型
#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Git 未安装")]
    GitNotInstalled,

    #[error("不是 Git 仓库")]
    NotAGitRepository,

    #[error("远程仓库不可访问: {0}")]
    RemoteUnavailable(String),

    #[error("合并冲突")]
    MergeConflict,

    #[error("加密失败: {0}")]
    EncryptionError(String),

    #[error("解密失败: {0}")]
    DecryptionError(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git 错误: {0}")]
    Git(#[from] git2::Error),
}
