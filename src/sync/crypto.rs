use serde::{Deserialize, Serialize};

/// 加密后的值标记
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedValue {
    /// 加密方法
    pub method: String,

    /// 加密数据（Base64）
    pub data: String,

    /// Nonce（Base64，用于 AES-GCM）
    pub nonce: Option<String>,
}
