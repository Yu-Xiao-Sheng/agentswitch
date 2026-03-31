pub mod cipher;
pub mod error;
pub mod keyring;

pub use cipher::Aes256GcmCipher;
pub use error::CryptoError;

use base64::{engine::general_purpose, Engine as _};

// Re-export EncryptedValue from sync module

/// 加密管理器 trait
pub trait CryptoManager {
    /// 加密 API Key
    fn encrypt_api_key(&self, api_key: &str) -> Result<String, CryptoError>;

    /// 解密 API Key
    fn decrypt_api_key(&self, encrypted: &str) -> Result<String, CryptoError>;
}

/// AES-GCM 加密管理器
pub struct AesGcmCryptoManager {
    cipher: Aes256GcmCipher,
}

impl AesGcmCryptoManager {
    /// 从密码创建加密管理器
    pub fn from_password(password: &str, salt: &[u8; 32]) -> Result<Self, CryptoError> {
        let cipher = Aes256GcmCipher::from_password(password, salt)?;
        Ok(Self { cipher })
    }

    /// 从密钥创建加密管理器
    pub fn from_key(key: &[u8; 32]) -> Self {
        Self {
            cipher: Aes256GcmCipher::new(key),
        }
    }
}

impl CryptoManager for AesGcmCryptoManager {
    fn encrypt_api_key(&self, api_key: &str) -> Result<String, CryptoError> {
        let encrypted = self.cipher.encrypt(api_key.as_bytes())?;
        let base64 = general_purpose::STANDARD.encode(&encrypted);

        Ok(base64)
    }

    fn decrypt_api_key(&self, encrypted: &str) -> Result<String, CryptoError> {
        let decoded = general_purpose::STANDARD
            .decode(encrypted)
            .map_err(|e| CryptoError::DecryptionFailed(format!("Base64 解码失败: {}", e)))?;

        let decrypted = self.cipher.decrypt(&decoded)?;
        String::from_utf8(decrypted)
            .map_err(|e| CryptoError::DecryptionFailed(format!("UTF-8 解码失败: {}", e)))
    }
}
