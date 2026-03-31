use thiserror::Error;

/// 加密错误类型
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("加密失败: {0}")]
    EncryptionFailed(String),

    #[error("解密失败: {0}")]
    DecryptionFailed(String),

    #[error("无效的密钥")]
    InvalidKey,

    #[error("密钥派生失败: {0}")]
    KeyDerivationFailed(String),

    #[error("密钥不存在: {0}")]
    KeyNotFound(String),

    #[error("密钥已存在: {0}")]
    KeyAlreadyExists(String),

    #[error("密钥无效: {0}")]
    KeyInvalid(String),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("Base64 编码错误: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("序列化错误: {0}")]
    Serde(#[from] toml::ser::Error),
}
